# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0](https://github.com/asfaload/asfald/compare/v0.1.0...v0.2.0) (2024-10-17)


### Features

* accept hash value on command line ([4093382](https://github.com/asfaload/asfald/commit/40933825af60aef62cabe5f08be14a50e765d54b))
* add pre-commit support ([53b3bcf](https://github.com/asfaload/asfald/commit/53b3bcf6c7ca2c58ac2b2c1706d743a24943a1a4))
* download checksums file from our mirror when flag -a used ([759edca](https://github.com/asfaload/asfald/commit/759edcac64327fd7a67be4a00cdfbe19392a0975))
* enable lto in release build ([9e39df1](https://github.com/asfaload/asfald/commit/9e39df1e80a98a1dfd76ff727998a3398faeb7cf))
* potentially support multiple mirrors ([c922aaf](https://github.com/asfaload/asfald/commit/c922aafd55d77a5c66731bc10391227ab05c625a))
* report host that served the checksums file ([2698a5b](https://github.com/asfaload/asfald/commit/2698a5b3a7c044d2bde5d66699c989824bb524df))
* support cloudflare pages ([300b5d0](https://github.com/asfaload/asfald/commit/300b5d0cd265c59ac9fba52b732697776500d6ad))


### Bug Fixes

* defining the CNAME to gh removes the prfix in the url ([df93ed4](https://github.com/asfaload/asfald/commit/df93ed4f0965c43028c2ca8d0f262148a58f0bfc))
* explicitly write out lifetime ([533d8f8](https://github.com/asfaload/asfald/commit/533d8f883f4fc883bbf66ee8a396a6a3141982cb))

## [Unreleased]

### Added

- Add `--hash` flag to pass the expected hash of the downloaded file.

### Changed

- Renamed to asfald

## 0.1.0 - 2024-09-18

### Added

- Publish `asfd` executables as individual files in releases, making it easier to use in Dockerfiles

### Fixed

- Handle `https` URLs
- Handle checksums files with filenames having a path component (like [mise](https://github.com/jdx/mise/) releases)
- Handle binary file makers in checksums files



## [0.0.1] - 2024-09-13

### Added

- Download file and validate it against a checksum file in the same directory
- `-p` flag to specify location of the checksum file. Supports variable filename, path, and fullpath (See `--help`)
- quiet mode with `-q`
- `--force-absent` to continue if the checksum is absent.
- `--force-invalid` to continue even if the checksum of the file downloaded is invalid.
- `-o` flag to specify destination of downloaded file. WARNING: overrides existing files without confirmation prompt.
