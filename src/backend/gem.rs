use crate::backend::Backend;
use crate::backend::backend_type::BackendType;
use crate::cli::args::BackendArg;
use crate::cmd::CmdLineRunner;
use crate::config::Settings;
use crate::file;
use crate::http::HTTP_FETCH;
use crate::install_context::InstallContext;
use crate::toolset::ToolVersion;
use crate::{Result, config::Config};
use async_trait::async_trait;
use futures::TryFutureExt;
use futures::stream;
use futures::stream::StreamExt;
use indoc::formatdoc;
use std::{fmt::Debug, sync::Arc};
use url::Url;

#[derive(Debug)]
pub struct GemBackend {
    ba: Arc<BackendArg>,
}

#[async_trait]
impl Backend for GemBackend {
    fn get_type(&self) -> BackendType {
        BackendType::Gem
    }

    fn ba(&self) -> &Arc<BackendArg> {
        &self.ba
    }

    fn get_dependencies(&self) -> eyre::Result<Vec<&str>> {
        Ok(vec!["ruby"])
    }

    async fn _list_remote_versions(&self, _config: &Arc<Config>) -> eyre::Result<Vec<String>> {
        let sources = Settings::get().gem.sources;
        let mut versions: Vec<String> = vec![];
        let gem_versions = stream::iter(sources)
            .filter_map(|gem_source| async move {
                let versions = fetch_gem_api_versions(&"", &self.tool_name())
                    .or_else(|_| fetch_compact_index_info_versions(&"", &self.tool_name()))
                    .await;
                match versions {
                    Ok(versions) => Some(versions),
                    Err(e) => None,
                }
            })
            .next();
        for version in gem_versions.iter().rev() {
            versions.push(version.number.clone());
        }
        Ok(versions)
    }

    async fn install_version_(&self, ctx: &InstallContext, tv: ToolVersion) -> Result<ToolVersion> {
        Settings::get().ensure_experimental("gem backend")?;

        CmdLineRunner::new("gem")
            .arg("install")
            .arg(self.tool_name())
            .arg("--version")
            .arg(&tv.version)
            .arg("--install-dir")
            .arg(tv.install_path().join("libexec"))
            // NOTE: Use `#!/usr/bin/env ruby` may cause some gems to not work properly
            //       using a different ruby then they were installed with. Therefore we
            //       we avoid the use of `--env-shebang` for now. However, this means that
            //       uninstalling the ruby version used to install the gem will break the
            //       gem. We should find a way to fix this.
            // .arg("--env-shebang")
            .with_pr(&ctx.pr)
            .envs(self.dependency_env(&ctx.config).await?)
            .execute()?;

        // We install the gem to {install_path}/libexec and create a wrapper script for each executable
        // in {install_path}/bin that sets GEM_HOME and executes the gem installed
        env_script_all_bin_files(&tv.install_path())?;

        Ok(tv)
    }
}

impl GemBackend {
    pub fn from_arg(ba: BackendArg) -> Self {
        Self { ba: Arc::new(ba) }
    }
}

async fn fetch_gem_api_versions(gem_source: &str, gem_name: &str) -> eyre::Result<Vec<GemVersion>> {
    // The `gem list` command does not supporting listing versions as json output
    // so we use the rubygems.org api to get the list of versions.
    let versions_url: Url = format!("{gem_source}/api/v1/versions/{gem_name}.json").parse()?;
    let raw = HTTP_FETCH.get_text(versions_url).await?;
    let gem_versions: Vec<GemVersion> = serde_json::from_str(&raw)?;
    Ok(gem_versions)
}

async fn fetch_compact_index_info_versions(
    gem_source: &str,
    gem_name: &str,
) -> eyre::Result<Vec<GemVersion>> {
    // Some gem servers do not support the JSON API, so we fall back to Bundler's compact index API.
    // https://guides.rubygems.org/rubygems-org-compact-index-api/#get---inforubygem
    let info_url: Url = format!("{gem_source}/info/{gem_name}").parse()?;
    let raw = HTTP_FETCH.get_text(info_url).await?;
    let mut gem_versions: Vec<GemVersion> = raw
        .lines()
        .skip_while(|line| line.trim() != "---")
        .skip(1) // Skip the "---" line
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                return None;
            }
            // Split on "-" to get the version number
            let version_parts: Vec<&str> = parts[0].split('-').collect();
            if version_parts.is_empty() {
                return None;
            }
            Some(GemVersion {
                number: version_parts[0].to_string(),
            })
        })
        .collect();
    gem_versions.sort_by(|a, b| a.number.cmp(&b.number));
    Ok(gem_versions)
}

fn env_script_all_bin_files(install_path: &std::path::Path) -> eyre::Result<bool> {
    let install_bin_path = install_path.join("bin");
    let install_libexec_path = install_path.join("libexec");

    match std::fs::create_dir_all(&install_bin_path) {
        Ok(_) => {}
        Err(e) => {
            return Err(eyre::eyre!("couldn't create directory: {}", e));
        }
    }

    get_gem_executables(install_path)?
        .into_iter()
        .for_each(|path| {
            let exec_path = install_bin_path.join(path.file_name().unwrap());
            file::write(
                &exec_path,
                formatdoc!(
                    r#"
                    #!/usr/bin/env bash
                    GEM_HOME="{gem_home}" exec {gem_exec_path} "$@"
                    "#,
                    gem_home = install_libexec_path.to_str().unwrap(),
                    gem_exec_path = path.to_str().unwrap(),
                ),
            )
            .unwrap();
            file::make_executable(&exec_path).unwrap();
        });

    Ok(true)
}

fn get_gem_executables(install_path: &std::path::Path) -> eyre::Result<Vec<std::path::PathBuf>> {
    // TODO: Find a way to get the list of executables from the gemspec of the
    //       installed gem rather than just listing the files in the bin directory.
    let install_libexec_bin_path = install_path.join("libexec/bin");
    let mut files = vec![];

    for entry in std::fs::read_dir(install_libexec_bin_path)? {
        let entry = entry?;
        let path = entry.path();
        if file::is_executable(&path) {
            files.push(path);
        }
    }

    Ok(files)
}

#[derive(Debug, serde::Deserialize)]
struct GemVersion {
    number: String,
}
