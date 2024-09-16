# About

`asfd` is a command line downloader which validates the integrity of the downloaded file against a checksums file published alongside it.

By default it looks for a checksums file alongside the file to be downloaded. The names it is looking for are `checksum.txt`, `checksums.txt`, `CHECKSUMS256.txt`, `SHASUMS256`, `SHASUMS256.txt`. It also checks for checksums in a file named identically to the file to be downloaded, but with extension `.sha256sum` (the [neovim releases](https://github.com/neovim/neovim/releases) convention).

If the checksum file to be used is not located alongside the file to be downloaded, you can specify custom locations with the `-p` flag. The custom location you provide can even be on another server, allowing you to maintain your own checksums files internally. Call `asfd` with the `--help` flag to get more info and an example.

Publishing a checksums file alongisde a file proposed for download is done by numerous projects in their releases (see for example [Lazygit](https://github.com/jesseduffield/lazydocker), [mise](https://github.com/jdx/mise), [watchexec](https://github.com/watchexec/watchexec), [Github's CLI](https://github.com/cli/cli/), [act](https://github.com/nektos/act/releases/tag/v0.2.66)(run Github Actions locally),[neovim](https://github.com/neovim/neovim), ...).

You can get `asfd` by downloading it from its Github releases (see below how to secure the initial download) or by building it yourself.

# Downloading asfd

`asfd` automates the validation of checksums of downloaded files, but when your are downloading `asfd` for the first time you need to do it manually.

To increase your security, we also publish the checksums files of releases on our website (in two locations, see below if interested), which we will use here.

So, after you download asfd for the first time, we encourage you to validate it. On Linux and MacOS, you can do it with the following command to be run in the directory in which you downloaded the file:
```
sha256sum --ignore-missing -c <(curl --silent  http://asfaload.com/releases-checksums/v0.0.1.txt)
```
You should get an output of the form
```
asfd-x86_64-apple-darwin.tar.gz: OK
```
Subsequent downloads can be done with `asfd` itself. For example when downloading the FreeBSD version of `asfd`, you get this output:
```
$ asfd https://github.com/asfaload/asfd/releases/download/v0.0.1/asfd-x86_64-unknown-freebsd.tar.gz
INFO 🔍 Looking for checksum files...
INFO ✨ Checksum file found !
INFO 🗑️ Create temporary file...
INFO 🚚 Downloading file...
  [00:00:00] [##################################################] 2.15 MiB/2.15 MiB (00:00:00)
INFO ✅ File's checksum is valid !
```

# Release checksums files locations

We publish checksums files for `asfd` releases in our Github releases, under the name `checksums.txt`

We also publish the checkums files on our website.
The first location has the same url as the checksums file in the Github release, but you replace `github` by `asfaload` in the hostname, for example
```
 https://asfaload.com/asfaload/asfd/releases/tag/v0.0.1/checksums.txt
```

The second location is under the  `asfd-releases` directory, with the checksums file named as the release version. For example, for version `v0.0.1`, the checksums file is:
```
https://asfaload.com/asfd-checksums/v0.0.1
```

Publishing the checkums files in another location increases security as malevolent actors now have 2 locations to compromise before they trick you in downloading erroneous software without you noticing.

# Building

## Dynamic linking

You can build a dynamically linked version with these pre-requisites(these instructions were tested on Linux):

* Rust toolchain, see for example [Rustup.rs](https://rustup.rs/)
* pkg-config ([how to install](https://command-not-found.com/pkg-config))
* OpenSSL lib and headers ([how to install](https://docs.rs/openssl/0.10.16/openssl/#automatic))

and then run `cargo build` (this uses the `dev` [profile](https://doc.rust-lang.org/cargo/reference/profiles.html)). The binary can then be found in `target/debug/asfd`

## Static linking on Linux

We provide a convenient Dockerfile to build a static binary. You can just run `make linux-static` and it will build a Docker image named `asfd-build` based on the official Rust Alpine container image. It will then use that image to build a static binary. You can choose the build [profile](https://doc.rust-lang.org/cargo/reference/profiles.html) with the `PROFILE` variable, eg `PROFILE=release make linux-static` and the binary will then be found at `target/debug/asfd`.
