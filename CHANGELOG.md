# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0-rc.1] - 2023-12-21

### Added

- Warning if running unauthenticated commands as a sudo user

### Changed

- Cleaner unauthenticated error

### Fixed

- Custom engines no longer get prompted to select engine when running `rivet init` for the second time

## [v0.4.0] - 2023-12-20

### Added

- Auto-generate GitHub Actions with `rivet ci generate github`
- Development token cache to make `rivet token create development` run faster
- Shorthand `-n` for `--namespace` flag in `rivet token create development`
- `rivet deploy` validates config before building & uploading resources
- `rivet unlink` command to remove authentication token
- Pretty-printed errors instead of default debug format
- Error reporting to Sentry

### Changed

- Removed engine prompt if Rivet config already exists
- **[BREAKING]** No longer automatically creates/updates `.env` file in favor of using `rivet token create development`
- Global flags (`--api-endpoint`, `--token`, and `--disable-telemetry`) can now be used in subcommands (e.g. `rivet init --token foobar` instead of `rivet --token foobar init`)
- Moved project metadata to global configuration file
- Removed `.rivet` from auto-generated `.gitignore`
- `rivet namespace create` can be called without specifying `--version`
- **[BREAKING]** Change `TELEMETRY_DISABLED` env var to `RIVET_TELEMETRY_DISABLED`
- Remove trailing line break from `rivet token create development`
- Rename `rivet site` subcommands to `rivet cdn` (alias still supported)
- Rename `rivet image` subcommands to `rivet docker` (alias still supported)
- Rename `dashboard` subcommands to `view` (alias still supported)
- Move `rivet version deploy` to `rivet deploy`
- Move `rivet version config-validate` to `rivet config validate`
- Move `RIVET_CONCURRENT_UPLOADS` env var to CLI flag on appropriate commands (env var still works)
- Streamline `rivet init` experience
- Add `rivet token create public` command

### Fixed

- Fix `matchmaker.game_modes.*.docker.image_id` falling back to `matchmaker.docker.image_id`
- **Install script** Now installs non-prerelease GitHub releases

## [v0.3.0] - 2023-12-10

### Added

- **Install script (Unix)** Configure installation directory by passing `$BIN_DIR`
- **Install script (Unix)** Warning if `$BIN_DIR` is not in `$PATH`

### Changed

- Auto-generated & recommended config is now a `rivet.yaml` file
- Default version names are now generated as `YYYY.MM (X)` format (where `X` is an incrementing index)
- Merged `.rivet/cloud_token` and `.rivet/config.toml` in to unified internal `.rivet/config.yaml` config file
- **[BREAKING]** Removed support for file formats that are not YAML, TOML, or JSON in order to simplify maintaining forward compatibility
- **[BREAKING]** Throw error if both `.yaml` and `.yml` config exist

### Fixed

- **Install script (Unix)** Installing ARM64 `jq` binary on ARM-based Macs
- **Install script (Unix)** Automatically create `$BIN_DIR` if doesn't exist, specifically on macOS Sonoma which does not provide a `/usr/local/bin` by default

## [v0.2.0] - 2023-12-1

### Added

- Support for building OCI bundles
- Support for LZ4 compression of builds
- **[BREAKING]** Expose `RIVET_API_ENDPOINT` to `cdn.build_command` to help automate deploying to multiple clusters
- **[BREAKING]** Unset `RIVET_TOKEN` to `cdn.build_command` in order to ensure the cloud token isn't accidentally baked in a build
- `image build-push` command to automatically build & push an image
- `site build-push` command to automatially build and push a site
- E2E cross-platform tests in GitHub Actions

### Changed

- **[BREAKING]** Support new single-origin API endpoint (configured with `RIVET_API_ENDPOINT` environment variable or `--api-endpoint` flag)
- **[BREAKING]** Rename `RIVET_CLOUD_TOKEN` environment variable to `RIVET_TOKEN`
- **[BREAKING]** Rename `--cloud-token` flag to `--token`
- **[BREAKING]** Removed `RIVET_API_CLOUD_URL` in favor of `RIVET_API_ENDPOINT`
- **[BREAKING]** Updated custom games config schema
- **[BREAKING]** Removed domain map from turnstile configuration, replaced with `site_key` and `secret_key`
- Added telemetry beacon for fatal errors. Opt out with `--telemetry-disabled` or `TELEMETRY_DISABLED=1`
- Added internal config to store api endpoint and telemetry options
- Implemented multipart uploads for builds and sites, disable multipart uploads with `_RIVET_UPLOAD_DISABLE_MULTIPART`

## [v0.1.4] - 2023-12-9

### Added

- Darwin ARM release artifact

### Changed

- Update `cargo-dist` to 0.5.0

## [v0.1.3] - 2023-12-3

### Changed

- Replace Smithy-generated API library with OpenAPI-generated library in order to fix `invalid certificate timestamp: UnknownLog` error

## [v0.1.2] - 2023-08-26

### Changed

- Added custom games + lobby state + external verification

## [v0.1.1] - 2023-07-17

### Changed

- `rivet deploy` now gracefully falls back to the native build method if Docker Buildx is not installed

## [v0.1.0] - 2023-07-17

### Added

- Unreal helper in `rivet init`
- Installer for the Unreal Engine plugin with `rivet unreal install-plugin` or `rivet init --unreal`

### Changed

- Renamed `rivet.version.toml` to `rivet.toml`. All changes are backwards compatible.
- Renamed `rivet publish` command to `rivet deploy` since this is the more commonly used alias
- `rivet token create dev` now prints token in plain text

### Fixed

- Broken links to old docs
- Docker builder now catches missing builder errors correctly for older Docker versions

## [v0.0.51] - 2023-04-26

### Fixed

- Docker builder now catches missing builder errors correctly for older Docker versions

### Changed

- Remove `PORT`, `RIVET_LOBBY_TOKEN`, and `RIVET_PUBLIC_TOKEN` from generated .env file
- Document development token in .env

## [v0.0.50] - 2023-04-18

### Changed

- Description, homepage, and repository to Cargo.toml

### Fixed

- Incorrect package version

## [v0.0.49] - 2023-04-18

### Added

- Experimental build configuration flag `_RIVET_DOCKER_BUILD_METHOD` can be set to `buildx` or `native`

### Changed

- Default Docker build method is now Buildx, even if the native platform is x86
- Update dependency: `rivet-api`
- Upgrade dependency: `tokio 1.27`
- Removed unnecessary feature flags from `tokio`
