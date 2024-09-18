# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add `--hash` flag to pass the expected hash of the downloaded file.

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
