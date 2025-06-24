# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0](https://github.com/asfaload/asfald/compare/v0.5.1...v0.6.0) (2025-06-24)


### Features

* add flag to overwrite existing files ([ebb8589](https://github.com/asfaload/asfald/commit/ebb8589c28fe05c0b2c423d95c9005ae220af857))
* print vulnerability window of index ([158048c](https://github.com/asfaload/asfald/commit/158048c573225d254b50e5a6d0d075cf3641fa73))
* print vulnerability window of index ([640e316](https://github.com/asfaload/asfald/commit/640e3164e565384004c62a75d30a2e841fc7a33d))
* refuse to overwrite existing file ([ac4b00c](https://github.com/asfaload/asfald/commit/ac4b00cd535d97391fe1ae540a59d86c2c266827))


### Bug Fixes

* add --version flag ([c564ac6](https://github.com/asfaload/asfald/commit/c564ac60419162d4b54336987695395a5f872dc4))
* add --version flag ([93e1982](https://github.com/asfaload/asfald/commit/93e19828c56e4f87f84a3a58df79b1352ffce619))
* unusable checkums in release are logged ([eca99cc](https://github.com/asfaload/asfald/commit/eca99ccc1b70c2cf8f3e5a59aa332e857eb6ca07))
* unusable checkums in release are logged ([9cd27f1](https://github.com/asfaload/asfald/commit/9cd27f155c163f51cfc9a894f4b58d20d0c75ad2))

## [0.5.1](https://github.com/asfaload/asfald/compare/v0.5.0...v0.5.1) (2024-12-10)


### Tests

* file renamed in release after mirror taken ([d2b5c01](https://github.com/asfaload/asfald/commit/d2b5c01c3bbe32b2d01e1db394b63c0284ec5402))

## [0.5.0](https://github.com/asfaload/asfald/compare/v0.4.0...v0.5.0) (2024-12-09)


### Features

* send to stdout if filename is - ([f05b2c4](https://github.com/asfaload/asfald/commit/f05b2c402608bb21af9000dadc4c1e3e7c71d617))


### Bug Fixes

* consider url's port in path to index ([97bc937](https://github.com/asfaload/asfald/commit/97bc937c83b8c9cc85c895f061f664c120aae479))
* logger prints all messages to stderr ([2cc16ba](https://github.com/asfaload/asfald/commit/2cc16ba3c4eb2247040a3e0a6cf2d70e51d87363))
* start file transfer before index download to report inexisting file first ([0313083](https://github.com/asfaload/asfald/commit/03130838b25cd10d3b597dacd7f39cddc9062211))

## [0.4.0](https://github.com/asfaload/asfald/compare/v0.3.0...v0.4.0) (2024-11-27)


### Features

* also compare to checksum published in release ([0a8c859](https://github.com/asfaload/asfald/commit/0a8c859dbefbc47bc93091e8e8f2aaff165a4b01))


### Bug Fixes

* disable cloudflare pages mirror ([18b4832](https://github.com/asfaload/asfald/commit/18b4832a06cd418344c63912acf2cf6fcb5deebc))

## [0.3.0](https://github.com/asfaload/asfald/compare/v0.2.0...v0.3.0) (2024-11-08)


### Features

* add get_all_hashes ([96b6976](https://github.com/asfaload/asfald/commit/96b697627522a1ff43bd8aa29e42731dae6dd701))
* add get_best_hash_for_file ([8e3c969](https://github.com/asfaload/asfald/commit/8e3c9698e9ec5638aaf1023c49579a405253ad8d))
* add lib parsing asfaload index files ([6312d8e](https://github.com/asfaload/asfald/commit/6312d8e7df58e76a5592c2a6408650326a1084b2))
* get_all_checksums indicates consistency with enum case ([2a30e26](https://github.com/asfaload/asfald/commit/2a30e26f776350f8b12845f5606985ba39c06e5c))
* implement use of asfaload index files ([8b4bad4](https://github.com/asfaload/asfald/commit/8b4bad47c01405bd6d4c5d34c9ca82a7854efffc))
* return Err if usable checksum not found ([3a9c016](https://github.com/asfaload/asfald/commit/3a9c0162e590a53d4dbfc30597a3db8c70c62081))
* return struct and not only hash value when looking for a file's hash ([9d7c1b7](https://github.com/asfaload/asfald/commit/9d7c1b7c3c0e85c018509893e2a83d07fde341b9))
* use asfaload indexes by default ([fc64e10](https://github.com/asfaload/asfald/commit/fc64e103330408d6cd4defad23faff9ff6afa73b))


### Bug Fixes

* index file is not hidden ([ff24cb6](https://github.com/asfaload/asfald/commit/ff24cb6f6f8fac772c8f57ff76ab9f069b44cfac))

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
