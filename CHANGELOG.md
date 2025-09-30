# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.1](https://github.com/asfaload/asfald/compare/v0.7.0...v0.5.1) (2025-09-30)


### Features

* accept hash value on command line ([4093382](https://github.com/asfaload/asfald/commit/40933825af60aef62cabe5f08be14a50e765d54b))
* add -p / --pattern argument ([36829fd](https://github.com/asfaload/asfald/commit/36829fd4c49a9a6da8e4112b3d3c15f75aa178f8))
* add -q argument ([5e55fe7](https://github.com/asfaload/asfald/commit/5e55fe7cd0021dfdf2d5ae337930e696eba9bd3a))
* add examples section to --help ([e21335c](https://github.com/asfaload/asfald/commit/e21335c0b0b2a9e08c577abb3d7b2516ae5eb719))
* add extensible changelog patterns ([18d19bb](https://github.com/asfaload/asfald/commit/18d19bbd9b062a6097c1bb4c76d356ee681e3d44))
* add extensible checkums files patterns ([c338d5f](https://github.com/asfaload/asfald/commit/c338d5f77ddfe196d2aba706e9f51356a0762b4e))
* add flag to overwrite existing files ([ebb8589](https://github.com/asfaload/asfald/commit/ebb8589c28fe05c0b2c423d95c9005ae220af857))
* add fullpath ([23e3c39](https://github.com/asfaload/asfald/commit/23e3c39b0d801184e4f84a0b7e13927a93618d31))
* add get_all_hashes ([96b6976](https://github.com/asfaload/asfald/commit/96b697627522a1ff43bd8aa29e42731dae6dd701))
* add get_best_hash_for_file ([8e3c969](https://github.com/asfaload/asfald/commit/8e3c9698e9ec5638aaf1023c49579a405253ad8d))
* add lib parsing asfaload index files ([6312d8e](https://github.com/asfaload/asfald/commit/6312d8e7df58e76a5592c2a6408650326a1084b2))
* add pre-commit support ([53b3bcf](https://github.com/asfaload/asfald/commit/53b3bcf6c7ca2c58ac2b2c1706d743a24943a1a4))
* also compare to checksum published in release ([0a8c859](https://github.com/asfaload/asfald/commit/0a8c859dbefbc47bc93091e8e8f2aaff165a4b01))
* download checksums file from our mirror when flag -a used ([759edca](https://github.com/asfaload/asfald/commit/759edcac64327fd7a67be4a00cdfbe19392a0975))
* enable lto in release build ([f53a637](https://github.com/asfaload/asfald/commit/f53a637604faf7fdd6245473c85536835b101130))
* enable lto in release build ([9e39df1](https://github.com/asfaload/asfald/commit/9e39df1e80a98a1dfd76ff727998a3398faeb7cf))
* get_all_checksums indicates consistency with enum case ([2a30e26](https://github.com/asfaload/asfald/commit/2a30e26f776350f8b12845f5606985ba39c06e5c))
* implement use of asfaload index files ([8b4bad4](https://github.com/asfaload/asfald/commit/8b4bad47c01405bd6d4c5d34c9ca82a7854efffc))
* potentially support multiple mirrors ([c922aaf](https://github.com/asfaload/asfald/commit/c922aafd55d77a5c66731bc10391227ab05c625a))
* print vulnerability window of index ([158048c](https://github.com/asfaload/asfald/commit/158048c573225d254b50e5a6d0d075cf3641fa73))
* print vulnerability window of index ([640e316](https://github.com/asfaload/asfald/commit/640e3164e565384004c62a75d30a2e841fc7a33d))
* refuse to overwrite existing file ([ac4b00c](https://github.com/asfaload/asfald/commit/ac4b00cd535d97391fe1ae540a59d86c2c266827))
* report host that served the checksums file ([2698a5b](https://github.com/asfaload/asfald/commit/2698a5b3a7c044d2bde5d66699c989824bb524df))
* return Err if usable checksum not found ([3a9c016](https://github.com/asfaload/asfald/commit/3a9c0162e590a53d4dbfc30597a3db8c70c62081))
* return struct and not only hash value when looking for a file's hash ([9d7c1b7](https://github.com/asfaload/asfald/commit/9d7c1b7c3c0e85c018509893e2a83d07fde341b9))
* send to stdout if filename is - ([f05b2c4](https://github.com/asfaload/asfald/commit/f05b2c402608bb21af9000dadc4c1e3e7c71d617))
* split force flag in absent and invalid cases ([7375255](https://github.com/asfaload/asfald/commit/73752559f14c12a28e16bad2b5d084b8690c9d3f))
* split force flag in absent and invalid cases ([bcc6bb2](https://github.com/asfaload/asfald/commit/bcc6bb28e41cb5f08b5f2c99e8ccf1206ca3e45e))
* support cloudflare pages ([300b5d0](https://github.com/asfaload/asfald/commit/300b5d0cd265c59ac9fba52b732697776500d6ad))
* use asfaload indexes by default ([fc64e10](https://github.com/asfaload/asfald/commit/fc64e103330408d6cd4defad23faff9ff6afa73b))
* use github published checksums ([fb3c30a](https://github.com/asfaload/asfald/commit/fb3c30a6e36180953bde485d43d9c47e43980403))
* use github published checksums ([b83815d](https://github.com/asfaload/asfald/commit/b83815d093069070552e4317ff6b60a64ffc8eac))


### Bug Fixes

* add --version flag ([c564ac6](https://github.com/asfaload/asfald/commit/c564ac60419162d4b54336987695395a5f872dc4))
* add --version flag ([93e1982](https://github.com/asfaload/asfald/commit/93e19828c56e4f87f84a3a58df79b1352ffce619))
* compress release with zip for windows ([0974781](https://github.com/asfaload/asfald/commit/097478180a93d55d3b6be8cb3ca323351b3b5d3c))
* compress release with zip for windows ([c853e78](https://github.com/asfaload/asfald/commit/c853e78ce3de982d48cf007d0e7ac92a08aec123))
* consider url's port in path to index ([97bc937](https://github.com/asfaload/asfald/commit/97bc937c83b8c9cc85c895f061f664c120aae479))
* defining the CNAME to gh removes the prfix in the url ([df93ed4](https://github.com/asfaload/asfald/commit/df93ed4f0965c43028c2ca8d0f262148a58f0bfc))
* disable cloudflare pages mirror ([4f43ca8](https://github.com/asfaload/asfald/commit/4f43ca83fa7cae10b29a4081e85380285dc88426))
* disable cloudflare pages mirror ([18b4832](https://github.com/asfaload/asfald/commit/18b4832a06cd418344c63912acf2cf6fcb5deebc))
* do not support unrecommended hashing algos ([ab38644](https://github.com/asfaload/asfald/commit/ab386442fc74706a0139a00542885c69ab889b71))
* do not support unrecommended hashing algos ([6a665e9](https://github.com/asfaload/asfald/commit/6a665e90e4c5004841b1ae91dad6230cc25a9a9d))
* exit process with error code if error occured ([cb5d67f](https://github.com/asfaload/asfald/commit/cb5d67f8fd453c91ce3d935025305e5b8cb2b4bd))
* explicitly write out lifetime ([533d8f8](https://github.com/asfaload/asfald/commit/533d8f883f4fc883bbf66ee8a396a6a3141982cb))
* handle binary file marker in shasum files ([6983c86](https://github.com/asfaload/asfald/commit/6983c8689106f2c4749024e841ac701c02c02fa8))
* handle remote checksums ([94e42db](https://github.com/asfaload/asfald/commit/94e42db4add5d491a9d56042066cb96170cb97cd))
* if remote checksums pattern is not parsed as a url, used it as path ([3d0182d](https://github.com/asfaload/asfald/commit/3d0182daf6362297f1180fc6863402d9d58c0268))
* ignore paths inside checkums files ([08971e7](https://github.com/asfaload/asfald/commit/08971e7789b8fe68096ff49f1acf5b0f4753ec04))
* ignore paths inside checkums files ([84d4a8f](https://github.com/asfaload/asfald/commit/84d4a8f3b138c808b9613e36a1d14cef49228afa))
* index file is not hidden ([ff24cb6](https://github.com/asfaload/asfald/commit/ff24cb6f6f8fac772c8f57ff76ab9f069b44cfac))
* logger prints all messages to stderr ([2cc16ba](https://github.com/asfaload/asfald/commit/2cc16ba3c4eb2247040a3e0a6cf2d70e51d87363))
* make linux-static ([5a821e1](https://github.com/asfaload/asfald/commit/5a821e12f718856a66fcf43a339580cc3e1b5921))
* make linux-static ([f2c48a3](https://github.com/asfaload/asfald/commit/f2c48a37979594519668f3b47a64c24255dcea9b))
* make sure to hide the progress bar in quiet mode ([e8a8c50](https://github.com/asfaload/asfald/commit/e8a8c50b574508288b410f76ab46df053b1bb5d8))
* remove error-causing space in https url scheme string comparison ([4bb8ab3](https://github.com/asfaload/asfald/commit/4bb8ab3231e6553a7516beffe6161f3bc28313dc))
* set correct name in code ([97f5ea7](https://github.com/asfaload/asfald/commit/97f5ea772d9d2342e4bf0b95bf435a74e3d61c2d))
* start file transfer before index download to report inexisting file first ([0313083](https://github.com/asfaload/asfald/commit/03130838b25cd10d3b597dacd7f39cddc9062211))
* the path passed to -p is always considered from the root ([48b6a1e](https://github.com/asfaload/asfald/commit/48b6a1ecbafdb936319409e2dcbc4c745b374585))
* unusable checkums in release are logged ([eca99cc](https://github.com/asfaload/asfald/commit/eca99ccc1b70c2cf8f3e5a59aa332e857eb6ca07))
* unusable checkums in release are logged ([9cd27f1](https://github.com/asfaload/asfald/commit/9cd27f155c163f51cfc9a894f4b58d20d0c75ad2))
* update Cargo.lock after rebase ([e1c0315](https://github.com/asfaload/asfald/commit/e1c0315ae702a0f133f800b15709e4ac45b57769))


### Tests

* file renamed in release after mirror taken ([d2b5c01](https://github.com/asfaload/asfald/commit/d2b5c01c3bbe32b2d01e1db394b63c0284ec5402))

## [0.7.0](https://github.com/asfaload/asfald/compare/v0.6.0...v0.7.0) (2025-09-30)


### Features

* use github published checksums ([fb3c30a](https://github.com/asfaload/asfald/commit/fb3c30a6e36180953bde485d43d9c47e43980403))
* use github published checksums ([b83815d](https://github.com/asfaload/asfald/commit/b83815d093069070552e4317ff6b60a64ffc8eac))

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
