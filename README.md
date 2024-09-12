# About

`asfd` is a command line downloader which validates the integrity of the downloaded file against a checksums file published alongside it.
This is done by numerous projects in their releases (see for example [Lazygit](https://github.com/jesseduffield/lazydocker), [mise](https://github.com/jdx/mise), [watchexec](https://github.com/watchexec/watchexec), [Github's CLI](https://github.com/cli/cli/), [act](https://github.com/nektos/act/releases/tag/v0.2.66)(run Github Actions locally),[neovim](https://github.com/neovim/neovim), ...).

The tool recognises different naming conventions ([CHECKSUM.txt, checksums.txt,....](https://github.com/asfaload/asfd/blob/main/src/main.rs#L15)) but also takes a flag to manually the checksums file name.

# Building

## Dynamic linking on Linux
You can build a dynamically linked version on Linux with these pre-requisites:

* Rust toolchain, see for example [Rustup.rs](https://rustup.rs/)
* pkg-config ([how to install](https://command-not-found.com/pkg-config))
* OpenSSL lib and headers ([how to install](https://docs.rs/openssl/0.10.16/openssl/#automatic))

and then run `cargo build` (this uses the `dev` [profile](https://doc.rust-lang.org/cargo/reference/profiles.html)). The binary can then be found in `target/debug/asfd`

## Static linking on Linux

We provide a convenient Dockerfile to build a static binary. You can just run `make linux-static` and it will build a Docker image named `asfd-build` based on the official Rust Alpine container image. It will then use that image to build a static binary. You can choose the build [profile](https://doc.rust-lang.org/cargo/reference/profiles.html) with the `PROFILE` variable, eg `PROFILE=release make linux-static` and the binary will then be found at `target/debug/asfd`.
