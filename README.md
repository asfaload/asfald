# About

`asfd` is a command line downloader which validates the integrity of the downloaded file against a checksums file published alongside it.

Note that **using a checksums file stored on the same server as the downloaded file has no security benefit**, it only ensure the file you have wasn't corrupted in transit.
To increase security, the checksums file has to be downloaded from another server. For example, when downloading from a mirror, you can use the checksums file from the origin server. For other security-enhancing possibilities offered by asfd's checksums checking features, check [this blog post](https://www.asfaload.com/blog/increasing-security-checking-integrity/).

By default it looks for a checksums file alongside the file to be downloaded. The names it is looking for are `checksum.txt`, `checksums.txt`, `CHECKSUMS256.txt`, `SHASUMS256`, `SHASUMS256.txt`. It also checks for checksums in a file named identically to the file to be downloaded, but with extension `.sha256sum` (the [neovim releases](https://github.com/neovim/neovim/releases) convention).

If the checksum file to be used is not located alongside the file to be downloaded, you can specify custom locations with the `-p` flag. The custom location you provide can be on another server, allowing you to maintain your own checksums files internally. Call `asfd` with the `--help` flag to get more info and an example.

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
# Using asfd

## On the command line
Using `asfd` is easy: just call it with the file to be downloaded's URL as argument. For example:
```
asfd https://github.com/asfaload/asfd/releases/download/v0.0.1/asfd-x86_64-unknown-freebsd.tar.gz
```

If the checksum could not be validated, the execution exits with a non-zero status. This makes `asfd` usable in script, especially when combined with the `--quiet` flag.


```bash
lazygit_url="https://github.com/jesseduffield/lazygit/releases/download/v0.44.0/lazygit_0.44.0_freebsd_arm64.tar.gz"
if asfd -q  "$lazygit_url"; then
  echo "proceeding with lazygit install";
else
  echo "problem getting lazygit";
fi
```

## In Dockerfiles

You can safely download and install `asfd` in your linux containers by adding this snippet to your `Dockerfile` (you can choose the version to install by modifying the value of `asfd_version` on the first line):
```
RUN bash -c 'asfd_version=v0.1.0 && \
    curl --silent  -L -O https://github.com/asfaload/asfd/releases/download/${asfd_version}/asfd-x86_64-unknown-linux-musl && \
    sha256sum --ignore-missing -c <(curl --silent  https://asfaload.com/asfd-checksums/${asfd_version}) && \
    mv asfd-x86_64-unknown-linux-musl /usr/bin/asfd && chmod +x /usr/bin/asfd'
```

This will download `asfd` from Github, validate the file's checksum against the checksums published on [asfaload.com](http://www.asfaload.com/asfd-checksums) and if successful, place the `asfd` binary in the container's `/usr/bin` directory.

<details>
<summary>
Example: a full `Dockerfile` letting you run `asfd` in a container
</summary>

```
FROM ubuntu

RUN apt-get update && apt-get install -y curl
RUN bash -c 'asfd_version=v0.1.0 && \
    curl --silent  -L -O https://github.com/asfaload/asfd/releases/download/${asfd_version}/asfd-x86_64-unknown-linux-musl && \
    sha256sum --ignore-missing -c <(curl --silent  https://asfaload.com/asfd-checksums/${asfd_version}) && \
    mv asfd-x86_64-unknown-linux-musl /usr/bin/asfd && chmod +x /usr/bin/asfd'

ENTRYPOINT [ "/usr/bin/asfd" ]
```
Using the image built with this `Dockerfile`, you can display the help of `asfd` with
```
docker run -it --rm 0f8748 --help
```

</details>


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

# Contributing

We welcome contributions of all kinds! See our [blog port](https://www.asfaload.com/blog/handling-outside-contributions/) on the subject.

To ensure code quality and consistency, this project uses [pre-commit](https://pre-commit.com/) hooks to automatically check and format code before it's committed.

To install pre-commit:

```console
$ pip install pre-commit
```

To install the necessary pre-commit hooks for this repository, run the following commands:

```console
$ pre-commit install
$ pre-commit install --hook-type commit-msg
```

If you'd like to run the pre-commit checks manually before committing, use:

```console
$ pre-commit run -a
```
