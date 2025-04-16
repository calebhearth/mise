# Changelog

## [2025.4.5](https://github.com/jdx/mise/compare/v2025.4.4..v2025.4.5) - 2025-04-16

### 🐛 Bug Fixes

- **(ubi)** API URL for GitHub should not have /repos segement by [@roele](https://github.com/roele) in [#4848](https://github.com/jdx/mise/pull/4848)
- **(ubi)** URL syntax fails by [@roele](https://github.com/roele) in [#4859](https://github.com/jdx/mise/pull/4859)
- allow to install non-numeric elixir versions by [@roele](https://github.com/roele) in [#4850](https://github.com/jdx/mise/pull/4850)
- removed possible single-point-of-failure while running `mise upgrade` by [@hitblast](https://github.com/hitblast) in [#4847](https://github.com/jdx/mise/pull/4847)
- `#MISE tools=` in task header by [@jdx](https://github.com/jdx) in [#4860](https://github.com/jdx/mise/pull/4860)

### New Contributors

- @hitblast made their first contribution in [#4847](https://github.com/jdx/mise/pull/4847)

## [2025.4.4](https://github.com/jdx/mise/compare/v2025.4.3..v2025.4.4) - 2025-04-15

### 🧪 Testing

- remove kpt test by [@jdx](https://github.com/jdx) in [b9d35ac](https://github.com/jdx/mise/commit/b9d35ac57936291a0a4629f9c200dfdb500a7efb)

## [2025.4.3](https://github.com/jdx/mise/compare/v2025.4.2..v2025.4.3) - 2025-04-15

### 🚀 Features

- **(aqua)** support SLSA source_uri setting by [@scop](https://github.com/scop) in [#4833](https://github.com/jdx/mise/pull/4833)
- **(aqua)** use source tag in SLSA verification by [@scop](https://github.com/scop) in [#4836](https://github.com/jdx/mise/pull/4836)
- **(ubi)** add support for self-hosted GitHub/GitLab by [@roele](https://github.com/roele) in [#4765](https://github.com/jdx/mise/pull/4765)

### 📚 Documentation

- Update configuration.md by [@jdx](https://github.com/jdx) in [#4829](https://github.com/jdx/mise/pull/4829)
- correct `mise use` paths by [@jdx](https://github.com/jdx) in [c8374c0](https://github.com/jdx/mise/commit/c8374c00ca68e5722c28f9abfd2425b9722bdd83)

## [2025.4.2](https://github.com/jdx/mise/compare/v2025.4.1..v2025.4.2) - 2025-04-11

### 🚀 Features

- **(registry)** update aws-nuke backend by [@StingRayZA](https://github.com/StingRayZA) in [#4815](https://github.com/jdx/mise/pull/4815)

### 🐛 Bug Fixes

- do not default to writing to mise.$MISE_ENV.toml by [@jdx](https://github.com/jdx) in [#4817](https://github.com/jdx/mise/pull/4817)
- mise watch forward --exts and --filter to watchexec by [@cmhms](https://github.com/cmhms) in [#4826](https://github.com/jdx/mise/pull/4826)

### 📚 Documentation

- Fixing typo in code for flags in toml-tasks.md by [@arafays](https://github.com/arafays) in [#4820](https://github.com/jdx/mise/pull/4820)
- branding by [@jdx](https://github.com/jdx) in [9ad2c17](https://github.com/jdx/mise/commit/9ad2c17ec75b7460ebea09a9f0601a561349cc7f)
- remove references to not-working docker: tasks by [@jdx](https://github.com/jdx) in [2c2fd27](https://github.com/jdx/mise/commit/2c2fd272e3d76329a7c67e4070bfb122ae1e1120)
- document some dependencies by [@jdx](https://github.com/jdx) in [6e8bd51](https://github.com/jdx/mise/commit/6e8bd518757c5e49624fc2bef5777a2f2339c304)
- simplify mise.toml example by [@jdx](https://github.com/jdx) in [66d927b](https://github.com/jdx/mise/commit/66d927ba4db81ba70de261cd76e399e9f4fe35da)

### 📦️ Dependency Updates

- update dependency vitepress-plugin-tabs to ^0.7.0 by [@renovate[bot]](https://github.com/renovate[bot]) in [#4822](https://github.com/jdx/mise/pull/4822)
- update rust crate petgraph to 0.8 by [@renovate[bot]](https://github.com/renovate[bot]) in [#4823](https://github.com/jdx/mise/pull/4823)
- update rust crate strum to 0.27 by [@renovate[bot]](https://github.com/renovate[bot]) in [#4780](https://github.com/jdx/mise/pull/4780)

### New Contributors

- @cmhms made their first contribution in [#4826](https://github.com/jdx/mise/pull/4826)
- @StingRayZA made their first contribution in [#4815](https://github.com/jdx/mise/pull/4815)

## [2025.4.1](https://github.com/jdx/mise/compare/v2025.4.0..v2025.4.1) - 2025-04-09

### 🚀 Features

- **(registry)** added localstack by [@mnm364](https://github.com/mnm364) in [#4785](https://github.com/jdx/mise/pull/4785)
- **(registry)** added skeema by [@mnm364](https://github.com/mnm364) in [#4786](https://github.com/jdx/mise/pull/4786)
- **(registry)** add television by [@mangkoran](https://github.com/mangkoran) in [#4778](https://github.com/jdx/mise/pull/4778)

### 🐛 Bug Fixes

- show gh rate limit reset time in local time by [@someoneinjd](https://github.com/someoneinjd) in [#4799](https://github.com/jdx/mise/pull/4799)

### 📚 Documentation

- all experimental note for lockfile by [@zeitlinger](https://github.com/zeitlinger) in [#4781](https://github.com/jdx/mise/pull/4781)
- Include post about Mise secrets in the context of Swift app dev by [@pepicrft](https://github.com/pepicrft) in [#4809](https://github.com/jdx/mise/pull/4809)

### Chore

- update deps to fix deny check by [@jdx](https://github.com/jdx) in [432023b](https://github.com/jdx/mise/commit/432023b2cd04d2ea7f590d7b338054944512abd0)
- pin zip to avoid issue with ubi by [@jdx](https://github.com/jdx) in [315deb4](https://github.com/jdx/mise/commit/315deb4e24177408c598d22951adb95f3e841683)

### New Contributors

- @someoneinjd made their first contribution in [#4799](https://github.com/jdx/mise/pull/4799)
- @mnm364 made their first contribution in [#4786](https://github.com/jdx/mise/pull/4786)
- @zeitlinger made their first contribution in [#4781](https://github.com/jdx/mise/pull/4781)

## [2025.4.0](https://github.com/jdx/mise/compare/v2025.3.11..v2025.4.0) - 2025-04-02

### 🐛 Bug Fixes

- s/runtimes/tools by [@jdx](https://github.com/jdx) in [#4754](https://github.com/jdx/mise/pull/4754)
- add clarification on RUSTUP_HOME and CARGO_HOME by [@lachieh](https://github.com/lachieh) in [#4759](https://github.com/jdx/mise/pull/4759)
- enhance confirmation logic to respect SETTINGS.yes by [@roele](https://github.com/roele) in [#4764](https://github.com/jdx/mise/pull/4764)

### 🚜 Refactor

- **(registry)** use aqua for ubi by [@scop](https://github.com/scop) in [#4745](https://github.com/jdx/mise/pull/4745)
- **(registry)** use aqua for ksops by [@scop](https://github.com/scop) in [#4746](https://github.com/jdx/mise/pull/4746)

### 📚 Documentation

- mark code block for dnf5 install as shell code by [@sina-hide](https://github.com/sina-hide) in [#4747](https://github.com/jdx/mise/pull/4747)
- update demo by [@hverlin](https://github.com/hverlin) in [c67be43](https://github.com/jdx/mise/commit/c67be43fcede947920c8eb17afb059037235a3e0)
- move demo to top-level by [@jdx](https://github.com/jdx) in [2b6f45a](https://github.com/jdx/mise/commit/2b6f45ac73d6f59542f9c7b401042ad5c75e37e2)
- Update config.ts by [@jdx](https://github.com/jdx) in [05ad4bc](https://github.com/jdx/mise/commit/05ad4bc9b2243737c0551fd36de1e37dc57ea578)
- Update walkthrough.md by [@jdx](https://github.com/jdx) in [89904b4](https://github.com/jdx/mise/commit/89904b46d8649a66bf960b1e5c7c0364dad8f94f)
- Update index.md by [@jdx](https://github.com/jdx) in [#4750](https://github.com/jdx/mise/pull/4750)
- Update walkthrough.md by [@jdx](https://github.com/jdx) in [#4751](https://github.com/jdx/mise/pull/4751)
- Update README.md by [@jdx](https://github.com/jdx) in [4f38142](https://github.com/jdx/mise/commit/4f38142bd3d822c3eafd78a74aa7a8d31791d2e3)

### New Contributors

- @lachieh made their first contribution in [#4759](https://github.com/jdx/mise/pull/4759)
- @sina-hide made their first contribution in [#4747](https://github.com/jdx/mise/pull/4747)

## [2025.3.11](https://github.com/jdx/mise/compare/v2025.3.10..v2025.3.11) - 2025-03-28

### 🚀 Features

- **(registry)** add protoc-gen-validate by [@akanter](https://github.com/akanter) in [#4703](https://github.com/jdx/mise/pull/4703)

### 🚜 Refactor

- **(registry)** use aqua for swiftlint by [@scop](https://github.com/scop) in [#4726](https://github.com/jdx/mise/pull/4726)
- **(registry)** use ubi for opensearch-cli by [@scop](https://github.com/scop) in [#4725](https://github.com/jdx/mise/pull/4725)
- **(registry)** use ubi for mdbook-linkcheck by [@scop](https://github.com/scop) in [#4724](https://github.com/jdx/mise/pull/4724)
- **(registry)** use ubi for velad by [@scop](https://github.com/scop) in [#4727](https://github.com/jdx/mise/pull/4727)

## [2025.3.10](https://github.com/jdx/mise/compare/v2025.3.9..v2025.3.10) - 2025-03-26

## [2025.3.9](https://github.com/jdx/mise/compare/v2025.3.8..v2025.3.9) - 2025-03-26

### 🚀 Features

- Set usage arguments and flag as environment variables before running the command by [@gturi](https://github.com/gturi) in [#4700](https://github.com/jdx/mise/pull/4700)

### 🚜 Refactor

- **(registry)** use ubi for assh by [@scop](https://github.com/scop) in [#4713](https://github.com/jdx/mise/pull/4713)
- **(registry)** use ubi for opsgenie-lamp by [@scop](https://github.com/scop) in [#4712](https://github.com/jdx/mise/pull/4712)
- **(registry)** use ubi for auto-doc by [@scop](https://github.com/scop) in [#4714](https://github.com/jdx/mise/pull/4714)
- **(registry)** use ubi for getenvoy by [@scop](https://github.com/scop) in [#4715](https://github.com/jdx/mise/pull/4715)
- **(registry)** use ubi for mockolo by [@scop](https://github.com/scop) in [#4705](https://github.com/jdx/mise/pull/4705)
- **(registry)** use ubi for haxe by [@scop](https://github.com/scop) in [#4716](https://github.com/jdx/mise/pull/4716)
- **(registry)** use ubi for helm-diff by [@scop](https://github.com/scop) in [#4717](https://github.com/jdx/mise/pull/4717)
- **(registry)** use ubi for grain by [@scop](https://github.com/scop) in [#4718](https://github.com/jdx/mise/pull/4718)

## [2025.3.8](https://github.com/jdx/mise/compare/v2025.3.7..v2025.3.8) - 2025-03-24

### 🚀 Features

- **(registry)** add aichat by [@kit494way](https://github.com/kit494way) in [#4691](https://github.com/jdx/mise/pull/4691)

### 🐛 Bug Fixes

- Update flake to fix nix build by [@akanter](https://github.com/akanter) in [#4686](https://github.com/jdx/mise/pull/4686)

### 📚 Documentation

- fix bash completion setup instructions by [@bestagi](https://github.com/bestagi) in [6df6f92](https://github.com/jdx/mise/commit/6df6f927178de174b043f3f4a8d26fbc0452b3f8)
- small tidy of shims docs by [@AlecRust](https://github.com/AlecRust) in [#4693](https://github.com/jdx/mise/pull/4693)

### Chore

- remove broken ripsecrets test by [@jdx](https://github.com/jdx) in [bb382aa](https://github.com/jdx/mise/commit/bb382aa783a2a1bfc44f02a5bb34f9397efb2e57)
- make awscli compatible with R2 by [@jdx](https://github.com/jdx) in [cad7fa2](https://github.com/jdx/mise/commit/cad7fa285e96483ba8d6aeb22f83de10e92700b2)
- enable workflow_dispatch for docs task by [@jdx](https://github.com/jdx) in [b0578db](https://github.com/jdx/mise/commit/b0578db141decc63992ebb0f74e29a53238611ba)

### New Contributors

- @akanter made their first contribution in [#4686](https://github.com/jdx/mise/pull/4686)
- @bestagi made their first contribution

## [2025.3.7](https://github.com/jdx/mise/compare/v2025.3.6..v2025.3.7) - 2025-03-21

### 🐛 Bug Fixes

- **(node)** skip gpg verification of sig file not found by [@jdx](https://github.com/jdx) in [#4663](https://github.com/jdx/mise/pull/4663)
- **(task)** allow args to be used with tera tests by [@risu729](https://github.com/risu729) in [6d07ed1](https://github.com/jdx/mise/commit/6d07ed13d462d95e874840389d99c64c955b7f76)
- Fix syntax error on `activate nu` when PATH contains shims by [@atty303](https://github.com/atty303) in [290b9fc](https://github.com/jdx/mise/commit/290b9fc62f862c24a7c4f930138a58983cf229eb)

### 🚜 Refactor

- **(registry)** use ubi for yamlscript by [@scop](https://github.com/scop) in [#4670](https://github.com/jdx/mise/pull/4670)

### 📚 Documentation

- Fix typo in java.md by [@hverlin](https://github.com/hverlin) in [#4672](https://github.com/jdx/mise/pull/4672)

### ◀️ Revert

- "chore: temporarily disable bootstrap test" by [@jdx](https://github.com/jdx) in [#4658](https://github.com/jdx/mise/pull/4658)

### 📦️ Dependency Updates

- update rust crate ctor to 0.4 by [@renovate[bot]](https://github.com/renovate[bot]) in [cf01e04](https://github.com/jdx/mise/commit/cf01e04baa83551fdcc39c50e23bbf62ced8a78a)

### Chore

- **(registry)** declare copier by [@looztra](https://github.com/looztra) in [#4669](https://github.com/jdx/mise/pull/4669)
- Update to the latest version of ubi by [@autarch](https://github.com/autarch) in [#4648](https://github.com/jdx/mise/pull/4648)
- bump expr by [@jdx](https://github.com/jdx) in [#4666](https://github.com/jdx/mise/pull/4666)
- added android-sdk by [@jdx](https://github.com/jdx) in [#4668](https://github.com/jdx/mise/pull/4668)
- rename mise-php to asdf-php by [@jdx](https://github.com/jdx) in [#4674](https://github.com/jdx/mise/pull/4674)

### New Contributors

- @atty303 made their first contribution
- @looztra made their first contribution in [#4669](https://github.com/jdx/mise/pull/4669)

## [2025.3.6](https://github.com/jdx/mise/compare/v2025.3.5..v2025.3.6) - 2025-03-18

### Chore

- unpin aws-cli by [@jdx](https://github.com/jdx) in [7fabed5](https://github.com/jdx/mise/commit/7fabed5c70fccfe095647c7b2220965ca2f1c07d)
- temporarily disable bootstrap test by [@jdx](https://github.com/jdx) in [599258a](https://github.com/jdx/mise/commit/599258aa4f5c0ab0b5581740b0c9eec17f1c7318)

## [2025.3.5](https://github.com/jdx/mise/compare/v2025.3.4..v2025.3.5) - 2025-03-18

### 🚀 Features

- **(registry)** use ubi for glab by [@scop](https://github.com/scop) in [#4643](https://github.com/jdx/mise/pull/4643)
- ubi forge option support by [@scop](https://github.com/scop) in [#4642](https://github.com/jdx/mise/pull/4642)

### 🐛 Bug Fixes

- **(tera)** use default inline shell to parse exec template by [@risu729](https://github.com/risu729) in [#4645](https://github.com/jdx/mise/pull/4645)

## [2025.3.4](https://github.com/jdx/mise/compare/v2025.3.3..v2025.3.4) - 2025-03-18

### 🐛 Bug Fixes

- Failed to create venv at the same time by multiple uv processes by [@NavyD](https://github.com/NavyD) in [#4640](https://github.com/jdx/mise/pull/4640)

## [2025.3.3](https://github.com/jdx/mise/compare/v2025.3.2..v2025.3.3) - 2025-03-14

### 🚀 Features

- **(env)** support env files in toml by [@risu729](https://github.com/risu729) in [#4618](https://github.com/jdx/mise/pull/4618)
- **(registry)** add harper-ls and harper-cli by [@kit494way](https://github.com/kit494way) in [788b1e2](https://github.com/jdx/mise/commit/788b1e25aba0a61aaca6df5c0d2d837c38a77fdf)
- **(registry)** add curlie by [@reitzig](https://github.com/reitzig) in [323b8bd](https://github.com/jdx/mise/commit/323b8bd5bbdab9370f3f00ae0dda375bbb50b65f)
- cleanup the mutex use. by [@boris-smidt-klarrio](https://github.com/boris-smidt-klarrio) in [74738db](https://github.com/jdx/mise/commit/74738dbbccb4425743b27561ee81f6d16c877cc2)
- Add flag to fmt command to read from stdin by [@erickgnavar](https://github.com/erickgnavar) in [56656b5](https://github.com/jdx/mise/commit/56656b583fb7e66c0078f0dbe43613bfe37687da)

### 🐛 Bug Fixes

- **(uv)** avoid deadlocks while initializing UV_VENV by [@risu729](https://github.com/risu729) in [26d203a](https://github.com/jdx/mise/commit/26d203a5aacc7d8b75b3f05ce0c54da5edb62d86)
- handle error when getting modified duration in file::modified_duration by [@roele](https://github.com/roele) in [#4624](https://github.com/jdx/mise/pull/4624)
- SwiftPM backend not working with the Swift 6 toolchain by [@pepicrft](https://github.com/pepicrft) in [#4632](https://github.com/jdx/mise/pull/4632)
- quiet in file task not working by [@roele](https://github.com/roele) in [ce94c5f](https://github.com/jdx/mise/commit/ce94c5f34feda8f121977a2cbafc321a4010814c)
- Unable to find uv when first creating py venv by [@NavyD](https://github.com/NavyD) in [b32bd67](https://github.com/jdx/mise/commit/b32bd67d660e318fa58c5bcc4fcf90101bb7c7dc)

### 🚜 Refactor

- migrate humantime to jiff by [@risu729](https://github.com/risu729) in [157c79b](https://github.com/jdx/mise/commit/157c79bbb1b15016047611b96fe6dc90b88671b0)
- use method to get the default inline shell instead of accessing the fields by [@risu729](https://github.com/risu729) in [#4621](https://github.com/jdx/mise/pull/4621)

### 📚 Documentation

- **(settings)** clarify the usage of disable_default_registry by [@gbloquel](https://github.com/gbloquel) in [67f5ea8](https://github.com/jdx/mise/commit/67f5ea8317bcf26755ef6ff65ed460fa272db9ff)

### ⚡ Performance

- speed up self-update by calling /releases/latest api instead of /releases by [@vemoo](https://github.com/vemoo) in [#4619](https://github.com/jdx/mise/pull/4619)

### 🧪 Testing

- **(registry)** fix test of lazyjournal by [@risu729](https://github.com/risu729) in [9f84c3e](https://github.com/jdx/mise/commit/9f84c3e510bb0b0a37cc74d84a189caa4e7e0078)

### Chore

- deny fixes by [@jdx](https://github.com/jdx) in [17d7c6e](https://github.com/jdx/mise/commit/17d7c6ee5e035272a8dc1b93c8fc7ac9cffb7f80)
- ignore humantime unmaintained advisory by [@risu729](https://github.com/risu729) in [15a1332](https://github.com/jdx/mise/commit/15a1332ec88d338c69a6c0489d54053d011be1e7)
- remove rustup update in github actions by [@risu729](https://github.com/risu729) in [#4617](https://github.com/jdx/mise/pull/4617)

### New Contributors

- @erickgnavar made their first contribution
- @vemoo made their first contribution in [#4619](https://github.com/jdx/mise/pull/4619)
- @gbloquel made their first contribution

## [2025.3.1](https://github.com/jdx/mise/compare/v2025.3.0..v2025.3.1) - 2025-03-06

### 🚀 Features

- **(registry)** added sampler by [@tony-sol](https://github.com/tony-sol) in [f39ec04](https://github.com/jdx/mise/commit/f39ec046d498c9d840a414504e47739b34877072)
- **(registry)** added lazyjournal by [@tony-sol](https://github.com/tony-sol) in [8cdea39](https://github.com/jdx/mise/commit/8cdea397dc2fc6008310d9c9f990034edfdee485)
- add support for components property in rust-toolchain.toml by [@roele](https://github.com/roele) in [27d18b5](https://github.com/jdx/mise/commit/27d18b5406fe92b8fd2e1f4a4bd948ad08cb6e9a)
- add --local flag for ls by [@tony-sol](https://github.com/tony-sol) in [14b3222](https://github.com/jdx/mise/commit/14b3222ec124a59502ff7107cd6aa8ff7e14b083)

### 🐛 Bug Fixes

- favor aqua backend over asdf by [@dud225](https://github.com/dud225) in [e385702](https://github.com/jdx/mise/commit/e385702e59f21c577d779c9e5b2ccc058c65dde1)

### 📚 Documentation

- continuous-integration.md: fix gitlab caching example by [@nafg](https://github.com/nafg) in [a8440d0](https://github.com/jdx/mise/commit/a8440d0d0f58b1c63b10b09df33f255b5c6074c1)

### Chore

- edition 2024 by [@jdx](https://github.com/jdx) in [8186b3d](https://github.com/jdx/mise/commit/8186b3d625bdc3de838251ca9d8d6e26395612e4)

### New Contributors

- @nafg made their first contribution
- @dud225 made their first contribution

## [2025.3.0](https://github.com/jdx/mise/compare/v2025.2.9..v2025.3.0) - 2025-03-01

### 🚀 Features

- **(registry)** added helmwave by [@tony-sol](https://github.com/tony-sol) in [bf251bd](https://github.com/jdx/mise/commit/bf251bd82b72c3bc0056a6ee97dfa609fb040863)
- **(registry)** added doggo by [@tony-sol](https://github.com/tony-sol) in [cefa092](https://github.com/jdx/mise/commit/cefa092ee3efae09eadc864c3d081a914e2b8c6e)
- **(registry)** Add Boilerplate by [@ZachGoldberg](https://github.com/ZachGoldberg) in [ca1b8de](https://github.com/jdx/mise/commit/ca1b8de3b54495306d06d86ac4ced56913979090)
- **(registry)** added htmlq by [@tony-sol](https://github.com/tony-sol) in [e76cd16](https://github.com/jdx/mise/commit/e76cd16f3fd907ff60ab42b56b693284fe098273)
- **(registry)** added gokey by [@tony-sol](https://github.com/tony-sol) in [8e876fa](https://github.com/jdx/mise/commit/8e876fa8c4b9261e1cc2c20e4bc090b8dc8cbc67)
- **(registry)** added octosql by [@tony-sol](https://github.com/tony-sol) in [059232c](https://github.com/jdx/mise/commit/059232cdf6d42595c95f397f4a5c903b3130a578)
- **(registry)** added hexyl by [@tony-sol](https://github.com/tony-sol) in [7aeac74](https://github.com/jdx/mise/commit/7aeac74e711d206ab3ecac705f6659eba17a5f84)
- **(registry)** added kubeone by [@tony-sol](https://github.com/tony-sol) in [76609e4](https://github.com/jdx/mise/commit/76609e41273029a01fe5374f763c22e2d9acb906)
- task confirmation by [@roele](https://github.com/roele) in [097660b](https://github.com/jdx/mise/commit/097660beacba00c7d3b1aa1a9dd7bf70ffb482cb)

### 🐛 Bug Fixes

- remote tasks and devcontainer by [@acesyde](https://github.com/acesyde) in [6ba5d3b](https://github.com/jdx/mise/commit/6ba5d3be1fe5c242b95dbbf3dbc224e1f30224e7)

### 📚 Documentation

- **(shim)** add faq for vscode windows spawn EINVAL & format value to list by [@qianlongzt](https://github.com/qianlongzt) in [3a2fec5](https://github.com/jdx/mise/commit/3a2fec50e93ffb34ae99487b9ab36c92f16339d4)

### New Contributors

- @ZachGoldberg made their first contribution

## [2025.2.9](https://github.com/jdx/mise/compare/v2025.2.8..v2025.2.9) - 2025-02-26

### 🚀 Features

- **(registry)** add cocogitto by [@reitzig](https://github.com/reitzig) in [32ac1c9](https://github.com/jdx/mise/commit/32ac1c93e6422f4f2ce8f6a3ed80577d0f644934)
- **(registry)** Added foundry by [@suicide](https://github.com/suicide) in [abf2401](https://github.com/jdx/mise/commit/abf24019dc2b60a305998575d3d80dfd655b186a)
- **(registry)** added ast-grep by [@tony-sol](https://github.com/tony-sol) in [2924ded](https://github.com/jdx/mise/commit/2924ded0fb4b4ea202c52d03c783c4b82c9b05d0)

### 🐛 Bug Fixes

- non-utf8 external process handling by [@jdx](https://github.com/jdx) in [fc4f3a3](https://github.com/jdx/mise/commit/fc4f3a3603d68ebdf83626ffc7e508e9c2d7f075)

### 📚 Documentation

- **(cookbook)** add shell powerline-go config env recipe by [@scop](https://github.com/scop) in [701bc8b](https://github.com/jdx/mise/commit/701bc8b916d45e0c42459bd4b3e3e07747916a16)
- update mise.el repo link by [@tecoholic](https://github.com/tecoholic) in [e8d88a1](https://github.com/jdx/mise/commit/e8d88a1ab22a91497f9a7ccfbcf09d6c40c3f6e7)

### Chore

- bump rust version for releases by [@jdx](https://github.com/jdx) in [f4e5970](https://github.com/jdx/mise/commit/f4e5970f00bf56d9be16a7e7e83289085c0e5cce)
- bump rust version for releases by [@jdx](https://github.com/jdx) in [52cff1c](https://github.com/jdx/mise/commit/52cff1c00b452b93b3ca1e4fc01fd21de73569e5)
- bump rust version for releases by [@jdx](https://github.com/jdx) in [9121c5e](https://github.com/jdx/mise/commit/9121c5e9270fae59ce753226ecbbe2939c4661e4)
- bump msrv for edition compatibility by [@jdx](https://github.com/jdx) in [3a222dd](https://github.com/jdx/mise/commit/3a222ddf272eef655b50796f34634fcedc3f1288)
- remove unused deny rule by [@jdx](https://github.com/jdx) in [053f5c1](https://github.com/jdx/mise/commit/053f5c1c0746e363c24b19577b958621ea91c40c)

### New Contributors

- @tony-sol made their first contribution
- @tecoholic made their first contribution
- @suicide made their first contribution
- @reitzig made their first contribution

## [2025.2.8](https://github.com/jdx/mise/compare/v2025.2.7..v2025.2.8) - 2025-02-25

### 🚀 Features

- **(registry)** add checkmake to registry by [@eread](https://github.com/eread) in [c8adbaa](https://github.com/jdx/mise/commit/c8adbaa54da0995f74a609d7a060ae32335a2feb)
- **(registry)** added sops from aqua registry by [@ldrouard](https://github.com/ldrouard) in [130f598](https://github.com/jdx/mise/commit/130f598fc81133c2654750f4d4433a571230e903)
- **(registry)** added k9s from aqua registry by [@ldrouard](https://github.com/ldrouard) in [5bfc60a](https://github.com/jdx/mise/commit/5bfc60ab270b2fd24f3809213864e9d30ff4faf3)
- **(registry)** added hadolint from aqua registry by [@ldrouard](https://github.com/ldrouard) in [0de4089](https://github.com/jdx/mise/commit/0de40898b934f9415b5524c3a9ced54ac07a31a7)
- **(shim)** Windows shim add hardlink & symlink mode by [@qianlongzt](https://github.com/qianlongzt) in [e9bd775](https://github.com/jdx/mise/commit/e9bd775b9f1c71f49e6b553d02c96e1fb2dd1049)
- **(ubi)** add option `rename_exe` by [@wlmitch](https://github.com/wlmitch) in [ce932ed](https://github.com/jdx/mise/commit/ce932edffd15030e3cac62307360b44f7fb385c4)
- use aqua for hk by [@jdx](https://github.com/jdx) in [f68de38](https://github.com/jdx/mise/commit/f68de3849c5ceb20475f2f30224abaa5f3f7441d)
- add bazel-watcher to registry by [@betaboon](https://github.com/betaboon) in [f5798eb](https://github.com/jdx/mise/commit/f5798ebd9a337a9397ffb34f6afeb40c0772c750)

### 🐛 Bug Fixes

- behavior of .disable-self-update by [@ZeroAurora](https://github.com/ZeroAurora) in [8123465](https://github.com/jdx/mise/commit/812346505a9da2f41dcee3d85bb5be1cc6c8d4a9)
- devcontainer by [@acesyde](https://github.com/acesyde) in [4cf8a1f](https://github.com/jdx/mise/commit/4cf8a1f0a9504038bcf291a2ecd1ffd5b49fb090)
- mise outdated --json does not return json if all tools are up-to-date by [@roele](https://github.com/roele) in [b93076e](https://github.com/jdx/mise/commit/b93076e960ad7b1fb7cb24d5037b780063c4572d)
- bug when using mise use -g when MISE_ENV is filled by [@roele](https://github.com/roele) in [7f249ac](https://github.com/jdx/mise/commit/7f249ac3ee45192c0da25f93fc13425818a920bd)
- config of symlink tracked on windows is not respected by [@NavyD](https://github.com/NavyD) in [c0837c9](https://github.com/jdx/mise/commit/c0837c90f52487e40f7ce6af6edd87966cd131fa)
- pruning unused tool leaves broken symlinks by [@roele](https://github.com/roele) in [b4db224](https://github.com/jdx/mise/commit/b4db2249e6a27ed051e61d68e883d247def6d20d)

### 📚 Documentation

- Fixes typo in lang/zig by [@carldaws](https://github.com/carldaws) in [7575910](https://github.com/jdx/mise/commit/7575910b2086d82676528024677b88f02337c5b1)
- Fix activation on PowerShell by [@kit494way](https://github.com/kit494way) in [a7bfb4b](https://github.com/jdx/mise/commit/a7bfb4b5c05e3618c8cdb27db472c5fd82f73c89)

### Chore

- remove aur job by [@jdx](https://github.com/jdx) in [fe5a71d](https://github.com/jdx/mise/commit/fe5a71dc486e6e585167d9d97018f2b467bc43fe)
- remove reference to aur in release script by [@jdx](https://github.com/jdx) in [0824490](https://github.com/jdx/mise/commit/0824490c14d17cd93c7d68930b514eb11635c451)
- deny ring sec by [@jdx](https://github.com/jdx) in [08e334c](https://github.com/jdx/mise/commit/08e334cb1209471d9c18b289473925ff0931053f)

### New Contributors

- @betaboon made their first contribution
- @ldrouard made their first contribution
- @qianlongzt made their first contribution
- @wlmitch made their first contribution
- @carldaws made their first contribution
- @ZeroAurora made their first contribution

## [2025.2.7](https://github.com/jdx/mise/compare/v2025.2.6..v2025.2.7) - 2025-02-19

### 🚀 Features

- **(registry)** add lychee to registry by [@eread](https://github.com/eread) in [26bba02](https://github.com/jdx/mise/commit/26bba0271aa39e889da2fa94244869876e74efa2)
- Install latest nominated zig from https://machengine.org/zig/index.json by [@tamadamas](https://github.com/tamadamas) in [70f923c](https://github.com/jdx/mise/commit/70f923c9fcfb2497669ede8777499b2fc5fa3323)

### 🐛 Bug Fixes

- **(cli/run)** inherit stdio by --raw even when redactions are enabled by [@risu729](https://github.com/risu729) in [6b6fea7](https://github.com/jdx/mise/commit/6b6fea71b2b9cd02aa3c868e3d1399cf86c8ced1)
- **(task)** Running programs on windows without cmd.exe by [@NavyD](https://github.com/NavyD) in [e68b624](https://github.com/jdx/mise/commit/e68b62484fbbf8a4cfc081221452179328b6de63)
- bugs with grep in tar_supports_zstd in mise.run script by [@glasser](https://github.com/glasser) in [f8bd7b5](https://github.com/jdx/mise/commit/f8bd7b55c9e55c9676291376c0bc5f5c35b4e466)

### 📚 Documentation

- fix watch files hook example by [@rsyring](https://github.com/rsyring) in [0930b25](https://github.com/jdx/mise/commit/0930b250a0c451b22f6ae266b0f15df84ebd7bc8)
- Fix run-on sentence by [@henrebotha](https://github.com/henrebotha) in [4d081c7](https://github.com/jdx/mise/commit/4d081c7e7a2d609cdd7ff423e9bb5e5bcc9ab4eb)
- mention hk by [@jdx](https://github.com/jdx) in [1a58e86](https://github.com/jdx/mise/commit/1a58e86ce2ce16d848755df8feccf514000053fd)
- discord link by [@jdx](https://github.com/jdx) in [b586085](https://github.com/jdx/mise/commit/b58608521cccee812adaa642145f061ccbcbac43)
- Add a section on how to use environment variables by [@hverlin](https://github.com/hverlin) in [b2529a2](https://github.com/jdx/mise/commit/b2529a2f9e98be23cf801c03538fbfb40f9eaebe)
- Update installation for archLinux by [@Nicknamely](https://github.com/Nicknamely) in [ce5fad1](https://github.com/jdx/mise/commit/ce5fad16c3d687b8759ebf6f6bf2af7859a08619)
- Fix typo in getting-started by [@alefteris](https://github.com/alefteris) in [82d215d](https://github.com/jdx/mise/commit/82d215d8ce69116eb562204ca4da11e408280c7a)

### 🧪 Testing

- always set experimental = true in tests by [@jdx](https://github.com/jdx) in [46e8783](https://github.com/jdx/mise/commit/46e8783024561601ab9742115a04513749d1457c)

### Chore

- fixed new clippy lints by [@jdx](https://github.com/jdx) in [6c3b333](https://github.com/jdx/mise/commit/6c3b3339d2d4865ea6759b2be363feb2d35decf6)

### New Contributors

- @alefteris made their first contribution
- @tamadamas made their first contribution
- @Nicknamely made their first contribution
- @eread made their first contribution
- @rsyring made their first contribution

## [2025.2.6](https://github.com/jdx/mise/compare/v2025.2.5..v2025.2.6) - 2025-02-16

### 🚀 Features

- add devcontainer generator by [@acesyde](https://github.com/acesyde) in [9d69158](https://github.com/jdx/mise/commit/9d69158c1fade56386ea1f6e040082888e2114eb)
- added hk by [@jdx](https://github.com/jdx) in [cef73c4](https://github.com/jdx/mise/commit/cef73c41919e2ac68d81ad4d6e6ccd1dd61190e3)

### 🐛 Bug Fixes

- short flag with value and var=#true bug by [@jdx](https://github.com/jdx) in [2340499](https://github.com/jdx/mise/commit/234049939c05cb114af2e5ff08492b83d3fcd537)
- regression with env overriding by [@jdx](https://github.com/jdx) in [f057457](https://github.com/jdx/mise/commit/f057457b81e0f2721955f2f4f680de7ad014bb57)

### 📚 Documentation

- **(shims)** clarify `activate` only removes shims from `PATH` by [@risu729](https://github.com/risu729) in [77032ac](https://github.com/jdx/mise/commit/77032acd6deefd19cb3cea2584574562b186f65c)
- Update shims page by [@hverlin](https://github.com/hverlin) in [c3881bf](https://github.com/jdx/mise/commit/c3881bf5fc034d89bd0c91d9f47a303552bf9da7)

## [2025.2.5](https://github.com/jdx/mise/compare/v2025.2.4..v2025.2.5) - 2025-02-16

### 🐛 Bug Fixes

- properly replace non set flags with "false" by [@IxDay](https://github.com/IxDay) in [07da7da](https://github.com/jdx/mise/commit/07da7da0e0fbfcadbc6de2876bcb3479b6260e88)
- path env order with subdirs by [@jdx](https://github.com/jdx) in [ebfb076](https://github.com/jdx/mise/commit/ebfb076a15313a2d0e650aea19b1ec046fc6098e)

### ◀️ Revert

- "feat: set usage arguments and flags as environment variables for toml tasks" by [@jdx](https://github.com/jdx) in [3bbb222](https://github.com/jdx/mise/commit/3bbb222fd37fdb8079069d3609b86dc5c6ddfc6c)

## [2025.2.4](https://github.com/jdx/mise/compare/v2025.2.3..v2025.2.4) - 2025-02-14

### 🚀 Features

- **(registry)** add e1s by [@kiwamizamurai](https://github.com/kiwamizamurai) in [f3b5247](https://github.com/jdx/mise/commit/f3b52471061ab4f425f5260ef7b4c3557136e283)
- **(registry)** add 'marksman' via 'aqua:artempyanykh/marksman' backend by [@iamoeg](https://github.com/iamoeg) in [a884ddc](https://github.com/jdx/mise/commit/a884ddca2ba52931d4851c0ef552bc3392096603)
- use `machengine.org` for downloading nominated zig versions by [@hadronomy](https://github.com/hadronomy) in [344e37c](https://github.com/jdx/mise/commit/344e37c8a0e29fe8e571d43f7fd02e4c7dfe384a)

### 🐛 Bug Fixes

- **(aqua)** apply override of version_prefix by [@risu729](https://github.com/risu729) in [8dda731](https://github.com/jdx/mise/commit/8dda73111f181ce93d9939a0bad71929833be896)
- **(env_directive)** apply redactions only to env with redact by [@risu729](https://github.com/risu729) in [d5c1be0](https://github.com/jdx/mise/commit/d5c1be04f82b17fe93a808100538be04bf49b4ac)
- **(hook_env)** don't exit early if watching files are deleted by [@risu729](https://github.com/risu729) in [a1a3c9f](https://github.com/jdx/mise/commit/a1a3c9fe7e4b50a951beb940222a5ea3a90577c4)
- **(rubygems_plugin)** Replace which ruby check for Windows compatibility by [@genskyff](https://github.com/genskyff) in [bbfce59](https://github.com/jdx/mise/commit/bbfce59d58338302cd714815e0b0ad1ecd2043ff)
- lowercase desired shim names by [@KevSlashNull](https://github.com/KevSlashNull) in [4e0557f](https://github.com/jdx/mise/commit/4e0557f6e9aedc200f7016ac01234ba3924ad38a)
- allow cosign opts to be empty in aqua by [@IxDay](https://github.com/IxDay) in [d9ac9c2](https://github.com/jdx/mise/commit/d9ac9c2e56b110d6af6ded967dff54977884af57)

### 📚 Documentation

- update Fedora install for dnf5 by [@rkben](https://github.com/rkben) in [fddcf15](https://github.com/jdx/mise/commit/fddcf151efde3ef84203edd878fb76b912a32d63)
- fix links to idiomatic version file option by [@pietrodn](https://github.com/pietrodn) in [b6c0595](https://github.com/jdx/mise/commit/b6c05951e21cfc88d2971741231a576cb1a9eb83)
- add mise bootstrap example in CI docs by [@hverlin](https://github.com/hverlin) in [2b004f4](https://github.com/jdx/mise/commit/2b004f478ec67113f41b60b87b02b8a87086c3f3)
- Update link in comparison-to-asdf.md by [@hverlin](https://github.com/hverlin) in [2365292](https://github.com/jdx/mise/commit/23652922ee703ee1e6009076e9cd8d483a132c82)

### 📦️ Dependency Updates

- update rust crate bzip2 to v0.5.1 by [@renovate[bot]](https://github.com/renovate[bot]) in [af39ef6](https://github.com/jdx/mise/commit/af39ef6a9f429f3e2188a45649ea67d9ed0a491f)
- update rust crate built to v0.7.6 by [@renovate[bot]](https://github.com/renovate[bot]) in [f90a7e4](https://github.com/jdx/mise/commit/f90a7e41e6762658271e3ae5e4b27995f66a8904)

### Chore

- issue closer by [@jdx](https://github.com/jdx) in [bee1f55](https://github.com/jdx/mise/commit/bee1f5557b829b9a637a28af90b519fdfa74b8dd)

### New Contributors

- @iamoeg made their first contribution
- @hadronomy made their first contribution
- @pietrodn made their first contribution
- @genskyff made their first contribution
- @kiwamizamurai made their first contribution
- @rkben made their first contribution
- @IxDay made their first contribution
- @KevSlashNull made their first contribution

## [2025.2.3](https://github.com/jdx/mise/compare/v2025.2.2..v2025.2.3) - 2025-02-09

## [2025.2.2](https://github.com/jdx/mise/compare/v2025.2.1..v2025.2.2) - 2025-02-08

### 🚀 Features

- **(registry)** add jd by [@risu729](https://github.com/risu729) in [8ba107a](https://github.com/jdx/mise/commit/8ba107ab6ff7156d95d5a6d5bdd3001c6268dd89)
- **(registry)** add jc by [@risu729](https://github.com/risu729) in [cd8dc01](https://github.com/jdx/mise/commit/cd8dc01d80b8d46d9985736bf745069da9e18afe)
- **(registry)** Add qsv cli by [@vjda](https://github.com/vjda) in [16d5a28](https://github.com/jdx/mise/commit/16d5a287552aa80540b50687b9bc18cd6425089f)
- add support for idiomatic go.mod file by [@roele](https://github.com/roele) in [842d051](https://github.com/jdx/mise/commit/842d051e22ee0bb317da0a0e86969cf3b9c1daca)
- add -g short version for unuse cmd by [@kimle](https://github.com/kimle) in [e11027a](https://github.com/jdx/mise/commit/e11027aa24953fc800eff7d7a7118e0cf6cda04c)
- add git remote task provider by [@acesyde](https://github.com/acesyde) in [1bf4a0f](https://github.com/jdx/mise/commit/1bf4a0f0293366cdf23218911c296c47ea031f74)
- set usage arguments and flags as environment variables for toml tasks by [@gturi](https://github.com/gturi) in [2613829](https://github.com/jdx/mise/commit/26138291e3e994861edb6641e0706dd653065a94)

### 🐛 Bug Fixes

- **(aqua)** trim prefix before comparing versions by [@risu729](https://github.com/risu729) in [1de278d](https://github.com/jdx/mise/commit/1de278d124106c3975da0f3d33aea43b738cb02e)
- wrong config file type for rust-toolchain.toml files by [@roele](https://github.com/roele) in [ba43871](https://github.com/jdx/mise/commit/ba43871ae67ad205b848b60d294a910637324953)

### 🚜 Refactor

- **(registry)** use aqua for yq by [@scop](https://github.com/scop) in [ded48ca](https://github.com/jdx/mise/commit/ded48ca6fbdd49e1737e80f76b45ba97db3f369a)

### 📚 Documentation

- **(schema)** fix description of task.dir default by [@risu729](https://github.com/risu729) in [62f4ca0](https://github.com/jdx/mise/commit/62f4ca0d42efb89482defe629c0b5c7a4663ae6d)
- Add PowerShell example by [@jahanson](https://github.com/jahanson) in [67eb899](https://github.com/jdx/mise/commit/67eb899d6ef306c25c83281ff9c750b7e377cff7)
- Include "A Mise guide for Swift developers" by [@pepicrft](https://github.com/pepicrft) in [c1f69f4](https://github.com/jdx/mise/commit/c1f69f47828423fed99c6e8bcd0d4e6f1301bde5)
- Update documentation for core tools by [@hverlin](https://github.com/hverlin) in [bdf2939](https://github.com/jdx/mise/commit/bdf29399291defc188dd899ec7612afb0609e47f)
- Update vitepress to fix search by [@hverlin](https://github.com/hverlin) in [f292094](https://github.com/jdx/mise/commit/f2920948f18ff55b7ffb006e93a84dc9f0b70535)

### Chore

- **(bun.lock)** migrate bun lockfiles to text-based by [@risu729](https://github.com/risu729) in [ebfe0da](https://github.com/jdx/mise/commit/ebfe0dabd5789a843206c0f13a380cf27f33dde8)

### New Contributors

- @vjda made their first contribution
- @kimle made their first contribution
- @pepicrft made their first contribution
- @jahanson made their first contribution

## [2025.2.1](https://github.com/jdx/mise/compare/v2025.2.0..v2025.2.1) - 2025-02-03

### Chore

- fix winget releaser job by [@jdx](https://github.com/jdx) in [e67c653](https://github.com/jdx/mise/commit/e67c653de35ff83d4ee280bf5cb2381741a2108e)

## [2025.2.0](https://github.com/jdx/mise/compare/v2025.1.17..v2025.2.0) - 2025-02-02

### 🚀 Features

- **(registry)** add kwokctl by [@mangkoran](https://github.com/mangkoran) in [c8546ef](https://github.com/jdx/mise/commit/c8546efbe1f5d126711f1718b0231bcf46cdc234)
- add biome to registry by [@kit494way](https://github.com/kit494way) in [d260468](https://github.com/jdx/mise/commit/d2604680978e40570fc16200b684a65a6798c931)
- add gittool/gitversion by [@acesyde](https://github.com/acesyde) in [48e6aae](https://github.com/jdx/mise/commit/48e6aae09003f1a18b36b42fbad899d2893277d2)

### 📚 Documentation

- add filtering support to registry docs page by [@roele](https://github.com/roele) in [fca4aa4](https://github.com/jdx/mise/commit/fca4aa439625c42ac9152fa3d0155a200ab11734)
- improve registry filtering performance by [@roele](https://github.com/roele) in [2769d89](https://github.com/jdx/mise/commit/2769d89cddb53303ade4a1edd9d2f79aecd875a4)
- fix registry table rendering for mobile by [@roele](https://github.com/roele) in [5de7936](https://github.com/jdx/mise/commit/5de79368fdcfa1d6ce89048516f700ab2e30896d)

### Chore

- updated deps by [@jdx](https://github.com/jdx) in [125e8ff](https://github.com/jdx/mise/commit/125e8ff8ee88c824e39bc93cc53996f32fea50b4)
- do not run autofix on renovate PRs by [@jdx](https://github.com/jdx) in [41c5ce4](https://github.com/jdx/mise/commit/41c5ce4c6581f856bf0d756e3fe99ec2fae2e7bd)

### New Contributors

- @ELLIOTTCABLE made their first contribution

## [2025.1.17](https://github.com/jdx/mise/compare/v2025.1.16..v2025.1.17) - 2025-01-31

### 🚀 Features

- **(registry)** use aqua for duckdb by [@mangkoran](https://github.com/mangkoran) in [c85741e](https://github.com/jdx/mise/commit/c85741e41d939e95f849537c27706a5177a0fde9)

### 🐛 Bug Fixes

- mise does not operate well under Git Bash on Windows by [@roele](https://github.com/roele) in [5750630](https://github.com/jdx/mise/commit/575063067d29f0bffd468587c7efee3a8f4c6dab)
- mise rm removes/reports wrong version of tool by [@roele](https://github.com/roele) in [0527f18](https://github.com/jdx/mise/commit/0527f18ba310aa3d68cd31cf20d705bb32c9fa5b)

### 📚 Documentation

- Update python documentation by [@hverlin](https://github.com/hverlin) in [3156d3c](https://github.com/jdx/mise/commit/3156d3c690763c708f054fc326016dbd60c81c14)
- fix postinstall typo in nodejs cookbook by [@arafays](https://github.com/arafays) in [c610d18](https://github.com/jdx/mise/commit/c610d185630d8f4571f1e0c4075e79ef0f92ebaf)
- Fix typo by [@henrebotha](https://github.com/henrebotha) in [9457278](https://github.com/jdx/mise/commit/9457278c91284bcc06f92d5439e49541f228612c)

### Hooks.md

- MISE_PROJECT_DIR -> MISE_PROJECT_ROOT by [@jubr](https://github.com/jubr) in [d10ba77](https://github.com/jdx/mise/commit/d10ba775db0e55ea24737a59cba909de7b2469d5)

### New Contributors

- @mangkoran made their first contribution
- @jubr made their first contribution
- @arafays made their first contribution

## [2025.1.16](https://github.com/jdx/mise/compare/v2025.1.15..v2025.1.16) - 2025-01-29

### 🚀 Features

- **(registry)** add duckdb by [@swfz](https://github.com/swfz) in [bc1c4a0](https://github.com/jdx/mise/commit/bc1c4a03f6cd315a3e333537bc9d138c4857211e)

### 🐛 Bug Fixes

- Swift on Ubuntu 24.04 arm64 generates the incorrect download URL by [@spyder-ian](https://github.com/spyder-ian) in [31bd9ba](https://github.com/jdx/mise/commit/31bd9ba54cf887f7e2fd4d9e7c18f498181d1e13)
- Do not attempt to parse directories by [@adamcohen2](https://github.com/adamcohen2) in [d2fd341](https://github.com/jdx/mise/commit/d2fd341c36c9ddb87880a04f17ae14dcc78fa0ae)
- path option should take precedence over global configuration by [@roele](https://github.com/roele) in [f342d0a](https://github.com/jdx/mise/commit/f342d0a4fce7c6b2e4ec25506c690a64805a98e5)

### 📚 Documentation

- Add devtools.fm episode about mise to external-resources.md by [@CanRau](https://github.com/CanRau) in [405a730](https://github.com/jdx/mise/commit/405a730b17ac5cdbef331ffb100e9fb25c02b360)
- Update sections about idiomatic version files by [@hverlin](https://github.com/hverlin) in [64f4698](https://github.com/jdx/mise/commit/64f4698d5d3f7fbd0df94856035ad1f0d5e1c4f0)

### Chore

- make self_update optional by [@jdx](https://github.com/jdx) in [d1730f0](https://github.com/jdx/mise/commit/d1730f0d268c2c7709715c299d0ceef320ff033e)
- added some defaul reqwest features by [@jdx](https://github.com/jdx) in [052cb4d](https://github.com/jdx/mise/commit/052cb4d2aff1e32652249f2c5ee5c04267b84067)

### New Contributors

- @adamcohen2 made their first contribution
- @CanRau made their first contribution
- @spyder-ian made their first contribution

## [2025.1.15](https://github.com/jdx/mise/compare/v2025.1.14..v2025.1.15) - 2025-01-26

### 🚀 Features

- add http cache by [@acesyde](https://github.com/acesyde) in [afa89de](https://github.com/jdx/mise/commit/afa89de9e7f944309eb35b7044a73f71ca883557)
- expose `test-tool` command by [@jdx](https://github.com/jdx) in [3ca3d9c](https://github.com/jdx/mise/commit/3ca3d9c9335c4e8327293e46c65a57773071454a)

### 🐛 Bug Fixes

- elixir installation failed by [@roele](https://github.com/roele) in [668d152](https://github.com/jdx/mise/commit/668d152c9cfcd9fdb7789c9e40edeacec4338994)
- re-run tasks when files removed or permissions change by [@jdx](https://github.com/jdx) in [9f26db8](https://github.com/jdx/mise/commit/9f26db85a315c70f527f2ce54bc612ee8ad56e65)

### 🚜 Refactor

- use builder pattern by [@acesyde](https://github.com/acesyde) in [3a2ee60](https://github.com/jdx/mise/commit/3a2ee60bed3307b2db45331e05b0783f8f927115)

### 📚 Documentation

- **(how-i-use-mise)** switch to discussion by [@risu729](https://github.com/risu729) in [cb6efa1](https://github.com/jdx/mise/commit/cb6efa1b1b1fcbf343c66039e21962fdba0f1bcc)
- add hint about environment variable parsing by [@roele](https://github.com/roele) in [93a3106](https://github.com/jdx/mise/commit/93a310638ba3478a3412e5d7407fb39f64a1687e)

### Chore

- added vscode workspace by [@jdx](https://github.com/jdx) in [a0d181f](https://github.com/jdx/mise/commit/a0d181f8d60270d09d06156ebc500a2fa85f74db)
- switch from git2 to gix by [@jdx](https://github.com/jdx) in [3463ef1](https://github.com/jdx/mise/commit/3463ef185d2c6026a62260217d07273a5326cc1b)
- remove git2 from built by [@jdx](https://github.com/jdx) in [d88870c](https://github.com/jdx/mise/commit/d88870ce8100dc151580bd726cc8dc31bec47a19)
- use mise-plugins/mise-jib by [@jdx](https://github.com/jdx) in [fd67a35](https://github.com/jdx/mise/commit/fd67a351ea1f04218dd9fbf72e828230d635f622)

### New Contributors

- @vgnh made their first contribution

## [2025.1.14](https://github.com/jdx/mise/compare/v2025.1.13..v2025.1.14) - 2025-01-24

### 🚀 Features

- **(registry)** add gron by [@MontakOleg](https://github.com/MontakOleg) in [fba0c9b](https://github.com/jdx/mise/commit/fba0c9b5eef287477fd0b3160d0556ded5ed0b73)

### 🐛 Bug Fixes

- spurious semver warning on `mise outdated` by [@jdx](https://github.com/jdx) in [637cd32](https://github.com/jdx/mise/commit/637cd32cd8b24c37fce99fe6953bc57fd164df71)

### Chore

- lint issue in Dockerfile by [@jdx](https://github.com/jdx) in [47ad5d6](https://github.com/jdx/mise/commit/47ad5d67890188478cf8c8f2e6796b6752546e6c)
- fix some typos in markdown file by [@chuangjinglu](https://github.com/chuangjinglu) in [96bbc58](https://github.com/jdx/mise/commit/96bbc5833f7b76fff5a62337356d762034d6cfea)
- pin aws-cli by [@jdx](https://github.com/jdx) in [f7311fd](https://github.com/jdx/mise/commit/f7311fd8fc85b6920c5a484862865adc9ef7261d)
- use arm64 runners for docker by [@jdx](https://github.com/jdx) in [74e3269](https://github.com/jdx/mise/commit/74e3269f78f4599e5415b61d8da1f55ee9f94021)

### New Contributors

- @chuangjinglu made their first contribution

## [2025.1.13](https://github.com/jdx/mise/compare/v2025.1.12..v2025.1.13) - 2025-01-24

### Chore

- fixing aws-cli in release.sh by [@jdx](https://github.com/jdx) in [5b4a65a](https://github.com/jdx/mise/commit/5b4a65a84e07141de9ed69798921b4b0ef69aa02)
- fixing aws-cli in release.sh by [@jdx](https://github.com/jdx) in [4c67db5](https://github.com/jdx/mise/commit/4c67db59ecfb55eb724dc05bca7eb7281a625929)

## [2025.1.12](https://github.com/jdx/mise/compare/v2025.1.11..v2025.1.12) - 2025-01-24

### Chore

- setup mise for release task by [@jdx](https://github.com/jdx) in [78d3dfb](https://github.com/jdx/mise/commit/78d3dfb164776cfb39a1920485c21fcd6ecd3ebe)

## [2025.1.11](https://github.com/jdx/mise/compare/v2025.1.10..v2025.1.11) - 2025-01-23

### Chore

- pin aws-cli by [@jdx](https://github.com/jdx) in [ca16daf](https://github.com/jdx/mise/commit/ca16daf5e5dbb9159d853570528087b24f63500b)

## [2025.1.10](https://github.com/jdx/mise/compare/v2025.1.9..v2025.1.10) - 2025-01-23

### 🚀 Features

- **(registry)** use aqua for periphery by [@MontakOleg](https://github.com/MontakOleg) in [f675c8f](https://github.com/jdx/mise/commit/f675c8f36aace3e3a18509f2e6fe7a7a6a1f742d)
- split remote task by [@acesyde](https://github.com/acesyde) in [727aee8](https://github.com/jdx/mise/commit/727aee866ca3e367664e4b0cc96f41c5c4d9085f)

### 🐛 Bug Fixes

- **(docs)** environment variable MISE_OVERRIDE_TOOL_VERSIONS_FILENAME should be plural by [@roele](https://github.com/roele) in [694391f](https://github.com/jdx/mise/commit/694391fc575cc2b317016865ec17400c62af43db)
- completions were missing non-asdf tools by [@jdx](https://github.com/jdx) in [55b31a4](https://github.com/jdx/mise/commit/55b31a452b807ada4e2ba40c8b5588b77b79642e)
- broken link for `/tasks/task-configuration` by [@134130](https://github.com/134130) in [4194265](https://github.com/jdx/mise/commit/4194265b0849e4a9035da70cab466ed78c062a18)
- whitespace in mise.run script by [@jdx](https://github.com/jdx) in [e264644](https://github.com/jdx/mise/commit/e264644cbf5c61a573e15b118c3ad8005a65c8a0)
- confusing error in fish_command_not_found by [@MrGreenTea](https://github.com/MrGreenTea) in [c79b483](https://github.com/jdx/mise/commit/c79b4833c9dcb5245b8308dab2ac08ff10db352e)
- use correct python path for venv creation in windows by [@tisoft](https://github.com/tisoft) in [5b2d795](https://github.com/jdx/mise/commit/5b2d79500ab89b2a4fe1a09a5abbcce1bde3cc55)

### 📚 Documentation

- neovim cookbook by [@EricDriussi](https://github.com/EricDriussi) in [491fb36](https://github.com/jdx/mise/commit/491fb36da8b23d56d4b68c996b50b132d525a56a)

### 🧪 Testing

- fix a couple of tool tests by [@jdx](https://github.com/jdx) in [9ef7056](https://github.com/jdx/mise/commit/9ef70564f64e842af31882658a113128f7d4649d)

### Chore

- added issue auto-closer by [@jdx](https://github.com/jdx) in [3c831c1](https://github.com/jdx/mise/commit/3c831c19a644fbb2f393f969ebaa5137f9415793)

### New Contributors

- @tisoft made their first contribution
- @MrGreenTea made their first contribution
- @EricDriussi made their first contribution
- @134130 made their first contribution

## [2025.1.9](https://github.com/jdx/mise/compare/v2025.1.8..v2025.1.9) - 2025-01-17

### 🚀 Features

- **(aqua)** pass --verbose flag down to cosign and added aqua.cosign_extra_args setting by [@jdx](https://github.com/jdx) in [f1dec69](https://github.com/jdx/mise/commit/f1dec69edf048070aa73b3e3e994e74f88dab552)
- **(doctor)** display redacted github token by [@jdx](https://github.com/jdx) in [98440d6](https://github.com/jdx/mise/commit/98440d6183d0c5870416326fdcad3c0e8980fd66)

### 🐛 Bug Fixes

- Fixes fish_command_not_found glob error by [@halostatue](https://github.com/halostatue) in [6ccb1d9](https://github.com/jdx/mise/commit/6ccb1d92e3b4a9bbb75f523bbf18f69441c95707)
- completions for `mise use` by [@jdx](https://github.com/jdx) in [2d32a6d](https://github.com/jdx/mise/commit/2d32a6d91eda87988b0bed1ba0a993eaeb0a0504)

### 🛡️ Security

- **(ruby)** remove ruby/gem tests by [@jdx](https://github.com/jdx) in [de703d8](https://github.com/jdx/mise/commit/de703d8baa436d3a2adc79a22d63837e40055501)

### 📦️ Dependency Updates

- update dependency bun to v1.1.44 by [@renovate[bot]](https://github.com/renovate[bot]) in [1b5e740](https://github.com/jdx/mise/commit/1b5e7406ad969e8fc461d088beaf7886111bfbc3)

### Chore

- add install.sh.sig to releases by [@jdx](https://github.com/jdx) in [1b6ea86](https://github.com/jdx/mise/commit/1b6ea8644edcf3a6ff68fc6d511622c44f1f1f9a)

### New Contributors

- @halostatue made their first contribution

## [2025.1.8](https://github.com/jdx/mise/compare/v2025.1.7..v2025.1.8) - 2025-01-17

### 🚀 Features

- upgrade ubi by [@jdx](https://github.com/jdx) in [cad188c](https://github.com/jdx/mise/commit/cad188c88e21f53db39eab7d6597ed7c37f20ef0)
- enable erlang for Windows by [@roele](https://github.com/roele) in [33a6390](https://github.com/jdx/mise/commit/33a63904cfa490af824a6f1879be4cf1fafdf81a)
- use aqua for opentofu by [@jdx](https://github.com/jdx) in [41dd67f](https://github.com/jdx/mise/commit/41dd67f451902bf260fea53497601140373036d3)

### 🐛 Bug Fixes

- **(spm)** install from annotated tag by [@MontakOleg](https://github.com/MontakOleg) in [ca0fcee](https://github.com/jdx/mise/commit/ca0fceea49089e99226a2fbe5633de6d9aff84e9)
- Fixes infinite loop in auto install not found bash function by [@bnorick](https://github.com/bnorick) in [fcbe2c4](https://github.com/jdx/mise/commit/fcbe2c421a51745283cfba9c419200318a769164)
- installing with empty version fails by [@roele](https://github.com/roele) in [a6f95d9](https://github.com/jdx/mise/commit/a6f95d9618e447942c1c5e67f100b54f4e2c917d)

### 📚 Documentation

- correct link to gem.rs source by [@petrblaho](https://github.com/petrblaho) in [c36cba7](https://github.com/jdx/mise/commit/c36cba76e89d0fa6a7c9272cb4c8cb19a94f6d36)
- fix {{config_root}} got interpolated by vitepress by [@peter50216](https://github.com/peter50216) in [3e004f6](https://github.com/jdx/mise/commit/3e004f61da95ffc2787a4a5975fc6d9ef2a5c13b)

### Chore

- remove minisign from mise.toml by [@jdx](https://github.com/jdx) in [b115ba9](https://github.com/jdx/mise/commit/b115ba962fce4e63e0d6ce85f41704f302ef3e9a)

### New Contributors

- @peter50216 made their first contribution
- @petrblaho made their first contribution

## [2025.1.7](https://github.com/jdx/mise/compare/v2025.1.6..v2025.1.7) - 2025-01-15

### 🚀 Features

- **(registry)** add gup by [@scop](https://github.com/scop) in [4fe03b9](https://github.com/jdx/mise/commit/4fe03b972c6f20711ed9d376b3a4e4f607c6d2ee)
- **(registry)** add aqua and cmdx by [@scop](https://github.com/scop) in [14fa24f](https://github.com/jdx/mise/commit/14fa24f485958ec9f2e37fb60b1059e1546ea872)
- use aqua for eza on linux by [@jdx](https://github.com/jdx) in [24d5ca2](https://github.com/jdx/mise/commit/24d5ca2c1acfdd0a227dd1cc509c0221685ca4ba)
- allow to specify Rust profile by [@roele](https://github.com/roele) in [3f6400f](https://github.com/jdx/mise/commit/3f6400fd3f4e1c9a19c12ce6caa257e970eb4ade)

### 🐛 Bug Fixes

- use vars in [env] templates by [@hverlin](https://github.com/hverlin) in [97fee36](https://github.com/jdx/mise/commit/97fee3645d343931965a0a43fd47fc4bf515c200)
- panic when directory name contains japanese characters by [@roele](https://github.com/roele) in [1b0c5d7](https://github.com/jdx/mise/commit/1b0c5d716908a3a3ff7d677b3fdbb3f678dd0637)
- incorrect config_root for project/.mise/config.toml by [@roele](https://github.com/roele) in [667f529](https://github.com/jdx/mise/commit/667f529a437ad25b2ec28df6defca96182e457ac)

### 🚜 Refactor

- **(registry)** alias protobuf to protoc by [@scop](https://github.com/scop) in [801a880](https://github.com/jdx/mise/commit/801a880be383dbdd1502a857b7281b431489a1e5)
- **(registry)** use aqua for go-getter and kcl by [@scop](https://github.com/scop) in [6027d66](https://github.com/jdx/mise/commit/6027d66610b2cf4892543f9349f8307a46af9a2b)
- **(registry)** use aqua for powerline-go by [@scop](https://github.com/scop) in [4006498](https://github.com/jdx/mise/commit/40064987fe16360f229027c18463bc46a49346e0)

### 📚 Documentation

- clean up activation instructions by [@jdx](https://github.com/jdx) in [e235c74](https://github.com/jdx/mise/commit/e235c74daa8f5e5f9e1bb89c70a6cff96c08956e)
- correct urls for crawler by [@jdx](https://github.com/jdx) in [21cb77b](https://github.com/jdx/mise/commit/21cb77b1f79a57e6ebd3fec367bd5b223239a3ed)
- added sitemap meta tag by [@jdx](https://github.com/jdx) in [033aa14](https://github.com/jdx/mise/commit/033aa149e8b7a45ea750c09c31438709420214c8)

## [2025.1.6](https://github.com/jdx/mise/compare/v2025.1.5..v2025.1.6) - 2025-01-12

### 🐛 Bug Fixes

- Panic when run without arguments with bootstrapped script by [@jdx](https://github.com/jdx) in [726a0f4](https://github.com/jdx/mise/commit/726a0f4867e63f4dbc5523ff291a142ba3370ea2)

### 🚜 Refactor

- use better rust syntax by [@jdx](https://github.com/jdx) in [868e0ee](https://github.com/jdx/mise/commit/868e0ee82dd7e447a489f8629965bc18f1a13621)

### 📚 Documentation

- fix TOML-based Tasks usage spec example by [@gturi](https://github.com/gturi) in [4689f3d](https://github.com/jdx/mise/commit/4689f3de13c85bc759e6eee330edda4a16376725)
- eza by [@jdx](https://github.com/jdx) in [5a80cbf](https://github.com/jdx/mise/commit/5a80cbf9e0b37be800bc6f6f0404bcf86cbe3bd9)
- removed bit about verifying with asdf by [@jdx](https://github.com/jdx) in [d505486](https://github.com/jdx/mise/commit/d505486fbbe49af0f7bf6029569812441c1e3fdc)
- added more getting started installers by [@jdx](https://github.com/jdx) in [b310e11](https://github.com/jdx/mise/commit/b310e118b00d2b0a64cf2d423d20ece6dc9692f6)
- clean up activation instructions by [@jdx](https://github.com/jdx) in [3df60dd](https://github.com/jdx/mise/commit/3df60dd9cbecf3086b1755d4e397159379d27b27)
- clean up activation instructions by [@jdx](https://github.com/jdx) in [8ab4bce](https://github.com/jdx/mise/commit/8ab4bcef77c4bc1e07951dbb8b5787df4a4b15bf)
- clean up activation instructions by [@jdx](https://github.com/jdx) in [d4a67e8](https://github.com/jdx/mise/commit/d4a67e8ec72fed064cc776ab643f41da1ae01caa)
- clean up activation instructions by [@jdx](https://github.com/jdx) in [d208418](https://github.com/jdx/mise/commit/d208418a5f63803185c4aa5f06afecd9e8832496)
- clean up activation instructions by [@jdx](https://github.com/jdx) in [b9f581d](https://github.com/jdx/mise/commit/b9f581d644295f372eb0cd026560e9c97dcb8091)

### New Contributors

- @gturi made their first contribution

## [2025.1.5](https://github.com/jdx/mise/compare/v2025.1.4..v2025.1.5) - 2025-01-11

### 🚀 Features

- added gdu and dua to registry by [@sassdavid](https://github.com/sassdavid) in [e571d99](https://github.com/jdx/mise/commit/e571d994e82baa5f8a3b1160587d116692f4f51d)
- added prefix-dev/pixi by [@jdx](https://github.com/jdx) in [3e1511f](https://github.com/jdx/mise/commit/3e1511fb8900c5707546ac384e648eafc968b19f)
- added `mise cfg --tracked-configs` by [@jdx](https://github.com/jdx) in [80658bc](https://github.com/jdx/mise/commit/80658bc6bbd8b7c7a2e14b797502b712d2142cdc)
- added `mise version --json` flag by [@jdx](https://github.com/jdx) in [18338bc](https://github.com/jdx/mise/commit/18338bce6c6f6f40b59b7aaa39f5bea346de49f9)
- added `mise ls --prunable` flag by [@jdx](https://github.com/jdx) in [1db6945](https://github.com/jdx/mise/commit/1db6945064e12aee243b2725aa4998f95a7c4c7f)

### 🐛 Bug Fixes

- switch jib back to asdf by [@jdx](https://github.com/jdx) in [bdbba71](https://github.com/jdx/mise/commit/bdbba7140a8c99db9fd366084d41f797596434bb)
- `mise unuse` bug not pruning if not in config file by [@jdx](https://github.com/jdx) in [6f4e072](https://github.com/jdx/mise/commit/6f4e072be6f6885adece03427404199343873f06)

### 📚 Documentation

- explain pipx better by [@jdx](https://github.com/jdx) in [42dcb3b](https://github.com/jdx/mise/commit/42dcb3bc5a6547d3d148c391ceccfd9228e34669)

### 🧪 Testing

- added test case for `mise rm` by [@jdx](https://github.com/jdx) in [f7511b6](https://github.com/jdx/mise/commit/f7511b696c2ada7af878074e89b0dfc1edb73197)

### New Contributors

- @sassdavid made their first contribution

## [2025.1.4](https://github.com/jdx/mise/compare/v2025.1.3..v2025.1.4) - 2025-01-10

### 🚀 Features

- update JSON output for task info/ls by [@hverlin](https://github.com/hverlin) in [659287c](https://github.com/jdx/mise/commit/659287cb4508d7b000cbc9134265f208aff0c5d8)
- **breaking** bump usage to 2.x by [@jdx](https://github.com/jdx) in [4e34333](https://github.com/jdx/mise/commit/4e34333ff21cb695586282dcef34a48c5c735191)

### 🐛 Bug Fixes

- ignore github releases marked as draft by [@jdx](https://github.com/jdx) in [ce5334a](https://github.com/jdx/mise/commit/ce5334a2e4a62d17ec2aabc4137c8b4ae8bf3824)
- `mise run` shorthand with tasks that have an extension by [@jdx](https://github.com/jdx) in [8f92865](https://github.com/jdx/mise/commit/8f928659a002eea14f64767f6ad2f1ac1f1de8fc)
- use consistent casing by [@jdx](https://github.com/jdx) in [a4d4133](https://github.com/jdx/mise/commit/a4d41338139355b0dd86a068fd89790eb7e34584)
- support latest ansible packages by [@jdx](https://github.com/jdx) in [432e0c3](https://github.com/jdx/mise/commit/432e0c3c0ed27460fca436a760b966827be22569)
- use go backend for goconvey/ginkgo by [@jdx](https://github.com/jdx) in [a19ec94](https://github.com/jdx/mise/commit/a19ec949eecdc3607e412d0461a86745ccc42551)
- Improve fig spec with better generators by [@miguelmig](https://github.com/miguelmig) in [b96b2c8](https://github.com/jdx/mise/commit/b96b2c88c16d2563978bebc0fe53be2c7d5d28d9)

### 📚 Documentation

- set prose-wrap with prettier by [@jdx](https://github.com/jdx) in [2fba8b0](https://github.com/jdx/mise/commit/2fba8b0fd6be05ece012721123e45d3392e93e9f)
- Fix "Example of a NodeJS file task with arguments" by [@highb](https://github.com/highb) in [41932e9](https://github.com/jdx/mise/commit/41932e90f56f1f73ec624e8221cd843d8c4cbddb)

### 🧪 Testing

- disable some non-working plugins by [@jdx](https://github.com/jdx) in [106ee40](https://github.com/jdx/mise/commit/106ee40b463923bb5c6444e0c0127dabc502d9ee)
- remove test for flarectl by [@jdx](https://github.com/jdx) in [a63b449](https://github.com/jdx/mise/commit/a63b44910d55ad2cdc801a472f0c196c605cce25)

### Chore

- added `cargo check` to pre-commit by [@jdx](https://github.com/jdx) in [73eb25a](https://github.com/jdx/mise/commit/73eb25a88bbfe1b979bb5483ca3c81a689be184f)
- fix release-plz pr creation by [@jdx](https://github.com/jdx) in [8299c6b](https://github.com/jdx/mise/commit/8299c6b943119ffda94d18445c5b789948b6f9c0)
- use -q in pre-commit:check by [@jdx](https://github.com/jdx) in [099b2d8](https://github.com/jdx/mise/commit/099b2d88d3ed31ace30c67be816170dc50f87b6d)
- fix release-plz pr creation by [@jdx](https://github.com/jdx) in [c2accc5](https://github.com/jdx/mise/commit/c2accc5f7192202d0a8249ae7f3ab0ea7f100e1b)
- make prettier/pre-commit much faster by [@jdx](https://github.com/jdx) in [cf07782](https://github.com/jdx/mise/commit/cf077820253f1580245f991f1d858dc21b227e0e)
- fix release-plz edit command by [@jdx](https://github.com/jdx) in [86b5816](https://github.com/jdx/mise/commit/86b5816660f5a13d45c1795132a29e881645e271)

## [2025.1.3](https://github.com/jdx/mise/compare/v2025.1.2..v2025.1.3) - 2025-01-09

### 🐛 Bug Fixes

- **(rust)** respect RUSTUP_HOME/CARGO_HOME by [@jdx](https://github.com/jdx) in [0f7f498](https://github.com/jdx/mise/commit/0f7f498bfe5cfc10852ba3fd15294d9bc44ce2fe)
- mise fails to install kubectl on windows from aqua registry by [@roele](https://github.com/roele) in [5b30972](https://github.com/jdx/mise/commit/5b30972ecbcfbc9ee5a1ffd9b053fd97e4d6e966)
- aliases with aqua by [@jdx](https://github.com/jdx) in [e3bf99f](https://github.com/jdx/mise/commit/e3bf99f0230329c927f2d7009f851f4878c48141)
- issue with enter hook and subdirs by [@jdx](https://github.com/jdx) in [f9ee0be](https://github.com/jdx/mise/commit/f9ee0be9c568bde8d7a41a3b66539b53e947e53d)
- allow using depends and depends_post on separate tasks by [@jdx](https://github.com/jdx) in [dea1532](https://github.com/jdx/mise/commit/dea1532decb0928b79b57ae9413e0868fa022d3b)
- mise fails to install kubectl on windows from aqua registry by [@roele](https://github.com/roele) in [8afa8a4](https://github.com/jdx/mise/commit/8afa8a405012943f4ccc83dc05ae93007883e337)

### 📚 Documentation

- Add default description to github token link by [@hverlin](https://github.com/hverlin) in [b11167b](https://github.com/jdx/mise/commit/b11167b8d82bdb1e646dd9e76e66c520208a99a2)
- fix source code links by [@jdx](https://github.com/jdx) in [fa8350a](https://github.com/jdx/mise/commit/fa8350a4dc2ef6b21a4dfc2118d2ac901643583d)

### Chore

- make pre-commit faster by [@jdx](https://github.com/jdx) in [70dfdd0](https://github.com/jdx/mise/commit/70dfdd0b874a5292b4b20fa72c9c341a13900bde)
- added commented out paths config by [@jdx](https://github.com/jdx) in [c1f25ac](https://github.com/jdx/mise/commit/c1f25ac4cdaf74219d700fcaf37d3341971a3120)

## [2025.1.2](https://github.com/jdx/mise/compare/v2025.1.1..v2025.1.2) - 2025-01-08

### 🚀 Features

- migrate asdf plugins to aqua/ubi by [@jdx](https://github.com/jdx) in [74a4774](https://github.com/jdx/mise/commit/74a4774a03aab79c77e64c95a7fa5921e04cdd44)
- migrate asdf plugins to aqua/ubi by [@jdx](https://github.com/jdx) in [b6c66a6](https://github.com/jdx/mise/commit/b6c66a641ad0e74e0ffa79bb9b841b4440e82368)
- migrate asdf plugins to aqua/ubi by [@jdx](https://github.com/jdx) in [fd924f7](https://github.com/jdx/mise/commit/fd924f7a2f14dff16efdd319d592ff7919445289)
- replace asdf-spark plugin with mise-spark plugin by [@benberryallwood](https://github.com/benberryallwood) in [63a6b3a](https://github.com/jdx/mise/commit/63a6b3a86029e7441187ed55ea4c853871ee1ebc)
- add kubectx/kubens to registry by [@roele](https://github.com/roele) in [5c892e1](https://github.com/jdx/mise/commit/5c892e14b373f6d3df14ad6177756b118a5718bc)
- added ktlint from aqua by [@jdx](https://github.com/jdx) in [a8e6a7a](https://github.com/jdx/mise/commit/a8e6a7a6b04b74327dff133271a4103d0306f3fc)

### 🐛 Bug Fixes

- **(schema)** fix task sources and outputs schema by [@risu729](https://github.com/risu729) in [0c9f23a](https://github.com/jdx/mise/commit/0c9f23a980bbd325f6acf5eed77a7a91eece94af)
- **(schema)** update task schema by [@risu729](https://github.com/risu729) in [e9eaf11](https://github.com/jdx/mise/commit/e9eaf119514653cb22b9cf44a6bb5d8328d952c2)
- correct age keyname by [@jdx](https://github.com/jdx) in [e28c293](https://github.com/jdx/mise/commit/e28c293bc5a241b043d0b72ec9aa0559e888f97b)
- mise install rust failed on windows by [@roele](https://github.com/roele) in [8e79442](https://github.com/jdx/mise/commit/8e794426a80c253b4ce47301e911a6b663a52a14)
- maven-mvnd does not install with aqua by [@roele](https://github.com/roele) in [34c5ed6](https://github.com/jdx/mise/commit/34c5ed63bdac03466926cf14b399050c8c17d483)
- maven-mvnd does not install with aqua by [@roele](https://github.com/roele) in [de32f03](https://github.com/jdx/mise/commit/de32f037bc399c98c9b772741d859a0e8fc70eef)
- use friendly error in `mise run` by [@jdx](https://github.com/jdx) in [38e556f](https://github.com/jdx/mise/commit/38e556f10e79338dec2f2064378f37499b2c2cd5)
- use task display_name in more places by [@hverlin](https://github.com/hverlin) in [8d647a3](https://github.com/jdx/mise/commit/8d647a3b7abc79d68cefda4a602c6314bc1f57d8)
- aqua:apache/spark doesn't work by [@roele](https://github.com/roele) in [5ca6035](https://github.com/jdx/mise/commit/5ca603583166cc23543a0c4dce7727b308f663f5)

### 📚 Documentation

- style on rustup settings by [@jdx](https://github.com/jdx) in [da91716](https://github.com/jdx/mise/commit/da91716c856b0bb1e8bdf70f9f97f74fe09f15ac)
- Escape template examples by [@henrebotha](https://github.com/henrebotha) in [ef714c4](https://github.com/jdx/mise/commit/ef714c46f7eae1f0408aec9786a4f052ad253a2f)
- update SECURITY.md by [@jdx](https://github.com/jdx) in [6372f10](https://github.com/jdx/mise/commit/6372f101639386e94cd8df400c78962eab1dbdd5)

### 🧪 Testing

- fix test-plugins CI job for ubuntu-24 by [@jdx](https://github.com/jdx) in [492f6ac](https://github.com/jdx/mise/commit/492f6acc99014cb70f97efdd12700ee365a418ea)
- remove postgres test-plugins test by [@jdx](https://github.com/jdx) in [e93bc80](https://github.com/jdx/mise/commit/e93bc80a780fd0f7b4619af37c3f646dd622bed4)

### Chore

- remove deprecated tar syntax by [@jdx](https://github.com/jdx) in [322735a](https://github.com/jdx/mise/commit/322735a75bef9c602ffcec4d81914662cac00647)
- fix tar/gzip syntax by [@jdx](https://github.com/jdx) in [cd0a049](https://github.com/jdx/mise/commit/cd0a049ecace47354a931cd364ac2f5915812658)
- fork remaining asdf plugins to mise-plugins by [@jdx](https://github.com/jdx) in [5ca7227](https://github.com/jdx/mise/commit/5ca72270e2ae96bcf18e24519a47882046e1cb7f)

### New Contributors

- @henrebotha made their first contribution

## [2025.1.1](https://github.com/jdx/mise/compare/v2025.1.0..v2025.1.1) - 2025-01-06

### 🚀 Features

- add databricks-cli to registry by [@benberryallwood](https://github.com/benberryallwood) in [ca257db](https://github.com/jdx/mise/commit/ca257dbcd20537914d3f3a5019951ad25d9e16b8)
- add navi to registry by [@kit494way](https://github.com/kit494way) in [34e348c](https://github.com/jdx/mise/commit/34e348c6aa527c5e45fb49d916f2f15c55b43bee)
- added allurectl to registry by [@MontakOleg](https://github.com/MontakOleg) in [09e5252](https://github.com/jdx/mise/commit/09e52524c5d7e47ca9400b3ce5505aff0eebf048)
- Add setting description to mise settings --json-extended output by [@hverlin](https://github.com/hverlin) in [56c300e](https://github.com/jdx/mise/commit/56c300e19a3d4444ef451eca17f7015988b785b7)

### 🐛 Bug Fixes

- improve mise generate bootstrap by [@hverlin](https://github.com/hverlin) in [39908f0](https://github.com/jdx/mise/commit/39908f01dd20b2a8005092f0fc492cac96823ee5)
- update year in copyright to dynamic with current year by [@nexckycort](https://github.com/nexckycort) in [191ca32](https://github.com/jdx/mise/commit/191ca32e32402c212e03db3060665b240c82c217)

### 📚 Documentation

- Fix broken link to environment variables doc by [@xcapaldi](https://github.com/xcapaldi) in [f84ae91](https://github.com/jdx/mise/commit/f84ae91877ff264189b1e84b9fb757481efbd983)
- Add usage property to mise schema by [@hverlin](https://github.com/hverlin) in [1bbe9d8](https://github.com/jdx/mise/commit/1bbe9d8f73655257b55048184d8c777e22d8491c)
- clarity on relative paths vs config_root in _.path by [@glasser](https://github.com/glasser) in [ecfc3f6](https://github.com/jdx/mise/commit/ecfc3f645e51dc98cd5a0e1d2f383fd225527081)

### 📦️ Dependency Updates

- update rust crate itertools to 0.14 by [@renovate[bot]](https://github.com/renovate[bot]) in [cde45d6](https://github.com/jdx/mise/commit/cde45d63b2d19ae62e0ce2634931d4849e42d3ef)
- update rust crate petgraph to 0.7 by [@renovate[bot]](https://github.com/renovate[bot]) in [b364794](https://github.com/jdx/mise/commit/b36479491729be6cab2c120091dd50d48aa56c50)
- update rust crate self_update to 0.42 by [@renovate[bot]](https://github.com/renovate[bot]) in [c85385d](https://github.com/jdx/mise/commit/c85385dffcb0ad371a2a6e714754e922bd4d53f8)

### Chore

- upgrade expr by [@jdx](https://github.com/jdx) in [c06a415](https://github.com/jdx/mise/commit/c06a41544e2cb09912244efe6a8f5bcc03eb24d7)
- mise up by [@jdx](https://github.com/jdx) in [678f648](https://github.com/jdx/mise/commit/678f6489a9501b32bf3c36771977771d933f2466)
- cargo-show by [@jdx](https://github.com/jdx) in [69d44fd](https://github.com/jdx/mise/commit/69d44fd064d2fdaae08ff9ea3300a42e560630cd)
- remove cargo-show dependency by [@jdx](https://github.com/jdx) in [ab8e9e9](https://github.com/jdx/mise/commit/ab8e9e9e429beeb23731c356537525f64bc59b28)
- remove cargo-show dependency by [@jdx](https://github.com/jdx) in [ca2f89c](https://github.com/jdx/mise/commit/ca2f89c6cd36d828a9eab2884a3f8c9cc1fe2c19)
- remove cargo-show dependency by [@jdx](https://github.com/jdx) in [82e3390](https://github.com/jdx/mise/commit/82e3390c5fc9a97c942dc407b2073edfcb3974bc)
- fix release-plz by [@jdx](https://github.com/jdx) in [52ac62a](https://github.com/jdx/mise/commit/52ac62a7d7e8439d32b84c4247ee366c28901863)
- fix release-plz by [@jdx](https://github.com/jdx) in [dba7044](https://github.com/jdx/mise/commit/dba7044b4dcce808fd4734e9a284ab2174758be0)

### New Contributors

- @nexckycort made their first contribution
- @MontakOleg made their first contribution
- @kit494way made their first contribution
- @benberryallwood made their first contribution
- @xcapaldi made their first contribution
- @auxesis made their first contribution

## [2025.1.0](https://github.com/jdx/mise/compare/v2024.12.24..v2025.1.0) - 2025-01-01

### 🚀 Features

- use aqua for gradle by [@jdx](https://github.com/jdx) in [e2065ac](https://github.com/jdx/mise/commit/e2065acd66386d3f51535b82fb5b925d3cbcadee)
- added completions to more commands by [@jdx](https://github.com/jdx) in [0cfaf74](https://github.com/jdx/mise/commit/0cfaf74aabeef05086b67319359a510bd88fab51)

### 🐛 Bug Fixes

- panic when setting config value by [@roele](https://github.com/roele) in [47aaa3e](https://github.com/jdx/mise/commit/47aaa3e3f5dfbc69d5c28e99956c2347bcd1ffae)
- add hidden settings/task --complete option by [@jdx](https://github.com/jdx) in [432a3e4](https://github.com/jdx/mise/commit/432a3e4cee22ee97d1648d28960e9c5bba94829e)
- handle panic when task contains invalid template by [@jdx](https://github.com/jdx) in [2e0622b](https://github.com/jdx/mise/commit/2e0622b4d50bb3c2a07ba798e2efca6a8240c6fe)
- missing checksums in mise.run script by [@jdx](https://github.com/jdx) in [daae8d0](https://github.com/jdx/mise/commit/daae8d071693879169114bb8bbcf7be115ef5c52)
- active flag for symlinked tools in `mise ls --json` by [@jdx](https://github.com/jdx) in [a31cd43](https://github.com/jdx/mise/commit/a31cd43519e033217b122d87ddc6c265b2d799b6)

### 📚 Documentation

- Update LICENSE by [@jdx](https://github.com/jdx) in [156db11](https://github.com/jdx/mise/commit/156db1130c2757aaaf6e53686148d8b9b0791ae7)
- updated roadmap by [@jdx](https://github.com/jdx) in [f8916d4](https://github.com/jdx/mise/commit/f8916d4cbd09fbbc8142bf25b4d586e146d19a21)

## [2024.12.24](https://github.com/jdx/mise/compare/v2024.12.23..v2024.12.24) - 2024-12-31

### 🐛 Bug Fixes

- switch back to asdf for gradle by [@jdx](https://github.com/jdx) in [cc88dca](https://github.com/jdx/mise/commit/cc88dca50e8e0dac94dbb83d0ce1ebcfc38a1ec4)

### Chore

- add commented out cleanup of old CLIs by [@jdx](https://github.com/jdx) in [bb7e022](https://github.com/jdx/mise/commit/bb7e022240c0e7019a595d093a33b414119e975f)

## [2024.12.23](https://github.com/jdx/mise/compare/v2024.12.22..v2024.12.23) - 2024-12-30

### 🐛 Bug Fixes

- winget release PRs by [@jdx](https://github.com/jdx) in [9dec542](https://github.com/jdx/mise/commit/9dec542188e731ef357fd74339dd08ac005cb9e3)
- mise settings unset does not seem to work by [@roele](https://github.com/roele) in [7489357](https://github.com/jdx/mise/commit/7489357b02d27e8437f8b11c0c185b985733a79b)
- gradle aqua package by [@jdx](https://github.com/jdx) in [50f812c](https://github.com/jdx/mise/commit/50f812c6d19dff8ca25350f8d73934b8502dcaba)
- **breaking** remove `root` env var in tasks by [@jdx](https://github.com/jdx) in [e306720](https://github.com/jdx/mise/commit/e306720c558c3d0c346716cdf2655702efad004e)

### 📚 Documentation

- syntax in `mise watch` by [@jdx](https://github.com/jdx) in [beab480](https://github.com/jdx/mise/commit/beab48029b3e7a91047012b655f3efe4fd722acf)
- Update registry link by [@bmulholland](https://github.com/bmulholland) in [62b7ad4](https://github.com/jdx/mise/commit/62b7ad4b5ab2e6e8ed4bdc012014d556660cbd76)
- clarify shims behaviour by [@syhol](https://github.com/syhol) in [742fdeb](https://github.com/jdx/mise/commit/742fdeb44b0ca9ad95f47625bf9e97531f6ac65d)

### Chore

- remove unused versioned tarballs from mise.jdx.dev by [@jdx](https://github.com/jdx) in [48f1021](https://github.com/jdx/mise/commit/48f1021048646061e7cd85d9f9969946b00962a6)
- trim newline in banner by [@jdx](https://github.com/jdx) in [c8f2c90](https://github.com/jdx/mise/commit/c8f2c90111c5d20fe4586d59eb66f3bb2f8cfd9a)

### New Contributors

- @bmulholland made their first contribution

## [2024.12.22](https://github.com/jdx/mise/compare/v2024.12.21..v2024.12.22) - 2024-12-30

### 🚀 Features

- colorize banner by [@jdx](https://github.com/jdx) in [ad3a5f0](https://github.com/jdx/mise/commit/ad3a5f040013bad046f2ca3abb9eebc941301368)

### 🐛 Bug Fixes

- add `:` escaping for tasks with multiple colons by [@eitamal](https://github.com/eitamal) in [a9b6e00](https://github.com/jdx/mise/commit/a9b6e001f12af9226ac2901ae602e17e1874041e)
- type issue in docs/JSON schema for python_create_args and uv_create_args by [@roele](https://github.com/roele) in [6d462d0](https://github.com/jdx/mise/commit/6d462d002e44e75d00ebf74fc0856124d656ba77)

### 📚 Documentation

- **(settings)** fix link to precompiled python binaries by [@scop](https://github.com/scop) in [a706333](https://github.com/jdx/mise/commit/a706333a355b9f9653c1008210e759db2a1b7754)
- Fix cargo install examples by [@orf](https://github.com/orf) in [6d7e3c5](https://github.com/jdx/mise/commit/6d7e3c56f812d32182df7d909d79ffea361265bf)

### New Contributors

- @orf made their first contribution
- @eitamal made their first contribution

## [2024.12.21](https://github.com/jdx/mise/compare/v2024.12.20..v2024.12.21) - 2024-12-27

### 🐛 Bug Fixes

- **(python)** force precompiled setting warning message syntax by [@scop](https://github.com/scop) in [6d2c94c](https://github.com/jdx/mise/commit/6d2c94c14caf991506190c396b8dfab237fd47ef)
- zstd detection false positive on MacOS by [@roele](https://github.com/roele) in [323a85b](https://github.com/jdx/mise/commit/323a85b6f7632b906cb113a428519f2ba89197e7)

### 📚 Documentation

- fix incorrect examples that were causing 'expected a sequence' error by [@ssbarnea](https://github.com/ssbarnea) in [0568768](https://github.com/jdx/mise/commit/05687682ec44c887a8a5c812b7bfd83a739772b6)

### 📦️ Dependency Updates

- update rust crate ubi to 0.3 by [@renovate[bot]](https://github.com/renovate[bot]) in [a28739b](https://github.com/jdx/mise/commit/a28739b6c0f4a7e831034970e85aeb637c755185)

## [2024.12.20](https://github.com/jdx/mise/compare/v2024.12.19..v2024.12.20) - 2024-12-25

### 🚀 Features

- **(hugo)** add extended registry from aqua and keep only one registry with all aliases by [@kilianpaquier](https://github.com/kilianpaquier) in [2387617](https://github.com/jdx/mise/commit/23876176720b0c7f10d34834f03a02c1dee5af47)
- build erlang with all cores by [@jdx](https://github.com/jdx) in [9b5d32e](https://github.com/jdx/mise/commit/9b5d32e9fb0062de3c79030691ecb56fb3515006)
- Modify install_rubygems_hook to place plugin in site_ruby directory by [@zkhadikov](https://github.com/zkhadikov) in [7a13356](https://github.com/jdx/mise/commit/7a13356d2b03edb36705d13abaf92814124b4b55)

### 🐛 Bug Fixes

- do not require "v" prefix in mise.run by [@jdx](https://github.com/jdx) in [9051db1](https://github.com/jdx/mise/commit/9051db1e8fa0a909aa133d080a06c69a53b49ab8)
- add checksum for macos-x86 by [@jdx](https://github.com/jdx) in [2dbabda](https://github.com/jdx/mise/commit/2dbabda481910ba470cdc2462c425e714b5b59d2)

### 📚 Documentation

- Correct link to aqua registry by [@jesse-c](https://github.com/jesse-c) in [0cea93b](https://github.com/jdx/mise/commit/0cea93b27bc46ba9219557532d6d1d061d05f81d)

### 🧪 Testing

- skip dotnet if not installed by [@jdx](https://github.com/jdx) in [1a663dd](https://github.com/jdx/mise/commit/1a663dd63e17cc08a961b86b5b0b6a1d7e9b2a1f)

### New Contributors

- @zkhadikov made their first contribution
- @kilianpaquier made their first contribution
- @jesse-c made their first contribution

## [2024.12.19](https://github.com/jdx/mise/compare/v2024.12.18..v2024.12.19) - 2024-12-23

### 🚀 Features

- use zstd in mise.run by [@jdx](https://github.com/jdx) in [13c9cb5](https://github.com/jdx/mise/commit/13c9cb589dc3b7fb20905ebf3b63ae44a3a73fc3)
- verify zig with minisign by [@jdx](https://github.com/jdx) in [de66eae](https://github.com/jdx/mise/commit/de66eaef3fffb52a14c95d9b202cfd859c27ade4)

### Chore

- increase tarball compression by [@jdx](https://github.com/jdx) in [a899155](https://github.com/jdx/mise/commit/a8991551bd7c61d1f75a800906d2f718b4bdf7c0)
- use max threads for zstd compression by [@jdx](https://github.com/jdx) in [a3f792a](https://github.com/jdx/mise/commit/a3f792a1eb0a395c7a82a063b96d30282b6343de)
- print all tarball sizes by [@jdx](https://github.com/jdx) in [29fbc04](https://github.com/jdx/mise/commit/29fbc04e52c76b16c9a72385ead4edbfaff984fb)

## [2024.12.18](https://github.com/jdx/mise/compare/v2024.12.17..v2024.12.18) - 2024-12-23

### 🚀 Features

- allow dotnet prerelease by [@acesyde](https://github.com/acesyde) in [6393920](https://github.com/jdx/mise/commit/6393920ed697853b5f3d1fa344dca6b3374de17d)
- added minisign to registry by [@jdx](https://github.com/jdx) in [0a709eb](https://github.com/jdx/mise/commit/0a709eb7c5a14f03ae527436d2c96385934ec336)
- `mise g bootstrap` by [@jdx](https://github.com/jdx) in [0d07383](https://github.com/jdx/mise/commit/0d07383bb934e79874ddff1e4295afbb4b17ae13)
- `mise g bootstrap` by [@jdx](https://github.com/jdx) in [f79ce71](https://github.com/jdx/mise/commit/f79ce719f9121eb6e0e821cf271af306f2a9d6c8)

### 🐛 Bug Fixes

- hide task file extension in completions by [@jdx](https://github.com/jdx) in [53c9144](https://github.com/jdx/mise/commit/53c9144df8ef5d7d75f19c949be6231a05f7405a)
- settings completions by [@jdx](https://github.com/jdx) in [38a7b54](https://github.com/jdx/mise/commit/38a7b54ba4264b52f9ad8a0240d9890af45d436c)

### 📚 Documentation

- update IDE integration page by [@hverlin](https://github.com/hverlin) in [ca07f5e](https://github.com/jdx/mise/commit/ca07f5e9d4d7d71a4f80f6226779ae4f9ad79221)
- add powershell sample by [@acesyde](https://github.com/acesyde) in [d410b62](https://github.com/jdx/mise/commit/d410b62594db73f413be655c5680f54e9f896b73)
- add missing dotnet left menu by [@acesyde](https://github.com/acesyde) in [1d84045](https://github.com/jdx/mise/commit/1d84045e32325dcad0f4bf318e4c9f6f5da4df8c)

### 🧪 Testing

- added stubbed test for https://github.com/jdx/mise/discussions/3783 by [@jdx](https://github.com/jdx) in [f79a3a4](https://github.com/jdx/mise/commit/f79a3a41ebf833d2c49bdc91ae4026c46498d9f7)

### Chore

- add shell to user-agent by [@jdx](https://github.com/jdx) in [7b0ce5e](https://github.com/jdx/mise/commit/7b0ce5ee07510ab525e6f18d20d4de2c0fa88247)
- sign releases with minisign by [@jdx](https://github.com/jdx) in [f87cafe](https://github.com/jdx/mise/commit/f87cafeab53de8e67776d8f187e574925e95c827)
- create minisign secret key by [@jdx](https://github.com/jdx) in [dea4676](https://github.com/jdx/mise/commit/dea4676f53ee4d1a905ae17b004131c6dee3b385)
- create minisign secret key by [@jdx](https://github.com/jdx) in [ecebebe](https://github.com/jdx/mise/commit/ecebebee13cc20773eaefda706bad4e5ac8cc25f)
- fix minisign signing by [@jdx](https://github.com/jdx) in [6401ff8](https://github.com/jdx/mise/commit/6401ff84e0dcbdb890dd037aff6fbcf3edc51af5)
- added install.sh to releases by [@jdx](https://github.com/jdx) in [2946d58](https://github.com/jdx/mise/commit/2946d5864cffb65a1ee1260f3c38070531743854)
- install minisign by [@jdx](https://github.com/jdx) in [f22272c](https://github.com/jdx/mise/commit/f22272c3838fcb8de0365a4022f8aefc00c46f4c)
- use ubuntu-24 for release by [@jdx](https://github.com/jdx) in [40a13f8](https://github.com/jdx/mise/commit/40a13f8e7088ba13762178eccc5eb8438bc9ce6b)
- set minisign pub key by [@jdx](https://github.com/jdx) in [fd6aa1e](https://github.com/jdx/mise/commit/fd6aa1eccf23f97e82ff166ff8950721c236239b)
- age encrypt minisign key by [@jdx](https://github.com/jdx) in [02c30e2](https://github.com/jdx/mise/commit/02c30e2c9167d3f4bf5ac05a82a43bc82b703123)
- apt install age by [@jdx](https://github.com/jdx) in [769a088](https://github.com/jdx/mise/commit/769a08875b3651c3edd63fd4387497ce6b16cd4b)
- switch back to MINISIGN_KEY by [@jdx](https://github.com/jdx) in [66dc8cf](https://github.com/jdx/mise/commit/66dc8cf199adb57c22ac398b3333ba12abaaf106)
- fix minisign signing by [@jdx](https://github.com/jdx) in [a3f8173](https://github.com/jdx/mise/commit/a3f81738bb4ab0827eb6bfae4a1639c29f29da36)
- add zst tarballs by [@jdx](https://github.com/jdx) in [85a1192](https://github.com/jdx/mise/commit/85a1192091b7f37ab7c3712e4100c8b43d587857)
- add zst tarballs by [@jdx](https://github.com/jdx) in [5238124](https://github.com/jdx/mise/commit/5238124dbda89fe32380beab9b64d31cb2cb4ddb)
- add zst tarballs by [@jdx](https://github.com/jdx) in [2a4d0bf](https://github.com/jdx/mise/commit/2a4d0bf0ee78dfe672d97bc763643300516d5a9b)
- add zst tarballs by [@jdx](https://github.com/jdx) in [285d777](https://github.com/jdx/mise/commit/285d777b3f33bfa587070b3d15cd904fc83e111f)
- extract artifact with zstd by [@jdx](https://github.com/jdx) in [ba66d46](https://github.com/jdx/mise/commit/ba66d4659c6d8f3ffa589dacfe402d6988e46d9a)

## [2024.12.17](https://github.com/jdx/mise/compare/v2024.12.16..v2024.12.17) - 2024-12-21

### 🚀 Features

- added a banner to `mise --version` by [@jdx](https://github.com/jdx) in [21b8114](https://github.com/jdx/mise/commit/21b8114b990ac53dc4259cb07370e7e81a83e9d8)
- add usage field to tasks by [@jdx](https://github.com/jdx) in [5d79196](https://github.com/jdx/mise/commit/5d79196da2ae5cf706697fe29d4d32eed6838b00)
- added keep-order task output type by [@jdx](https://github.com/jdx) in [d020c05](https://github.com/jdx/mise/commit/d020c0512a71ef31f4966609dfc4d61046750300)
- `replacing` task output type by [@jdx](https://github.com/jdx) in [3a955d5](https://github.com/jdx/mise/commit/3a955d5ec8919a6f7104108d9476f09c54e822b9)
- added timed task output type by [@jdx](https://github.com/jdx) in [cd0df14](https://github.com/jdx/mise/commit/cd0df148e66edc05ef63b4adc9eca11c381bc5bd)

### 🐛 Bug Fixes

- dotnet backend doc by [@acesyde](https://github.com/acesyde) in [c59fedf](https://github.com/jdx/mise/commit/c59fedf2659211f7cdd4eb0a89af094365b9824d)
- include full env in toolset tera_ctx by [@risu729](https://github.com/risu729) in [69a3fd6](https://github.com/jdx/mise/commit/69a3fd6f680d4fe80e64777355247ca281930393)
- set env vars in task templates by [@jdx](https://github.com/jdx) in [dca96d2](https://github.com/jdx/mise/commit/dca96d2900f3c6cacca79758789d47ec9f5e704b)

### 📚 Documentation

- update mise-action version in tips and tricks by [@scop](https://github.com/scop) in [20d50ff](https://github.com/jdx/mise/commit/20d50ff99c581523448fb843720fc8375ef65696)
- Small cookbooks fixes by [@hverlin](https://github.com/hverlin) in [821be63](https://github.com/jdx/mise/commit/821be633d6db21284de284bb1c4f7f52e78e532d)

### 🧪 Testing

- fix elixir release test by [@jdx](https://github.com/jdx) in [b4f11da](https://github.com/jdx/mise/commit/b4f11dabf7a16a875f9d7ab3ded6a516b481f6f8)
- add some test cases for env var templates by [@jdx](https://github.com/jdx) in [c938977](https://github.com/jdx/mise/commit/c938977ccc265c9530200e0b19bb0cce5f73ddbb)

### Chore

- updated usage by [@jdx](https://github.com/jdx) in [dad7857](https://github.com/jdx/mise/commit/dad785727c80efeb4bf498995ed5237f6cd94d79)

## [2024.12.16](https://github.com/jdx/mise/compare/v2024.12.15..v2024.12.16) - 2024-12-20

### 🚀 Features

- add dotnet backend by [@acesyde](https://github.com/acesyde) in [a8fb788](https://github.com/jdx/mise/commit/a8fb788c68cb1a51fa51cab03a8eb2079cce881a)
- added ignored_config_paths to `mise dr` by [@jdx](https://github.com/jdx) in [4727339](https://github.com/jdx/mise/commit/4727339de10cb1e86fa44260449599d36d28a987)

### 🐛 Bug Fixes

- **(ruby)** fix Ruby plugin to use `ruby_install` option correctly by [@yuhr](https://github.com/yuhr) in [9c77df6](https://github.com/jdx/mise/commit/9c77df6a0c49fd86c24e4b76008abf149a6cd0e8)
- `mise run` shorthand with options by [@jdx](https://github.com/jdx) in [e16e758](https://github.com/jdx/mise/commit/e16e758f6be649541c6c7376bd9a9269aa4c567d)
- zig on windows by [@jdx](https://github.com/jdx) in [b2e328d](https://github.com/jdx/mise/commit/b2e328d8f43bf08cf630e3ef05f3ae2971b11d67)
- allow using previously defined vars by [@jdx](https://github.com/jdx) in [d74f29e](https://github.com/jdx/mise/commit/d74f29e8c4ecb7db1f8f9307fc96d11301a55a13)
- make --help consistent with `mise run` and `mise <task>` by [@jdx](https://github.com/jdx) in [addad3b](https://github.com/jdx/mise/commit/addad3b8ff24dbbfeee51c832ccaf46af6cafa2a)
- use implicit keys for `mise config set` by [@jdx](https://github.com/jdx) in [3d61487](https://github.com/jdx/mise/commit/3d614875a22fcd2200d20f75fcf5b18c65d20bcf)

### 📚 Documentation

- update cookbook by [@hverlin](https://github.com/hverlin) in [4df4933](https://github.com/jdx/mise/commit/4df49338fea1cb1b76894de62c0a6b263c17dce7)
- remove reference to deprecated asdf_compat functionality by [@jdx](https://github.com/jdx) in [03a2afb](https://github.com/jdx/mise/commit/03a2afb4f8c738e3b172d0f5e1ca1465bf1d6a5c)
- describe behavior of `run --output` better by [@jdx](https://github.com/jdx) in [56a88ad](https://github.com/jdx/mise/commit/56a88ade5c441e0d79f6c56a3a7a0d4166109bc4)

### 📦️ Dependency Updates

- update dependency bun to v1.1.40 by [@renovate[bot]](https://github.com/renovate[bot]) in [041d258](https://github.com/jdx/mise/commit/041d258d95a3829749c36ddf218516400d50c009)

### Chore

- lint fix by [@jdx](https://github.com/jdx) in [118b8de](https://github.com/jdx/mise/commit/118b8de645712ff1d78c33b9a2c094a1f92c5b20)
- switch from home -> homedir crate by [@jdx](https://github.com/jdx) in [1f0c01b](https://github.com/jdx/mise/commit/1f0c01b0ec85d88653946205a694abc44bd8d82b)

### New Contributors

- @acesyde made their first contribution
- @ssbarnea made their first contribution
- @yuhr made their first contribution

## [2024.12.15](https://github.com/jdx/mise/compare/v2024.12.14..v2024.12.15) - 2024-12-19

### 🚀 Features

- unnest output when `mise run` is nested by [@jdx](https://github.com/jdx) in [ab902be](https://github.com/jdx/mise/commit/ab902be486920103d18cb92668b094730ee097c0)
- `mise rm` by [@jdx](https://github.com/jdx) in [829f224](https://github.com/jdx/mise/commit/829f22496adc7a9b983bb517565f34ea5aea7649)
- added *:_default task name by [@jdx](https://github.com/jdx) in [d78313e](https://github.com/jdx/mise/commit/d78313ea80b31754ed32dc9d02d8f9925348e351)
- `mise run --continue-on-error by [@jdx](https://github.com/jdx) in [996af34](https://github.com/jdx/mise/commit/996af348460efec4eb3a71f7b940faa086214aaa)
- added .tool-versions -> mise.toml converter by [@jdx](https://github.com/jdx) in [5311de5](https://github.com/jdx/mise/commit/5311de548412d8c4115824662e2f9fe84a5229f7)
- get mise sync python --uv to work by [@jdx](https://github.com/jdx) in [c48b6f3](https://github.com/jdx/mise/commit/c48b6f3f8dc61b0d8cf98a69da32f7bbd1113436)
- `mise install-into` by [@jdx](https://github.com/jdx) in [ccac508](https://github.com/jdx/mise/commit/ccac5081a89c110f9ddc3fc1daf50a91fc80ce0f)
- added `mise dr --json` by [@jdx](https://github.com/jdx) in [bde7bd5](https://github.com/jdx/mise/commit/bde7bd5f02a840a6d302dda342469c76cc3926d7)

### 🐛 Bug Fixes

- retain "os" options in `mise up --bump` by [@jdx](https://github.com/jdx) in [1670ae4](https://github.com/jdx/mise/commit/1670ae48ac8316af8050b7439fb04e7e17704bdf)
- unnest task cmd output by [@jdx](https://github.com/jdx) in [e973b53](https://github.com/jdx/mise/commit/e973b532a63874d731c0ba132281eca16fdc50c0)
- ensure MISE_PROJECT_ROOT is set with no mise.toml by [@jdx](https://github.com/jdx) in [d7f2f6c](https://github.com/jdx/mise/commit/d7f2f6c3c47610c078c70f05aeee98ffaf70c7ab)
- create venv uses absolute tool paths by [@syhol](https://github.com/syhol) in [c501e53](https://github.com/jdx/mise/commit/c501e533abe6a3d2302ed8a3b3b69aaa72d274c5)
- jj repository moved to an organization by [@phyrog](https://github.com/phyrog) in [c3112b2](https://github.com/jdx/mise/commit/c3112b2be5ee8c0c982d836fd9d74c8c2ba566d7)
- disable reverse uv syncing by [@jdx](https://github.com/jdx) in [d2b175e](https://github.com/jdx/mise/commit/d2b175efeadc10217206d1ee4db8a2b6b841a8f0)
- add full tera context to tasks by [@jdx](https://github.com/jdx) in [4cce843](https://github.com/jdx/mise/commit/4cce8438ede57732ec38dd0c5607f06528f2730f)
- powershell warning by [@jdx](https://github.com/jdx) in [b7c8da5](https://github.com/jdx/mise/commit/b7c8da56099708bc58b0b89016faec77dba97239)

### 🚜 Refactor

- **(registry)** use aqua for more tools by [@scop](https://github.com/scop) in [7effb07](https://github.com/jdx/mise/commit/7effb07d89f89d476851b2cd6fcf529c5aaef525)
- **(registry)** use aqua:skaji/relocatable-perl for perl by [@scop](https://github.com/scop) in [3852bdc](https://github.com/jdx/mise/commit/3852bdc896d0f3791ca0b14ede96accd098aaea5)
- switch to std::sync::LazyLock by [@jdx](https://github.com/jdx) in [7b57cf4](https://github.com/jdx/mise/commit/7b57cf48e41a412cf3d342b973be036b529b593f)

### 📚 Documentation

- fix some broken anchor links by [@hverlin](https://github.com/hverlin) in [127e164](https://github.com/jdx/mise/commit/127e1646f80fba36a2b2a1b7956d6d8e77f83c5f)
- note hooks require `mise activate` by [@jdx](https://github.com/jdx) in [211d3d3](https://github.com/jdx/mise/commit/211d3d3b91c52e418a3e25af4a021da93c64ed4d)

### 🧪 Testing

- fix conduit test for new structure by [@jdx](https://github.com/jdx) in [8691331](https://github.com/jdx/mise/commit/86913318f7705e6cabb999970475c958605219d1)

### Chore

- hide non-functioning docker tasks by [@jdx](https://github.com/jdx) in [40fd3f6](https://github.com/jdx/mise/commit/40fd3f60ebde1d549503a6d9927b79b37622b1b0)

### New Contributors

- @highb made their first contribution

## [2024.12.14](https://github.com/jdx/mise/compare/v2024.12.13..v2024.12.14) - 2024-12-18

### 🚀 Features

- **(registry)** Add lazydocker by [@hverlin](https://github.com/hverlin) in [c82f6e9](https://github.com/jdx/mise/commit/c82f6e971e702828b0538f8846f457856e4a3c52)
- **(registry)** Add btop by [@hverlin](https://github.com/hverlin) in [a5112b3](https://github.com/jdx/mise/commit/a5112b3807ff1e66f341013695e279dce6520d1a)
- Allows control of config_root for global config by [@bnorick](https://github.com/bnorick) in [123d4e8](https://github.com/jdx/mise/commit/123d4e83690aa5773e6d0cbb3e7e1fc15832660d)
- allow inserting PATH in env._.source by [@jdx](https://github.com/jdx) in [1072132](https://github.com/jdx/mise/commit/1072132a72b758c5a8ba6330f5849d00ba2fb142)

### 🐛 Bug Fixes

- Can not find the bin files when using python venv on windows by [@NavyD](https://github.com/NavyD) in [a4ab84f](https://github.com/jdx/mise/commit/a4ab84fabc4967ea6c9e6476777f66eda1828e0b)
- render tasks in task files by [@risu729](https://github.com/risu729) in [fd9f6d1](https://github.com/jdx/mise/commit/fd9f6d1ecb279550e50c1c1633f5030526c6c475)
- dont require run script for `task add` by [@jdx](https://github.com/jdx) in [7686af6](https://github.com/jdx/mise/commit/7686af6adfdad4448c56c7466247b7ddc1a599ca)
- auto-trust on `task add` by [@jdx](https://github.com/jdx) in [0f9dae3](https://github.com/jdx/mise/commit/0f9dae377eaa8a527c099310944e75bac60bc3a3)
- completions getting wrapped in quotes by [@jdx](https://github.com/jdx) in [257445c](https://github.com/jdx/mise/commit/257445c687d889b5666fca69bec4681aef5d39fd)
- pass pristine env to tera in final_env by [@risu729](https://github.com/risu729) in [2e62fbe](https://github.com/jdx/mise/commit/2e62fbe05acfeeb00447360712d25b0c988d9e44)
- trap panics in task resolving by [@jdx](https://github.com/jdx) in [9653a49](https://github.com/jdx/mise/commit/9653a49acbd48089607194b5897de163672f76a8)

### 📚 Documentation

- mark new features as experimental by [@syhol](https://github.com/syhol) in [d0f8cd3](https://github.com/jdx/mise/commit/d0f8cd38d02017f8673c80dc933eda2dd615b5c3)

### 🧪 Testing

- add test cases for venv templates by [@jdx](https://github.com/jdx) in [1cf5b49](https://github.com/jdx/mise/commit/1cf5b49b4efa8f9db3e98690ae8cd0937b34b96b)

### New Contributors

- @NavyD made their first contribution

## [2024.12.13](https://github.com/jdx/mise/compare/v2024.12.12..v2024.12.13) - 2024-12-17

### 🚀 Features

- `mise task add` by [@jdx](https://github.com/jdx) in [8d5214d](https://github.com/jdx/mise/commit/8d5214d4606434c26d6f01eeecc50d1496e7d963)
- elixir core tool by [@jdx](https://github.com/jdx) in [47b28bc](https://github.com/jdx/mise/commit/47b28bc6559abc7869f75c527aaea1b9f8c78c59)
- elixir on windows by [@jdx](https://github.com/jdx) in [3d7521b](https://github.com/jdx/mise/commit/3d7521bbd1ce53883c7226732780d6a0dd17bc40)
- added install_env tool option by [@jdx](https://github.com/jdx) in [9e4fd90](https://github.com/jdx/mise/commit/9e4fd9009df8250aee6f020d9e90a29f407e488f)
- Add Powershell support by [@fgilcc](https://github.com/fgilcc) in [e84c9d1](https://github.com/jdx/mise/commit/e84c9d1061dcd835e2384b1d62ecb08f5a726590)
- improve redactions by [@jdx](https://github.com/jdx) in [4ed4f02](https://github.com/jdx/mise/commit/4ed4f02ca99200175a020acb3e8c144181caeeb7)

### 🐛 Bug Fixes

- run venv after tools are loaded by [@jdx](https://github.com/jdx) in [4d4f9ff](https://github.com/jdx/mise/commit/4d4f9ff9dfe7ac72dc44a9741c6799d8949e889e)
- some improvements to `mise fmt` by [@jdx](https://github.com/jdx) in [9edf8ef](https://github.com/jdx/mise/commit/9edf8efdf5c5350a3b3c63da8b114edc1e64a083)
- always run postinstall hook by [@jdx](https://github.com/jdx) in [247d0d3](https://github.com/jdx/mise/commit/247d0d34af75b158e6a009e44fab31ee8e0c7866)
- move bat from aqua to ubi by [@jdx](https://github.com/jdx) in [60d0c79](https://github.com/jdx/mise/commit/60d0c798f695199bdc81f8beec737f0e2a8589e0)
- do not require version for `mise sh --unset` by [@jdx](https://github.com/jdx) in [34fe37f](https://github.com/jdx/mise/commit/34fe37f5da01b22748fe1fe22163535e9d662d5d)
- back nomad with nomad, not levant by [@rliebz](https://github.com/rliebz) in [8809852](https://github.com/jdx/mise/commit/8809852f2805e75df2d73b7311927143e14ca584)
- correct python precompiled urls for freebsd by [@jdx](https://github.com/jdx) in [b0fa5e9](https://github.com/jdx/mise/commit/b0fa5e91cd6a87838a145a64e24b868b4c15b947)
- bug fixes with tools=true in env by [@jdx](https://github.com/jdx) in [1aa3670](https://github.com/jdx/mise/commit/1aa367089ba9013959616e66e2f5822f1e7a57b0)
- sort keys in `__MISE_DIFF` to make the serialised value deterministic by [@joshbode](https://github.com/joshbode) in [2aabb5c](https://github.com/jdx/mise/commit/2aabb5c6f8a44a7b3d986540cf129e286552b9b0)
- resolve config_root for dir tasks option by [@risu729](https://github.com/risu729) in [ec9e03f](https://github.com/jdx/mise/commit/ec9e03f95826d086f31039d4e11a2ea002f5c4c2)

### 📚 Documentation

- add getting-started carousel by [@hverlin](https://github.com/hverlin) in [949a3fc](https://github.com/jdx/mise/commit/949a3fc1b78ce82e129a1aa92fd5baba88507073)
- Fix Sops URL by [@matthew-snyder](https://github.com/matthew-snyder) in [8d87eda](https://github.com/jdx/mise/commit/8d87eda31739f24a17d591c71045d5c587d24cde)
- add elixir to sidebar by [@risu729](https://github.com/risu729) in [a4ab246](https://github.com/jdx/mise/commit/a4ab2464932b4e21e183c87a28b8a47349ee6933)
- update task documentation by [@hverlin](https://github.com/hverlin) in [903563d](https://github.com/jdx/mise/commit/903563d6eb4229d16c03fd239991ec0752cdb91c)

### Chore

- format toml with taplo by [@jdx](https://github.com/jdx) in [b2c199a](https://github.com/jdx/mise/commit/b2c199a6409f874f74e9a379e8c6149f43ddf25b)
- add platform field to registry backends by [@jdx](https://github.com/jdx) in [4938b0b](https://github.com/jdx/mise/commit/4938b0b7bebd84500adc6586c6fd688db6db0204)

### New Contributors

- @fgilcc made their first contribution
- @rliebz made their first contribution
- @matthew-snyder made their first contribution

## [2024.12.12](https://github.com/jdx/mise/compare/v2024.12.11..v2024.12.12) - 2024-12-16

### 🚀 Features

- Add upx,actionlint and correct ripsecret error by [@boris-smidt-klarrio](https://github.com/boris-smidt-klarrio) in [c3479d2](https://github.com/jdx/mise/commit/c3479d2854190f7b3e98d0c1ba77a561fbdc113e)
- aqua:argo-cd by [@boris-smidt-klarrio](https://github.com/boris-smidt-klarrio) in [728bd02](https://github.com/jdx/mise/commit/728bd0211e3e90cef8f23ccb48e4fd87911dec6a)
- task tools by [@jdx](https://github.com/jdx) in [420d31d](https://github.com/jdx/mise/commit/420d31dd6561f5bd4036d61cc81362d554f88149)
- lazy env eval by [@jdx](https://github.com/jdx) in [ca33515](https://github.com/jdx/mise/commit/ca33515f3e9d66edab9cd3764e1ca15808a31442)
- added cache feature to templates by [@jdx](https://github.com/jdx) in [48c95c6](https://github.com/jdx/mise/commit/48c95c6527d231036d26f4114a6e9d1cb652132d)

### 🐛 Bug Fixes

- added MISE_SOPS_ROPS setting by [@jdx](https://github.com/jdx) in [234975c](https://github.com/jdx/mise/commit/234975c43af04593ab1725225be1619e872e5794)
- respect CLICOLOR_FORCE by [@jdx](https://github.com/jdx) in [4923b80](https://github.com/jdx/mise/commit/4923b80801dd4761c103a08a56f3a453a4038501)
- only create 1 venv by [@jdx](https://github.com/jdx) in [7efdb9f](https://github.com/jdx/mise/commit/7efdb9fc11c3ecc4f5234fd9528eea51162375f2)
- set bash --noprofile for env._.source by [@jdx](https://github.com/jdx) in [52d13fe](https://github.com/jdx/mise/commit/52d13feedd5fe256894050bd3ab84c4f23f3b1c5)

### 📚 Documentation

- improve settings a bit by [@jdx](https://github.com/jdx) in [d53d011](https://github.com/jdx/mise/commit/d53d01195e88e82d9a88a410e8feb991c1e8179d)
- Install on Windows - Update doc on install on Windows with Scoop and WinGet + fix NOTE section by [@o-l-a-v](https://github.com/o-l-a-v) in [3eb8fae](https://github.com/jdx/mise/commit/3eb8faef7558b72c43141b2f9135030179f96f50)
- remove note about winget by [@jdx](https://github.com/jdx) in [9c0c1ce](https://github.com/jdx/mise/commit/9c0c1ce943c6fb54ca049d6cdfb81c1122987d05)

### Chore

- disable automatic cargo up on release by [@jdx](https://github.com/jdx) in [3f0d91a](https://github.com/jdx/mise/commit/3f0d91a40928df8ed10cef1837730d8c3a15efea)

### New Contributors

- @o-l-a-v made their first contribution

## [2024.12.11](https://github.com/jdx/mise/compare/v2024.12.10..v2024.12.11) - 2024-12-15

### 🚀 Features

- added selector for `mise use` with no args by [@jdx](https://github.com/jdx) in [9e3179e](https://github.com/jdx/mise/commit/9e3179e3bba81d3fa631ed620f2551b52b26b4f4)
- added tool descriptions by [@jdx](https://github.com/jdx) in [4998ea6](https://github.com/jdx/mise/commit/4998ea66cd6f9fc2c71bd26ec2a3ce685904fd99)
- added `mise sync python --uv` by [@jdx](https://github.com/jdx) in [30d9e19](https://github.com/jdx/mise/commit/30d9e19eedfa948c1678c3d965825f47f1a40e4a)
- `sync ruby --brew` by [@jdx](https://github.com/jdx) in [2388dce](https://github.com/jdx/mise/commit/2388dceeafa2febc0f4f50692d959875455eef22)
- encrypted configs by [@jdx](https://github.com/jdx) in [04d23e7](https://github.com/jdx/mise/commit/04d23e791c7df85af0a20a61f1327dca9dc801cf)
- added `mise --no-config` by [@jdx](https://github.com/jdx) in [b35a109](https://github.com/jdx/mise/commit/b35a109c4062b60dbc55df1586d1299d04524057)
- allow _.file in vars by [@jdx](https://github.com/jdx) in [0a3a25e](https://github.com/jdx/mise/commit/0a3a25eebc0df2ac09f49bdf429980a648a16360)

### 🐛 Bug Fixes

- **(python)** reduce network usage for python precompiled manifests by [@jdx](https://github.com/jdx) in [7a3b414](https://github.com/jdx/mise/commit/7a3b414e2e1ac4ae1f141c4d3e6601405a0ea389)
- **(python)** check only if first or specified python is installed for _.venv by [@jdx](https://github.com/jdx) in [8e4b4d1](https://github.com/jdx/mise/commit/8e4b4d1b8a7853b038251a9f6d5ba67b4ece0ec9)
- **(swift)** prevent swift from using linux platforms that are not available by [@jdx](https://github.com/jdx) in [ca33329](https://github.com/jdx/mise/commit/ca33329faec074e9235dba9935f6cc1a40984021)
- correct headers on `mise ls` by [@jdx](https://github.com/jdx) in [5af3b17](https://github.com/jdx/mise/commit/5af3b17a41decd2d7368f5985f2cb5d3e3b341e8)
- correct message truncation in `mise run` by [@jdx](https://github.com/jdx) in [c668857](https://github.com/jdx/mise/commit/c6688571cfb0eca70a55377b70ec6b9cd0cb6a68)
- include uv in path for hook-env by [@jdx](https://github.com/jdx) in [cf3a20e](https://github.com/jdx/mise/commit/cf3a20e8972345a608e5327a486b2e8a53b8e588)
- correct subtitle in `mise use` selector by [@jdx](https://github.com/jdx) in [4be6d79](https://github.com/jdx/mise/commit/4be6d798f9398f9e072d4067a56e134463e71b41)
- some bugs with status.show_tools and status.show_env by [@jdx](https://github.com/jdx) in [0e648a9](https://github.com/jdx/mise/commit/0e648a993a788f7a5eb3013ce941e990779d290d)
- use task.display_name for `mise run` by [@jdx](https://github.com/jdx) in [a009de1](https://github.com/jdx/mise/commit/a009de13ffa4319de89b0fcaf1ba54ae2524a9b6)
- path is treated differently in nushell by [@samuelallan72](https://github.com/samuelallan72) in [59ac1d5](https://github.com/jdx/mise/commit/59ac1d50b978964b503ab6d33243cb54fdcbd7fe)
- allow number/bool in .env.json by [@jdx](https://github.com/jdx) in [cc35617](https://github.com/jdx/mise/commit/cc356179c0a62fe234bd301a12410366630382bc)

### 🚜 Refactor

- break up env_directive by [@jdx](https://github.com/jdx) in [78c41b6](https://github.com/jdx/mise/commit/78c41b6955ca5a764fd83f666738bed1a7a7598d)

### 📚 Documentation

- better warning when venv auto create is skipped by [@syhol](https://github.com/syhol) in [ec3853f](https://github.com/jdx/mise/commit/ec3853fecf3fabcdd5a9d1ebccdfa74b05dc7e4d)
- added rendered go settings by [@jdx](https://github.com/jdx) in [b41c3dd](https://github.com/jdx/mise/commit/b41c3dd8cfd97f97352900a9d856194185347e8d)

### New Contributors

- @fhalim made their first contribution

## [2024.12.10](https://github.com/jdx/mise/compare/v2024.12.9..v2024.12.10) - 2024-12-14

### 🚀 Features

- **(python)** add other indygreg flavors by [@jdx](https://github.com/jdx) in [f003b51](https://github.com/jdx/mise/commit/f003b51be9da6aa7c1f8e7fcc4a213ea2ffe56c4)
- redactions by [@jdx](https://github.com/jdx) in [3bc3f41](https://github.com/jdx/mise/commit/3bc3f41944b10627c3449a27cf28cd38d51fb216)
- show unload messages/run leave hook by [@jdx](https://github.com/jdx) in [ca453f4](https://github.com/jdx/mise/commit/ca453f4fb2db11f95cb09a4bb9c7acc23572899a)
- update demand and default `mise run` to filtering by [@jdx](https://github.com/jdx) in [48c366d](https://github.com/jdx/mise/commit/48c366d4d2256f6b12aabcbe82abe429622b120e)

### 🐛 Bug Fixes

- **(go)** only use "v" prefix if version is semver-like by [@jdx](https://github.com/jdx) in [6fa612d](https://github.com/jdx/mise/commit/6fa612d51679176ce7832b4ead4e816e774f3961)
- **(go)** fix non-v installs by [@jdx](https://github.com/jdx) in [36e7631](https://github.com/jdx/mise/commit/36e7631e26445f9f2bc34fd09a93ba9a15363c98)
- disable libgit2 for updating plugin repos for now by [@jdx](https://github.com/jdx) in [f0c4d7f](https://github.com/jdx/mise/commit/f0c4d7f25bce4a000ff99e5c2a7cde0562780de1)
- rename kubelogin to azure-kubelogin and add replace it with more popular kubelogin cli by [@jdx](https://github.com/jdx) in [9cf3c1c](https://github.com/jdx/mise/commit/9cf3c1cfba78a211799c5ac7bd8a5def42ac4a60)
- add backend to lockfile by [@jdx](https://github.com/jdx) in [b13f27e](https://github.com/jdx/mise/commit/b13f27e51a775a91e41e57c389ceaf1895e329c2)
- parse task env vars as templates by [@jdx](https://github.com/jdx) in [6a3dfe5](https://github.com/jdx/mise/commit/6a3dfe58b02edfd104d54a7756f5cfca37abe72f)
- do not add ignore file if not tty by [@jdx](https://github.com/jdx) in [ff1c65f](https://github.com/jdx/mise/commit/ff1c65fd3fb06eb21cb6fcc735749e5aeacee9a7)
- improve output of `mise tasks` by [@jdx](https://github.com/jdx) in [00f2648](https://github.com/jdx/mise/commit/00f2648e85630ca33646ea5e944fc1e9f137eb53)

### 📚 Documentation

- add installation via zinit by [@Finkregh](https://github.com/Finkregh) in [14e1494](https://github.com/jdx/mise/commit/14e1494c42b6142665948d274f52224cacac9b1c)

### Chore

- added comfy-table by [@jdx](https://github.com/jdx) in [a690ed8](https://github.com/jdx/mise/commit/a690ed8487acb3e548a8c5603f94caf94bc91d4b)
- pitchfork by [@jdx](https://github.com/jdx) in [2c47f72](https://github.com/jdx/mise/commit/2c47f721c03e8fed57a8ae5ed2f63a0649ffaa9b)
- updated usage by [@jdx](https://github.com/jdx) in [9b31915](https://github.com/jdx/mise/commit/9b31915666b2b5b5b1636ef53f12cebabf392e0e)
- added install-dev task by [@jdx](https://github.com/jdx) in [0c351a8](https://github.com/jdx/mise/commit/0c351a83d952cff8b953fd5c244698a14d74c305)

### New Contributors

- @Finkregh made their first contribution

## [2024.12.9](https://github.com/jdx/mise/compare/v2024.12.8..v2024.12.9) - 2024-12-14

### 🚀 Features

- **(tasks)** optional automatic outputs by [@jdx](https://github.com/jdx) in [9ab0674](https://github.com/jdx/mise/commit/9ab06742b238ae486646622fa141c8776bf8d372)
- added quiet field to tasks by [@jdx](https://github.com/jdx) in [ba51949](https://github.com/jdx/mise/commit/ba51949b518daaabdc0637ea0b081c24f0553096)
- show instructions for updating when min_version does not match by [@jdx](https://github.com/jdx) in [284d676](https://github.com/jdx/mise/commit/284d676fdac52de41a0165730ac99d60f4ccd7ee)
- several enhancements to tasks by [@jdx](https://github.com/jdx) in [6861587](https://github.com/jdx/mise/commit/686158747aa2469248caa98cc5cf1007407de853)

### 🐛 Bug Fixes

- make bash_completions lib optional by [@jdx](https://github.com/jdx) in [1984e66](https://github.com/jdx/mise/commit/1984e66564a8a1473240df669be360b8c7205bd5)
- make plugin update work with libgit2 by [@jdx](https://github.com/jdx) in [dd01ac1](https://github.com/jdx/mise/commit/dd01ac19102cab67a7f33fb4633712af9293978c)
- bug with `mise task edit` and new tasks by [@jdx](https://github.com/jdx) in [455bad9](https://github.com/jdx/mise/commit/455bad9b8d9ed59d0e6520ad2e2d2c8108fbbcb8)
- correct self-update message by [@jdx](https://github.com/jdx) in [eff0cff](https://github.com/jdx/mise/commit/eff0cffca079ee58fc2297396604b96e0253c324)
- task source bug fixes by [@jdx](https://github.com/jdx) in [c07fd8d](https://github.com/jdx/mise/commit/c07fd8d3e536fbbfd1f5464663af9f986c5141b3)

### 📚 Documentation

- add explanation about shebang by [@hverlin](https://github.com/hverlin) in [3dc6f83](https://github.com/jdx/mise/commit/3dc6f83af975165d1af034c4ecebe3570cf0a71e)
- add vitepress-plugin-group-icons by [@hverlin](https://github.com/hverlin) in [df1f69a](https://github.com/jdx/mise/commit/df1f69a4c49911bf040e28cbc4741f0d7be59f68)

### 🧪 Testing

- pin swift version by [@jdx](https://github.com/jdx) in [2b966a4](https://github.com/jdx/mise/commit/2b966a4945851b35be593182527bd40a80279fe4)
- skip firebase by [@jdx](https://github.com/jdx) in [e5714bc](https://github.com/jdx/mise/commit/e5714bcfe9cd45f173aecefcbd3c95fbeab83417)

### 📦️ Dependency Updates

- update rust crate bzip2 to 0.5 by [@renovate[bot]](https://github.com/renovate[bot]) in [d4b1a86](https://github.com/jdx/mise/commit/d4b1a86fa23d845bb8f4273a94f11c63faf50edf)

## [2024.12.8](https://github.com/jdx/mise/compare/v2024.12.7..v2024.12.8) - 2024-12-12

### 🚀 Features

- **(registry)** use pipx for pdm by [@risu729](https://github.com/risu729) in [8f43852](https://github.com/jdx/mise/commit/8f43852b5ff15fcae349fef90885175d12100d50)
- added pitchfork by [@jdx](https://github.com/jdx) in [bac731e](https://github.com/jdx/mise/commit/bac731e47f00245ce13e7eec5716509704519d71)

### 🐛 Bug Fixes

- Adds support for multi-use args by [@bnorick](https://github.com/bnorick) in [beb255c](https://github.com/jdx/mise/commit/beb255c6645b4b16335cae805849ddf2d8de6c6e)
- make task completion script POSIX by [@jdx](https://github.com/jdx) in [b92b560](https://github.com/jdx/mise/commit/b92b5603bb23d55b58e7ee8effe8d6293036c5a9)

### 📚 Documentation

- Add more examples for toml tasks by [@hverlin](https://github.com/hverlin) in [8551448](https://github.com/jdx/mise/commit/85514486f46d02d1f41aadf59ff14433f08ed338)

### Chore

- use main branch for winget by [@jdx](https://github.com/jdx) in [b4036cf](https://github.com/jdx/mise/commit/b4036cf0d10f6ccd8758b0bebc341963c8777d2e)

### New Contributors

- @bnorick made their first contribution
- @biggusbeetus made their first contribution

## [2024.12.7](https://github.com/jdx/mise/compare/v2024.12.6..v2024.12.7) - 2024-12-12

### 🚀 Features

- add the users PATH to `mise doctor` by [@syhol](https://github.com/syhol) in [5d57bd2](https://github.com/jdx/mise/commit/5d57bd2dc6ebecc8bbf8fc290c3c707c353414b3)
- feat : Add superfile with aqua backend to registery by [@yodatak](https://github.com/yodatak) in [f808d12](https://github.com/jdx/mise/commit/f808d1276ea202936c57f6c3a0b6234fdc8a3ae4)
- added `task_auto_install` setting by [@jdx](https://github.com/jdx) in [0f80777](https://github.com/jdx/mise/commit/0f80777606e36f85bf8368b3acc26aa67ba9d12d)
- Add yazi with aqua backend to registery by [@yodatak](https://github.com/yodatak) in [a4540c6](https://github.com/jdx/mise/commit/a4540c6b07d5515fd59e63f79bfe7f7af7285e27)
- Migrating Terragrunt asdf plugin over to gruntwork-io by [@yhakbar](https://github.com/yhakbar) in [96f5bcf](https://github.com/jdx/mise/commit/96f5bcf746207091ea086b774a4f72fdfe1c60e6)
- add settings for python venv creation by [@jdx](https://github.com/jdx) in [ed23343](https://github.com/jdx/mise/commit/ed23343d1c5cc711e3c0ff6ed551546b751badbd)
- added MISE_ARCH setting by [@jdx](https://github.com/jdx) in [59b6788](https://github.com/jdx/mise/commit/59b67886aecf684199e2f948de6753380ec14b64)
- add jj to registry by [@phyrog](https://github.com/phyrog) in [08a4d94](https://github.com/jdx/mise/commit/08a4d9401aa3c7035452af8707a9346877d8d964)
- add task descriptions to completions by [@jdx](https://github.com/jdx) in [224a0b5](https://github.com/jdx/mise/commit/224a0b523ba63f54b5eaf89a1fc5722188ce3f99)

### 🐛 Bug Fixes

- mise upgrade with rust by [@jdx](https://github.com/jdx) in [51d39e6](https://github.com/jdx/mise/commit/51d39e67de9fb0d97d44343c85161e151865be36)
- improve arg parsing for mise watch by [@jdx](https://github.com/jdx) in [b199c66](https://github.com/jdx/mise/commit/b199c66bcac690bfacdebab9e157c73440edbde2)
- skip reading ignored config dirs by [@jdx](https://github.com/jdx) in [6e02eab](https://github.com/jdx/mise/commit/6e02eab9bad8e5c37b8fbc8a29dcfadfd5bf8eb6)
- deprecated attribute in json schema by [@jdx](https://github.com/jdx) in [960fbd5](https://github.com/jdx/mise/commit/960fbd5ba697e813ad43180baaadfc3a24fcdd33)
- simplify auto_install settings by [@jdx](https://github.com/jdx) in [9f8d213](https://github.com/jdx/mise/commit/9f8d213cd253cde7035f6b9509ced723f966cceb)
- use config_root for env._.source by [@jdx](https://github.com/jdx) in [c6351da](https://github.com/jdx/mise/commit/c6351dafca00dee1457f8d063c6cc65f2086887a)
- allow directories as task source by [@jdx](https://github.com/jdx) in [a4b6316](https://github.com/jdx/mise/commit/a4b6316626123e7cb8ceb72fb9f4ec05ccf8ce07)
- Use arguments for to pass staged filenames to pre-commit task by [@joshbode](https://github.com/joshbode) in [d5781be](https://github.com/jdx/mise/commit/d5781beedf06c75c3e977e3a0ee26c2d7f80958e)

### 📚 Documentation

- updated `mise watch` docs to drop the `-t` by [@jdx](https://github.com/jdx) in [8ea6226](https://github.com/jdx/mise/commit/8ea622688cb01a0a0a2805692b38a4a7f1340ce5)

### Chore

- move debug log to trace by [@jdx](https://github.com/jdx) in [5c6c884](https://github.com/jdx/mise/commit/5c6c884cf51e704d1c8c347790ec30b30b0f401e)

### New Contributors

- @yhakbar made their first contribution

## [2024.12.6](https://github.com/jdx/mise/compare/v2024.12.5..v2024.12.6) - 2024-12-11

### 🚀 Features

- added descriptions to `mise run` by [@jdx](https://github.com/jdx) in [abdb02c](https://github.com/jdx/mise/commit/abdb02c4708550d3eb1183718d5cd37005289e69)
- `mise format` by [@jdx](https://github.com/jdx) in [6c28f1f](https://github.com/jdx/mise/commit/6c28f1f7525ff5d4758e0492be5908acfd556529)
- `mise fmt` (renamed from `mise format`) by [@jdx](https://github.com/jdx) in [e429a6e](https://github.com/jdx/mise/commit/e429a6e498c5e7ec6eef6833d8d7c1eff8340ed3)
- `mise format` by [@jdx](https://github.com/jdx) in [d18b040](https://github.com/jdx/mise/commit/d18b040b8ae8eea16ed98b7f7b884a6f52797edc)

### 🐛 Bug Fixes

- **(swift)** remove clang bins by [@jdx](https://github.com/jdx) in [58cd804](https://github.com/jdx/mise/commit/58cd8047a5cb3ce22ef32bb9fbd69d8dc9d43d1b)
- use 7zip for windows zip by [@jdx](https://github.com/jdx) in [475ae62](https://github.com/jdx/mise/commit/475ae62d209795cf8fe9cc846f258755e1092918)
- disable filtering by default on `mise run` by [@jdx](https://github.com/jdx) in [507ee27](https://github.com/jdx/mise/commit/507ee27a736b8cd57714a8365fc88855edf62507)
- deprecate direnv integration by [@jdx](https://github.com/jdx) in [816d6ee](https://github.com/jdx/mise/commit/816d6eec2c04302b37a8a4c9032bcfe344d80d5a)
- remove hidden commands from docs by [@jdx](https://github.com/jdx) in [42a9a05](https://github.com/jdx/mise/commit/42a9a0567fbd8ef61550cf2bfe956074777c7d76)
- improve hook-env by [@jdx](https://github.com/jdx) in [15fa19e](https://github.com/jdx/mise/commit/15fa19e1660f0e8e81c2dab5cc6cdd16a3584690)
- deprecate @system versions by [@jdx](https://github.com/jdx) in [f4d0744](https://github.com/jdx/mise/commit/f4d07440b4417aa2f2b4d09eff29bd4e3939e888)
- do not reuse local tool options for `mise use -g` by [@jdx](https://github.com/jdx) in [48c08c3](https://github.com/jdx/mise/commit/48c08c36cfbfc962880a68a2a6bcf294c8a3c628)
- allow "~" in python.default_packages_file by [@jdx](https://github.com/jdx) in [1c59fcb](https://github.com/jdx/mise/commit/1c59fcb842cea71dc30169efafb161bc57f005ff)
- read all config files for `mise set` by [@jdx](https://github.com/jdx) in [c6b7e51](https://github.com/jdx/mise/commit/c6b7e512097e08920b37259cf986bcd79154b257)

### 📚 Documentation

- fixing elvish install instructions by [@ejrichards](https://github.com/ejrichards) in [a3999eb](https://github.com/jdx/mise/commit/a3999eb4fee8dba6d09e12bb905cdbd978d05d30)
- remove bad formatting in setting by [@jdx](https://github.com/jdx) in [f33813b](https://github.com/jdx/mise/commit/f33813bde40cf65e946a3c1773a4275fce3cb0ef)
- added external links by [@jdx](https://github.com/jdx) in [8271e7b](https://github.com/jdx/mise/commit/8271e7ba0fa8628279cff0460715ec9c80a1c6bd)

### Chore

- fix windows zip structure by [@jdx](https://github.com/jdx) in [195039f](https://github.com/jdx/mise/commit/195039ff2bbe702c7e80ace3fcaeb95cb02d018b)

### New Contributors

- @ejrichards made their first contribution

## [2024.12.5](https://github.com/jdx/mise/compare/v2024.12.4..v2024.12.5) - 2024-12-10

### 🚀 Features

- make `mise trust` act on directories instead of files by [@jdx](https://github.com/jdx) in [7556fb0](https://github.com/jdx/mise/commit/7556fb0c5a4d144dd3afd614e3df0a8373ac5f62)

### 🐛 Bug Fixes

- correctly lowercase "zsh" for shell hooks by [@jdx](https://github.com/jdx) in [035ae59](https://github.com/jdx/mise/commit/035ae59bd898a16be4fcd55b708ae8ba620c60fe)
- read MISE_CONFIG_DIR/conf.d/*.toml configs by [@jdx](https://github.com/jdx) in [5e86f7a](https://github.com/jdx/mise/commit/5e86f7aabb87344e40ca7957e9ca3a5a97a0f981)
- retains spm artifacts by [@jdx](https://github.com/jdx) in [9fa728e](https://github.com/jdx/mise/commit/9fa728e8008ccbf032038afc9d40b7fc450e7910)
- add env var for MISE_NPM_BUN setting by [@jdx](https://github.com/jdx) in [b3c57e2](https://github.com/jdx/mise/commit/b3c57e29bd26d772e2f708351a3c61bf04ee3d65)
- hide hidden tasks in `mise run` selector UI by [@jdx](https://github.com/jdx) in [6e7c265](https://github.com/jdx/mise/commit/6e7c265d700c360194ce81c8ee40425ad6cea5a1)
- trim run scripts whitespace by [@jdx](https://github.com/jdx) in [facc513](https://github.com/jdx/mise/commit/facc5132080d166be4d5d3416336bef169dd3e8f)
- shell-escape arg() in tasks by [@jdx](https://github.com/jdx) in [d21212a](https://github.com/jdx/mise/commit/d21212aed5afc0cbf0e3c5fcac9defe434298e76)
- use shebang in run script to determine how arg escaping should work by [@jdx](https://github.com/jdx) in [2f70a65](https://github.com/jdx/mise/commit/2f70a65ace4bc90b10df845b40eb01dcab17eb31)

### 📚 Documentation

- example with required version by [@felixhummel](https://github.com/felixhummel) in [5d17c34](https://github.com/jdx/mise/commit/5d17c3472fbe5f68489d92ec64fa3fc9bddb9c1c)
- document new windows installers by [@jdx](https://github.com/jdx) in [8f40147](https://github.com/jdx/mise/commit/8f40147972606bd45f3696f5ea50feeeb238bff0)

### Chore

- added winget workflow by [@jdx](https://github.com/jdx) in [901e048](https://github.com/jdx/mise/commit/901e04865842f765188dd687584f9120ad4e5519)

### New Contributors

- @felixhummel made their first contribution

## [2024.12.4](https://github.com/jdx/mise/compare/v2024.12.3..v2024.12.4) - 2024-12-09

### 🚀 Features

- add staged files to `mise generate git-pre-commit` by [@jdx](https://github.com/jdx) in [607e98b](https://github.com/jdx/mise/commit/607e98ba6dbaaa827f5c49b55788367f11b4a81e)
- shell hooks by [@jdx](https://github.com/jdx) in [d6c2cd9](https://github.com/jdx/mise/commit/d6c2cd9091a68ffb0ce9eece35db870535584311)
- added cowsay by [@jdx](https://github.com/jdx) in [92dfc02](https://github.com/jdx/mise/commit/92dfc022f4c6ac662301be3e92d0e92f14116fc9)
- add openbao by [@phyrog](https://github.com/phyrog) in [a902347](https://github.com/jdx/mise/commit/a902347daef9b63b4560f020fa3b387f7225f9a0)
- add gocryptfs by [@phyrog](https://github.com/phyrog) in [3c52f45](https://github.com/jdx/mise/commit/3c52f454409e937a57bce5b1fdb52d6d50eca486)
- use aqua for flyctl by [@jdx](https://github.com/jdx) in [f7ed363](https://github.com/jdx/mise/commit/f7ed363b3eebb82e6242061e78f9ebfdf050d154)

### 🐛 Bug Fixes

- do not set debug mode when calling `mise -v` by [@jdx](https://github.com/jdx) in [47cce06](https://github.com/jdx/mise/commit/47cce069b0f482a5e56b51645fe0c1851c4f2d43)
- issue with usage and arg completions by [@jdx](https://github.com/jdx) in [c627a20](https://github.com/jdx/mise/commit/c627a206ffb03e060e3a30f52cb00009b377251c)

### 📚 Documentation

- Small documentation improvements by [@hverlin](https://github.com/hverlin) in [528f29f](https://github.com/jdx/mise/commit/528f29fa91d1e149b86d066194425d948593c4eb)
- updated demo.gif by [@jdx](https://github.com/jdx) in [979fe07](https://github.com/jdx/mise/commit/979fe07a2a390d9381a9f7701aa289a90f2f32c4)

### Build

- update default.nix by [@minhtrancccp](https://github.com/minhtrancccp) in [aeec038](https://github.com/jdx/mise/commit/aeec0385dfa39935071f3b3ccdd089e31b1086d1)

### New Contributors

- @will-ockmore made their first contribution
- @minhtrancccp made their first contribution
- @phyrog made their first contribution

## [2024.12.3](https://github.com/jdx/mise/compare/v2024.12.2..v2024.12.3) - 2024-12-08

### 🚀 Features

- add danger-swift by [@msnazarow](https://github.com/msnazarow) in [a37580e](https://github.com/jdx/mise/commit/a37580e36c6e2e721bc22dc54ee793603f560c48)

### 📚 Documentation

- **(backend)** fix git url syntax example by [@risu729](https://github.com/risu729) in [d7fc2f1](https://github.com/jdx/mise/commit/d7fc2f11e111e7dae576b57c2ad92a04d08afe2f)
- update dev-tools overview documentation by [@hverlin](https://github.com/hverlin) in [7b45f19](https://github.com/jdx/mise/commit/7b45f192157fc0cd14d33b6f71fb895594f4e641)

### ⚡ Performance

- increase performance of watch_files by [@jdx](https://github.com/jdx) in [cfda918](https://github.com/jdx/mise/commit/cfda918b07e7be556a9def807f96f60ef9addfce)
- make `ls --offline` default behavior by [@jdx](https://github.com/jdx) in [ca0cff2](https://github.com/jdx/mise/commit/ca0cff26f2683d1c2050e3c98676c937be7dd438)

### New Contributors

- @msnazarow made their first contribution

## [2024.12.2](https://github.com/jdx/mise/compare/v2024.12.1..v2024.12.2) - 2024-12-07

### 🚀 Features

- **(registry)** add zls to registry by [@hverlin](https://github.com/hverlin) in [cc0c4ce](https://github.com/jdx/mise/commit/cc0c4ce4db24d53b976799d6c03173b03f02fe94)
- Add --json-extended option to mise env by [@hverlin](https://github.com/hverlin) in [d1f9b6a](https://github.com/jdx/mise/commit/d1f9b6ada4642dcdb74c2d382281075f0f116471)

### 🐛 Bug Fixes

- **(config)** set config_root for tasks defined in included toml files by [@risu729](https://github.com/risu729) in [6b06ac9](https://github.com/jdx/mise/commit/6b06ac9cd12da6ee1423c50acd2004a612cc82df)
- global hooks by [@jdx](https://github.com/jdx) in [45a2513](https://github.com/jdx/mise/commit/45a251342a5bfb50407d0c04535e0fd50eda74f1)
- only run watch_file hook when it has changed file by [@jdx](https://github.com/jdx) in [3e5461d](https://github.com/jdx/mise/commit/3e5461d3b2847df317e955483775d8b14495c2e2)
- bug with aliasing core tools by [@jdx](https://github.com/jdx) in [af1bee0](https://github.com/jdx/mise/commit/af1bee0947ef6ef730e3e38eed722713f9674a55)
- remove shims directory before activating by [@jdx](https://github.com/jdx) in [b0709a0](https://github.com/jdx/mise/commit/b0709a0c825edf2edc7af849c8b54ad1e70857be)

### 🚜 Refactor

- use github crate to list zig releases by [@risu729](https://github.com/risu729) in [e57d0a4](https://github.com/jdx/mise/commit/e57d0a4c36a4136337f7fb8831db5fe893d2cd5e)

### 📚 Documentation

- add zig to core tools by [@risu729](https://github.com/risu729) in [88f8e89](https://github.com/jdx/mise/commit/88f8e89ac4cef83242cab97ddc6b422bcf0567de)

### Chore

- debug log by [@jdx](https://github.com/jdx) in [0075db0](https://github.com/jdx/mise/commit/0075db05a24a9bc2e3015b8a48bcfe730fe80d07)

## [2024.12.1](https://github.com/jdx/mise/compare/v2024.12.0..v2024.12.1) - 2024-12-06

### 🚀 Features

- **(registry)** use aqua for some tools by [@risu729](https://github.com/risu729) in [2a4d74c](https://github.com/jdx/mise/commit/2a4d74cab67a66ec0249fbfc89120e3cfdfc2dec)
- allow filtering `mise bin-paths` on tools by [@jdx](https://github.com/jdx) in [b545b0b](https://github.com/jdx/mise/commit/b545b0bf7508e96938c106719cad7b3f17ea4ee1)
- added aws-cli from aqua by [@jdx](https://github.com/jdx) in [3222e48](https://github.com/jdx/mise/commit/3222e48c35b915b2a396ceed9207554a78c0ae2d)
- multiple MISE_ENV environments by [@jdx](https://github.com/jdx) in [d3519c3](https://github.com/jdx/mise/commit/d3519c32881da46fe2a19ea11497b30a55f17e2d)
- add mise-task.json schema by [@hverlin](https://github.com/hverlin) in [00c1687](https://github.com/jdx/mise/commit/00c1687a7b3876212b6d75068fd3c3c2bcb503b8)
- automatically call `hook-env` by [@jdx](https://github.com/jdx) in [4683b32](https://github.com/jdx/mise/commit/4683b32e8c52a8c0952e4c4706eef57aade05652)

### 🐛 Bug Fixes

- **(docs)** correct syntax error in IDE integration examples by [@EricGusmao](https://github.com/EricGusmao) in [b3f09a2](https://github.com/jdx/mise/commit/b3f09a2e313cf44b2be4d79f2192bcda88a37cab)
- ensure version check message is displayed by [@jdx](https://github.com/jdx) in [711f028](https://github.com/jdx/mise/commit/711f02899d1b2403fbf0b3c3ffcd55e8ec18312d)
- show warning if no precompiled pythons found by [@jdx](https://github.com/jdx) in [d9158ac](https://github.com/jdx/mise/commit/d9158ac7e28c8b7a50cd030f5d80c6c492ac768c)
- allow compilation not on macOS, Linux, or Windows by [@avysk](https://github.com/avysk) in [977957c](https://github.com/jdx/mise/commit/977957c9538486508c35b6b4e021a8b70b8d2cae)
- make hook-env compatible with zsh auto_name_dirs by [@jdx](https://github.com/jdx) in [c8672af](https://github.com/jdx/mise/commit/c8672af8625952be3fe9297370964cd09f099b15)
- skip optional env._.file files by [@jdx](https://github.com/jdx) in [77d75f0](https://github.com/jdx/mise/commit/77d75f024f86c2d9694aeffcb04c6aeba11a35cb)
- .terraform-version by [@jdx](https://github.com/jdx) in [404c9f4](https://github.com/jdx/mise/commit/404c9f434f646b4384d738fc3687b071b1cfa1fd)

### 📚 Documentation

- update auto-completion docs by [@hverlin](https://github.com/hverlin) in [33e88e3](https://github.com/jdx/mise/commit/33e88e3fa49d62cddad208a29c0b67e09697bf7d)
- fix `Environment variables passed to tasks` section by [@hverlin](https://github.com/hverlin) in [83da7f2](https://github.com/jdx/mise/commit/83da7f2590877a29f172a0b356f0b549a2128b9f)

### 🧪 Testing

- try to fix coverage rate limits by [@jdx](https://github.com/jdx) in [d40a45c](https://github.com/jdx/mise/commit/d40a45c12ea33ca5b6d0aef56b22ea60c692a10a)

### New Contributors

- @avysk made their first contribution
- @EricGusmao made their first contribution

## [2024.12.0](https://github.com/jdx/mise/compare/v2024.11.37..v2024.12.0) - 2024-12-04

### 🚀 Features

- **(erlang)** use precompiled binaries for macos by [@jdx](https://github.com/jdx) in [1727515](https://github.com/jdx/mise/commit/172751528ff36f16634c11e4453e693b764c7151)
- add upctl by [@scop](https://github.com/scop) in [6569478](https://github.com/jdx/mise/commit/65694784bc1a407b34f51df6b6376e5ae23db85c)
- Add `json-with-sources` option to settings ls by [@hverlin](https://github.com/hverlin) in [2cd7e94](https://github.com/jdx/mise/commit/2cd7e94de3c67e1b1e0e6eb6416df3eb4ad99924)
- add ripsecrets to registry.toml by [@boris-smidt-klarrio](https://github.com/boris-smidt-klarrio) in [50b249a](https://github.com/jdx/mise/commit/50b249a36084695dfa64058166caa2a1ebd2a1a8)
- Add kyverno-cli by [@boris-smidt-klarrio](https://github.com/boris-smidt-klarrio) in [bb3c263](https://github.com/jdx/mise/commit/bb3c2635f205304bff57f11fae99fadf4143ab3d)

### 🐛 Bug Fixes

- add exec to `mise g git-pre-commit` by [@jdx](https://github.com/jdx) in [27a3aef](https://github.com/jdx/mise/commit/27a3aefa767c8ef142009dd54c4d7dcc19c235b2)
- bake gpg keys in by [@jdx](https://github.com/jdx) in [dfd8bd7](https://github.com/jdx/mise/commit/dfd8bd7cf536e42cd1a746b44fe7a2f8260f754a)
- deprecate `mise local|global` by [@jdx](https://github.com/jdx) in [9bbff66](https://github.com/jdx/mise/commit/9bbff6657a968a743b6a497f0db1b279ba66abd8)

### 🚜 Refactor

- use aqua for ruff by [@scop](https://github.com/scop) in [503d758](https://github.com/jdx/mise/commit/503d758ca4f5d060a574e9a84fd5d084b08dcdea)

### 📚 Documentation

- add terraform recipe to the cookbook by [@AliSajid](https://github.com/AliSajid) in [80579af](https://github.com/jdx/mise/commit/80579afa9199ba18fd5ad06bc03bc387ce3840f7)
- fix git examples for cargo backend by [@tmeijn](https://github.com/tmeijn) in [5eeff5d](https://github.com/jdx/mise/commit/5eeff5d32028adbe26504ea99d3669297fdba991)

### 🧪 Testing

- remove non-working maven test by [@jdx](https://github.com/jdx) in [5a3ed16](https://github.com/jdx/mise/commit/5a3ed16efb29dbf80f5ac251eec39e3a462d2219)
- remove gleam by [@jdx](https://github.com/jdx) in [fdfe20b](https://github.com/jdx/mise/commit/fdfe20b32b16b835655551d3f12b5d6e90856b2e)
- use latest golang in e2e test by [@jdx](https://github.com/jdx) in [5a70357](https://github.com/jdx/mise/commit/5a7035789f8fd0ba16bc01767af9fd251fa57eb5)

### Chore

- upgrade usage-lib by [@jdx](https://github.com/jdx) in [554d533](https://github.com/jdx/mise/commit/554d533a253a137c27c5cdac6da2ae09629029dc)
- use asdf:mise-plugins/mise-nim by [@jdx](https://github.com/jdx) in [754cace](https://github.com/jdx/mise/commit/754caceddfc76660c0e2e1211e8150d852ef6959)

### New Contributors

- @leogurja made their first contribution
- @tmeijn made their first contribution
- @boris-smidt-klarrio made their first contribution
- @AliSajid made their first contribution

## [2024.11.37](https://github.com/jdx/mise/compare/v2024.11.36..v2024.11.37) - 2024-11-30

### 🚀 Features

- add black by [@scop](https://github.com/scop) in [cfe4deb](https://github.com/jdx/mise/commit/cfe4debbda1c6bb5ca1319802fe7c4f3146fc008)
- migrate more tools away from asdf by [@jdx](https://github.com/jdx) in [40f92c6](https://github.com/jdx/mise/commit/40f92c6b0e1fefd171dd44ee9f62f1f597ee352c)

### 🐛 Bug Fixes

- handle General/Complex Versioning in --bump by [@liskin](https://github.com/liskin) in [e5efc7f](https://github.com/jdx/mise/commit/e5efc7fdeface6712cf09fc06bab0c5ac253dbb8)
- broken path example by [@minddust](https://github.com/minddust) in [1316dd7](https://github.com/jdx/mise/commit/1316dd75a014a2db707dd0b863ee546b7bfccb6b)
- swift path on macos by [@jdx](https://github.com/jdx) in [ee7def5](https://github.com/jdx/mise/commit/ee7def5eed6fcb66080042a87ded4db9db41385d)
- do not auto-install on `mise x` if some tools are passed by [@jdx](https://github.com/jdx) in [35d31a1](https://github.com/jdx/mise/commit/35d31a1baf96fe6f0e764e26228c1b03ba24ddce)
- fix: also make certain we are not auto installing inside shims by checking by [@jdx](https://github.com/jdx) in [b0c4a74](https://github.com/jdx/mise/commit/b0c4a749309064825852041d8d72c7eac9fb116c)
- cache github release information for 24 hours by [@jdx](https://github.com/jdx) in [4ff9960](https://github.com/jdx/mise/commit/4ff9960129965a100bb12e39ac2488f2fc68d5ce)

### 🚜 Refactor

- use aqua for snyk by [@scop](https://github.com/scop) in [f8fc242](https://github.com/jdx/mise/commit/f8fc242e1cae5ffcd0dbf3924a310ab76c470e2a)

### Chore

- bump expr-lang by [@jdx](https://github.com/jdx) in [774b7e4](https://github.com/jdx/mise/commit/774b7e43aeb02b2e6e1d18ca7ef7cbcab5a0eed9)
- mise up --bump by [@jdx](https://github.com/jdx) in [6872b54](https://github.com/jdx/mise/commit/6872b5469622140335a12131dfa4acf310fc0c2a)
- update mise.lock by [@jdx](https://github.com/jdx) in [4c12502](https://github.com/jdx/mise/commit/4c12502c459ba2e214689c3f55d964b8f75966af)
- disable tool tests until I can sort out gh rate limit issues by [@jdx](https://github.com/jdx) in [f42f010](https://github.com/jdx/mise/commit/f42f010f03a57cab128290c0b9d936fd7a90c785)

### New Contributors

- @minddust made their first contribution

## [2024.11.36](https://github.com/jdx/mise/compare/v2024.11.35..v2024.11.36) - 2024-11-29

### Chore

- mise i by [@jdx](https://github.com/jdx) in [8150732](https://github.com/jdx/mise/commit/81507327e7f1c9f2137b3dadcf35a8245d43a8ba)

## [2024.11.35](https://github.com/jdx/mise/compare/v2024.11.34..v2024.11.35) - 2024-11-29

### 🚀 Features

- migrate more tools away from asdf by [@jdx](https://github.com/jdx) in [0f26a1a](https://github.com/jdx/mise/commit/0f26a1a390e197cc6799171366503a1843e51e16)

### 🐛 Bug Fixes

- remove conflicting MISE_SHELL setting by [@jdx](https://github.com/jdx) in [9454d0a](https://github.com/jdx/mise/commit/9454d0a0c94f89939c0865b8f3f2724b90023abd)

### 🚜 Refactor

- simplify __MISE_WATCH variable to only contain the most recent timestamp by [@jdx](https://github.com/jdx) in [4d5daa5](https://github.com/jdx/mise/commit/4d5daa51cf65180e3e536636c0d7c50f003f727c)

### 🧪 Testing

- remove unnecessary cargo-binstall test by [@jdx](https://github.com/jdx) in [0a4da7a](https://github.com/jdx/mise/commit/0a4da7a023b1cb969b732afd3ad4b3cf02c42530)

### Chore

- dont require build-windows before unit-windows by [@jdx](https://github.com/jdx) in [c85e2ec](https://github.com/jdx/mise/commit/c85e2ec77193d73ff20d4ce8fb7e3787a6db223d)

## [2024.11.34](https://github.com/jdx/mise/compare/v2024.11.33..v2024.11.34) - 2024-11-29

### 🚀 Features

- fragmented configs by [@jdx](https://github.com/jdx) in [da6db80](https://github.com/jdx/mise/commit/da6db80cb0e94416d1e6e3e3f02dd74af44fc2d6)
- hooks by [@jdx](https://github.com/jdx) in [ce839ae](https://github.com/jdx/mise/commit/ce839aebab2a7231f554ac5a7e469cf4193f3faa)
- added MISE_TASK_DISABLE_PATHS setting by [@jdx](https://github.com/jdx) in [9c2e6e4](https://github.com/jdx/mise/commit/9c2e6e40f3a98f352fbf03107e1901dec445a7f5)
- gpg verification for node by [@jdx](https://github.com/jdx) in [3aa2de7](https://github.com/jdx/mise/commit/3aa2de7c32542992408d455a57eac9c44c5d976d)

### 🐛 Bug Fixes

- make _.file and _.source optional if the file is missing by [@jdx](https://github.com/jdx) in [ef37b7c](https://github.com/jdx/mise/commit/ef37b7cfdd11d469b56166867dc300a179e521d3)
- prevent deadlock when resetting by [@jdx](https://github.com/jdx) in [8e6d093](https://github.com/jdx/mise/commit/8e6d09377de81c65203684725fa9dfc2140db520)
- prevent deadlock when resetting by [@jdx](https://github.com/jdx) in [201ba90](https://github.com/jdx/mise/commit/201ba904052379595e399672d1657ed0e3c3a138)
- prevent deadlock when resetting by [@jdx](https://github.com/jdx) in [169338a](https://github.com/jdx/mise/commit/169338a2debb99ee4dd885376c4123740237af23)

### 🚜 Refactor

- clean up arcs by [@jdx](https://github.com/jdx) in [f49d330](https://github.com/jdx/mise/commit/f49d330b6f97b08e72b1a448af0021708b2a2417)

### 📚 Documentation

- added hooks to sidebar by [@jdx](https://github.com/jdx) in [4bbc340](https://github.com/jdx/mise/commit/4bbc3403e46aa817450e6936f37b5d4c983b43d4)
- added swift to sidebar by [@jdx](https://github.com/jdx) in [bc06cbf](https://github.com/jdx/mise/commit/bc06cbf240cc7aae2173575cfa83289ae526dad1)

### Chore

- skip checkov test by [@jdx](https://github.com/jdx) in [2ae18a3](https://github.com/jdx/mise/commit/2ae18a3e8329eb9913dc43ae94432f8f75b36a94)
- added timeout for release-plz by [@jdx](https://github.com/jdx) in [dae4bc3](https://github.com/jdx/mise/commit/dae4bc32bbb7de7873e3fa047a785c70f02a5c05)
- remove coverage by [@jdx](https://github.com/jdx) in [78ac92c](https://github.com/jdx/mise/commit/78ac92c77be25e946348617bff7c9567867f882c)

## [2024.11.33](https://github.com/jdx/mise/compare/v2024.11.32..v2024.11.33) - 2024-11-28

### 🚀 Features

- respect --quiet in `mise run` by [@jdx](https://github.com/jdx) in [11a785d](https://github.com/jdx/mise/commit/11a785d5ce4008f18d71cf4a1cc321703e683131)
- added special "_" portion of mise.toml for custom data by [@jdx](https://github.com/jdx) in [46e9b8a](https://github.com/jdx/mise/commit/46e9b8a9edca39bd465f25621d65c473a0c35ae7)
- **breaking** added MISE_OVERRIDE_CONFIG_FILENAMES config by [@jdx](https://github.com/jdx) in [5a0508f](https://github.com/jdx/mise/commit/5a0508fba974bcce0a5865000d3716e18b88d17e)
- added swift by [@jdx](https://github.com/jdx) in [1cfa6dc](https://github.com/jdx/mise/commit/1cfa6dc53ddcc9fa2b085eebd6b80dda37890159)

### 🐛 Bug Fixes

- **(spm)** git proxy config by [@jdx](https://github.com/jdx) in [9a953e0](https://github.com/jdx/mise/commit/9a953e00a11f829bfae882c314ece41426bcfc8c)
- clean up some windows error cases by [@jdx](https://github.com/jdx) in [9a5d1bf](https://github.com/jdx/mise/commit/9a5d1bfbcc8d378509739f11b07632284e42155f)
- run `hook-env` on directory change by [@jdx](https://github.com/jdx) in [e6b0754](https://github.com/jdx/mise/commit/e6b07540d174d76caf122de54f6c9c12fabc691f)
- always prefer glibc to musl in mise run by [@jdx](https://github.com/jdx) in [3860e95](https://github.com/jdx/mise/commit/3860e953cc0f4ac705cc8cfa412e53f9efb48b69)
- issue with non-default backends not getting tool options by [@jdx](https://github.com/jdx) in [fa2eb47](https://github.com/jdx/mise/commit/fa2eb47907a2e8ddcba16ae6973401adf8e8405a)
- explicitly stop progress bars when exiting by [@jdx](https://github.com/jdx) in [8c0b694](https://github.com/jdx/mise/commit/8c0b6942e04348b5e382c1da0043a9fa78ff1c19)

### 🚜 Refactor

- use aqua for shellcheck by [@scop](https://github.com/scop) in [9a33115](https://github.com/jdx/mise/commit/9a331154455f53da970b3a1ae401ccf3b628098d)
- use aqua for goreleaser by [@scop](https://github.com/scop) in [eef4656](https://github.com/jdx/mise/commit/eef4656d740b6e63222e2071ec932b2184273e1c)
- use aqua for golangci-lint by [@scop](https://github.com/scop) in [a443aa0](https://github.com/jdx/mise/commit/a443aa0bcd5beec3e24bd445d5bc799991647610)

### 📚 Documentation

- describe mise behavior when mise version is lower than min_version by [@erickguan](https://github.com/erickguan) in [8b77d2e](https://github.com/jdx/mise/commit/8b77d2e9e75732a3a923f54d8614a0fc1209d014)

### Chore

- wait for gh rate limit if expended by [@jdx](https://github.com/jdx) in [078bed1](https://github.com/jdx/mise/commit/078bed14246b73330a6cef071d0375958e7a5d8d)
- set github token for docs job by [@jdx](https://github.com/jdx) in [908dd18](https://github.com/jdx/mise/commit/908dd18fe3ddf19d1531c93695ee3ff98d0995c5)
- skip hyperfine unless on release pr by [@jdx](https://github.com/jdx) in [0da436d](https://github.com/jdx/mise/commit/0da436d825fa72b2a30cb6602d79b0b9254af09b)
- move tasks dir so it doesnt show up in unrelated projects by [@jdx](https://github.com/jdx) in [4f5c239](https://github.com/jdx/mise/commit/4f5c239e8a50d529eda0e8a97216d4f689c4be4c)

## [2024.11.32](https://github.com/jdx/mise/compare/v2024.11.31..v2024.11.32) - 2024-11-27

### 🚀 Features

- allow running tasks without `mise run`, e.g.: `mise test` as shorthand for `mise run test` by [@jdx](https://github.com/jdx) in [8f31a33](https://github.com/jdx/mise/commit/8f31a333ebb9776099d4269f5de9b0197aecea95)
- default task directory config by [@jdx](https://github.com/jdx) in [04f6c4b](https://github.com/jdx/mise/commit/04f6c4bb1db57357556093d2406924cc398e082b)
- standalone tasks by [@jdx](https://github.com/jdx) in [aba8c04](https://github.com/jdx/mise/commit/aba8c043423f25044c4a9f0715650756fc58d6d8)
- automatic uv venv activation by [@jdx](https://github.com/jdx) in [a8cdb2d](https://github.com/jdx/mise/commit/a8cdb2d68945014f3c2a7760d33845876ee28560)
- migrate more tools away from asdf by [@jdx](https://github.com/jdx) in [adef34a](https://github.com/jdx/mise/commit/adef34a17534793eeac450afee1d9d9a36306caf)
- add committed by [@scop](https://github.com/scop) in [1202940](https://github.com/jdx/mise/commit/1202940137cf637e22d337cb5c45c854a1e9dac5)
- use ubi for figma-export by [@jdx](https://github.com/jdx) in [19dbeac](https://github.com/jdx/mise/commit/19dbeac16a68248bb780a2de1056d16409714204)
- add vacuum by [@scop](https://github.com/scop) in [97686c8](https://github.com/jdx/mise/commit/97686c825f0104c46d9500924392759e0bd2dc6d)

### 🐛 Bug Fixes

- skip _.source files if not present by [@jdx](https://github.com/jdx) in [b3cb34f](https://github.com/jdx/mise/commit/b3cb34f3f727dc7200cb91daffa81122128194c7)
- rust idiomatic file parsing by [@jdx](https://github.com/jdx) in [9c13dcc](https://github.com/jdx/mise/commit/9c13dcc7420ab0f777b909201b5c9e30d94fa1fc)
- automatic reinstall of uvx tools during python upgrades by [@jdx](https://github.com/jdx) in [4e25299](https://github.com/jdx/mise/commit/4e25299460dda652b62138438163ff3f7cfca8f2)

### 🚜 Refactor

- use aqua for shfmt by [@scop](https://github.com/scop) in [9d91494](https://github.com/jdx/mise/commit/9d9149459c5658e1733deb5884ebcb13e64be8a2)
- use aqua for lefthook by [@scop](https://github.com/scop) in [4386584](https://github.com/jdx/mise/commit/43865846aca79dffd6c53bc23ad335de441c8297)
- use aqua for nfpm by [@scop](https://github.com/scop) in [244102f](https://github.com/jdx/mise/commit/244102f6dc02825b0890fbe7e89c24112cc2324c)

### 📚 Documentation

- correction in aqua by [@jdx](https://github.com/jdx) in [b7de2f3](https://github.com/jdx/mise/commit/b7de2f32e6a23458bbd3573372f9c49733b80e62)
- typo by [@jdx](https://github.com/jdx) in [98aa6bd](https://github.com/jdx/mise/commit/98aa6bd7b2631a5904243cbf9aeb2eaf218c9c64)

### Chore

- bump tabled by [@jdx](https://github.com/jdx) in [d987411](https://github.com/jdx/mise/commit/d987411451abbce14fa4f6db7b7b93604b9bb6ff)
- fix tools tests on release branch by [@jdx](https://github.com/jdx) in [675a2b0](https://github.com/jdx/mise/commit/675a2b086116f0afb431189c51136255b6f6c434)
- fix tools tests on release branch by [@jdx](https://github.com/jdx) in [130c3a4](https://github.com/jdx/mise/commit/130c3a4de60edfbed98642bc6dc71e67ba9b6ce1)
- fix tools tests on release branch by [@jdx](https://github.com/jdx) in [9feb3b6](https://github.com/jdx/mise/commit/9feb3b638ef634d320f576921b3e366f6cd73075)

### New Contributors

- @rmacklin made their first contribution

## [2024.11.31](https://github.com/jdx/mise/compare/v2024.11.30..v2024.11.31) - 2024-11-27

### 🚀 Features

- rust in core by [@jdx](https://github.com/jdx) in [de3a209](https://github.com/jdx/mise/commit/de3a209092d9fe6148b81898b88b18c6485f3acd)

### 🐛 Bug Fixes

- use tv.pathname() in `mise ls` by [@jdx](https://github.com/jdx) in [a06299e](https://github.com/jdx/mise/commit/a06299ea257953a538544e3a3bdc9604f762ccb5)
- show gh rate limit reset time by [@jdx](https://github.com/jdx) in [e579b79](https://github.com/jdx/mise/commit/e579b794eeede8f37a46f50accdb62540ad53acd)
- add @version back into show_tools by [@jdx](https://github.com/jdx) in [fd7d8d1](https://github.com/jdx/mise/commit/fd7d8d10395f8c80a80c60c0de89bf78e31fd762)
- use pipx for yamllint by [@jdx](https://github.com/jdx) in [f246584](https://github.com/jdx/mise/commit/f246584e0ece8f86b802680cff31186f36351257)
- remove shims directory in `mise activate` by [@jdx](https://github.com/jdx) in [7de1111](https://github.com/jdx/mise/commit/7de1111db6d166ff85996a6f32dbb815da7a4248)

### 🚜 Refactor

- remove duplicate remote_versions_caches by [@jdx](https://github.com/jdx) in [424a8f8](https://github.com/jdx/mise/commit/424a8f83676b431e3daeb4b1f131421a4374cefd)

### 📚 Documentation

- rename legacy version files to idiomatic version files by [@jdx](https://github.com/jdx) in [e80f211](https://github.com/jdx/mise/commit/e80f211e041a25beba131b0539f262ec3d875ea4)
- document aqua better by [@jdx](https://github.com/jdx) in [d10c7ca](https://github.com/jdx/mise/commit/d10c7ca13e20a62eb8768ddbae7e4dc1a63f3ceb)

### 🎨 Styling

- spelling and grammar fixes by [@scop](https://github.com/scop) in [70373d7](https://github.com/jdx/mise/commit/70373d7c7330861814779919b6f71765ff8eb03d)

### 🧪 Testing

- move some unit tests to e2e by [@jdx](https://github.com/jdx) in [a939f1c](https://github.com/jdx/mise/commit/a939f1cf913638b8bd7a98d2bde13c335082344d)
- migrate tests from unit to e2e by [@jdx](https://github.com/jdx) in [a530e44](https://github.com/jdx/mise/commit/a530e448269db362ac02889d9e7253db6366dfff)

## [2024.11.30](https://github.com/jdx/mise/compare/v2024.11.29..v2024.11.30) - 2024-11-26

### 🚀 Features

- migrate wren-cli to ubi by [@jdx](https://github.com/jdx) in [5c4332e](https://github.com/jdx/mise/commit/5c4332eccb5f807d410386c05700d67a3df3fc6d)
- migrate more tools away from asdf by [@jdx](https://github.com/jdx) in [1763953](https://github.com/jdx/mise/commit/1763953de202463fa0597a4d3845f5024819510f)
- automatically set `set -e` in toml tasks by [@jdx](https://github.com/jdx) in [5e8d801](https://github.com/jdx/mise/commit/5e8d8019d1f0ba47e68394dc48d378df0e3db292)
- added MISE_ORIGINAL_CWD to tasks by [@jdx](https://github.com/jdx) in [f7d5fdd](https://github.com/jdx/mise/commit/f7d5fdd20341436151541b93b288ccf54282f083)
- add ruby backend by [@andrewthauer](https://github.com/andrewthauer) in [551584c](https://github.com/jdx/mise/commit/551584cb20becf5fcbf6ac647d81326f24681f29)
- migrate more tools away from asdf by [@jdx](https://github.com/jdx) in [b200169](https://github.com/jdx/mise/commit/b200169041ce4d3df0bc420fd62b1cfea44a8d05)

### 🐛 Bug Fixes

- Make Rebar backend depend on Erlang by [@eproxus](https://github.com/eproxus) in [aae4602](https://github.com/jdx/mise/commit/aae46028fbb01a191a72143d4c59ed1323bd547e)
- trust system/global config by default by [@jdx](https://github.com/jdx) in [be22f00](https://github.com/jdx/mise/commit/be22f00298e78d3da761c2a9d0fad525c56d68c2)
- use tv.short in show_tools by [@jdx](https://github.com/jdx) in [e22613d](https://github.com/jdx/mise/commit/e22613d83351ca1a5d2c3fa5bf185838c3f20603)

### 📚 Documentation

- flatten tools in sidebar by [@jdx](https://github.com/jdx) in [0556024](https://github.com/jdx/mise/commit/0556024b5abdb2d5f1cb025d105494c71aa79647)

### 🧪 Testing

- remove flaky maven test by [@jdx](https://github.com/jdx) in [65f6eb4](https://github.com/jdx/mise/commit/65f6eb48880b6322439c33b3cd53eab7b8b97439)
- added test for vault by [@jdx](https://github.com/jdx) in [f463cd4](https://github.com/jdx/mise/commit/f463cd4360d24a441e6de6a4cb66ab684a349ee2)

### Chore

- bump expr-lang by [@jdx](https://github.com/jdx) in [7d3324b](https://github.com/jdx/mise/commit/7d3324b1aecbeac88d925691b2041c35efe57cac)
- add aqua-registry as submodule by [@jdx](https://github.com/jdx) in [2e3046e](https://github.com/jdx/mise/commit/2e3046e2eeaf99a4db9140953c3a266c682b55b4)

### New Contributors

- @eproxus made their first contribution

## [2024.11.29](https://github.com/jdx/mise/compare/v2024.11.28..v2024.11.29) - 2024-11-25

### 🚀 Features

- **(env)** Allow exporting env vars as dotenv format by [@miguelmig](https://github.com/miguelmig) in [7de9d12](https://github.com/jdx/mise/commit/7de9d1238ac9e0136721511bef0099a24ce9764e)
- move more tools away from asdf by [@jdx](https://github.com/jdx) in [d86c564](https://github.com/jdx/mise/commit/d86c564930c978e47efd79808f825d87c0076c51)
- use aqua for cargo-binstall by [@jdx](https://github.com/jdx) in [4d50133](https://github.com/jdx/mise/commit/4d50133715d4ab0a8e9595011edc4269d3fe1f9a)

### 🐛 Bug Fixes

- use shift_remove by [@jdx](https://github.com/jdx) in [0df8bf7](https://github.com/jdx/mise/commit/0df8bf7be73a1f8c1cddb3eb13a67817aa0e8baa)
- pass boolean tool options as strings by [@jdx](https://github.com/jdx) in [2c241cd](https://github.com/jdx/mise/commit/2c241cdcc9ada59b7505920cb77c9c6541ec5ea6)
- move semver cmp errors to debug by [@jdx](https://github.com/jdx) in [ab4e638](https://github.com/jdx/mise/commit/ab4e638cdeda9845f3b7421a22a0d3bf71d81eae)
- show more accurate error if no tasks are available by [@jdx](https://github.com/jdx) in [e1b1b48](https://github.com/jdx/mise/commit/e1b1b48840b8c96e45a567a47922138544ab9f59)
- move semver cmp errors to debug by [@jdx](https://github.com/jdx) in [c2f0a5a](https://github.com/jdx/mise/commit/c2f0a5a567eb96e5adff5b023dcbeced5e1e93b0)
- use aqua for terraform by [@jdx](https://github.com/jdx) in [a4d77b0](https://github.com/jdx/mise/commit/a4d77b0e85b4c9bfe989da9b318621ac6ada73de)

### 🧪 Testing

- disable cargo-binstall test by [@jdx](https://github.com/jdx) in [8fee82e](https://github.com/jdx/mise/commit/8fee82e652031a1c9a31dbb05437478c961b6107)

### Chore

- include aqua-registry yaml files in crate by [@jdx](https://github.com/jdx) in [0e3df3d](https://github.com/jdx/mise/commit/0e3df3d347a62058ad08ccda95ea207fe703998b)
- gitignore aqua-registry by [@jdx](https://github.com/jdx) in [1c38bca](https://github.com/jdx/mise/commit/1c38bca434cfc17792eb3053be2f4271a9e92fdd)
- gitignore aqua-registry by [@jdx](https://github.com/jdx) in [644cb6d](https://github.com/jdx/mise/commit/644cb6dfa762d6360b5aaa7fce0502fe61ac1067)

## [2024.11.28] - 2024-11-24

### 🚀 Features

- migrate more tools away from asdf by [@jdx](https://github.com/jdx) in [b6dd768](https://github.com/jdx/mise/commit/b6dd76827ad332b6bd3dc26c83bf4e512643c26b)
- auto-install tools on `mise run` by [@jdx](https://github.com/jdx) in [26a144b](https://github.com/jdx/mise/commit/26a144b9ac71ce18bb1faa74ef9cb005a64eff45)
- move more tools away from asdf by [@jdx](https://github.com/jdx) in [7480254](https://github.com/jdx/mise/commit/7480254d9ad76ff7079914aef4ca2864a63a8f26)

### 🐛 Bug Fixes

- allow passing integers to task env by [@jdx](https://github.com/jdx) in [0d59890](https://github.com/jdx/mise/commit/0d598908ca4a02d5bd04b6ab280d1934899ee257)
- remove __MISE_WATCH,__MISE_DIFF env vars on `mise deactivate` by [@jdx](https://github.com/jdx) in [edd3d69](https://github.com/jdx/mise/commit/edd3d69c76ad68bedb82b882391c6aee64584d0b)

### 📚 Documentation

- **(security)** added information about checksums/cosign/slsa verification by [@jdx](https://github.com/jdx) in [1faef6e](https://github.com/jdx/mise/commit/1faef6ecbb48692955f4ce424d77d03472aa4617)
- **(security)** added release gpg key by [@jdx](https://github.com/jdx) in [8f5dfd6](https://github.com/jdx/mise/commit/8f5dfd6dd2903c55fd792aeecd8ec97ef9f7f7ba)
- typos by [@jdx](https://github.com/jdx) in [8535827](https://github.com/jdx/mise/commit/8535827bd055c08a86e574fcec67efff842b2fa9)

### Chore

- clean up CHANGELOG by [@jdx](https://github.com/jdx) in [8ec0ca2](https://github.com/jdx/mise/commit/8ec0ca20fce57d07d769209fd9043a129daa86f1)

<!-- generated by git-cliff -->
