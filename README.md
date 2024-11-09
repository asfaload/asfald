# About

`asfald` is a command line downloader which validates the integrity of the downloaded file against a checksums file mirrored in a separate git repository (https://github.com/asfaload/checksums).
Currently the checksum file is mirrored as soon as possible for Github repositories watched by Asfaload, but very soon you will be able to notify of the publication of a new release with checksum file, for other hosting solutions ([Gitlab](https://gitlab.com), [Forgejo](https://forgejo.org/), http).


Publishing a checksums file alongisde a file proposed for download is done by numerous projects in their releases (see for example [Lazygit](https://github.com/jesseduffield/lazydocker), [mise](https://github.com/jdx/mise), [watchexec](https://github.com/watchexec/watchexec), [Github's CLI](https://github.com/cli/cli/), [act](https://github.com/nektos/act/releases/tag/v0.2.66)(run Github Actions locally),[neovim](https://github.com/neovim/neovim), ...).

Note however that **using a checksums file stored on the same server as the downloaded file has no security benefit**, it only ensure the file you have wasn't corrupted in transit.
Using a checksum file on a mirror increases security as two locations have to be compromised to serve you malevolent content.

We are already mirroring checksum files for more than 1200 projects.


You can get `asfald` by downloading it from its Github releases (see below how to secure the initial download) or by building it yourself.

# Downloading asfald

`asfald` automates the validation of checksums of downloaded files, but when your are downloading `asfald` for the first time you need to do it manually.

To increase your security, we also publish the checksums files of `asfald` releases on our checksums mirror.

So, after you download asfald for the first time, we encourage you to validate it. On Linux and MacOS, you can do it with the following command to be run in the directory in which you downloaded the file:
```
curl -L -O https://github.com/asfaload/asfald/releases/download/v0.3.0/asfald-x86_64-unknown-linux-musl
sha256sum --ignore-missing -c <(curl --silent  https://gh.checksums.asfaload.com/github.com/asfaload/asfald/releases/download/v0.3.0/checksums.txt)
```
You should get an output of the form
```
asfald-x86_64-unknown-linux-musl: OK

```
You can then move the file to your path and make it executable.
```
mv asfald-x86_64-unknown-linux-musl ~/local/bin/asfald
chmod +x ~/local/bin/asfald
```
Subsequent downloads can be done with `asfald` itself. For example when downloading the FreeBSD version of `asfald`, you get this output:
```
asfald https://github.com/asfaload/asfald/releases/download/v0.3.0/asfald-x86_64-unknown-freebsd.tar.gz
INFO ℹ️ Using asfaload index on mirror
INFO 🗑️ Create temporary file...
INFO 🚚 Downloading file...
  [00:00:00] [###################################################################################################################################################] 1.90 MiB/1.90 MiB (00:00:00)INFO ✅ File's checksum is valid !
```
# Using asfald

## On the command line
Using `asfald` is easy: just call it with the file to be downloaded's URL as argument. For example:
```
asfald https://github.com/asfaload/asfald/releases/download/v0.3.0/asfald-x86_64-unknown-freebsd.tar.gz
```

If the checksum could not be validated, the execution exits with a non-zero status. This makes `asfald` usable in script, especially when combined with the `--quiet` flag.


```bash
lazygit_url="https://github.com/jesseduffield/lazygit/releases/download/v0.44.0/lazygit_0.44.0_freebsd_arm64.tar.gz"
if asfald -q  "$lazygit_url"; then
  echo "proceeding with lazygit install";
else
  echo "problem getting lazygit";
fi
```

## In Dockerfiles

You can safely download and install `asfald` in your linux containers by adding this snippet to your `Dockerfile` (you can choose the version to install by modifying the value of `asfald_version` on the first line), the only requirement is to have `curl` installed available:
```
RUN bash -c 'asfald_version=v0.3.0 && \
    curl --silent  -L -O https://github.com/asfaload/asfald/releases/download/${asfald_version}/asfald-x86_64-unknown-linux-musl && \
    sha256sum --ignore-missing -c <(curl --silent https://gh.checksums.asfaload.com/github.com/asfaload/asfald/releases/download/${asfald_version}/checksums.txt ) && \
    mv asfald-x86_64-unknown-linux-musl /usr/bin/asfald && chmod +x /usr/bin/asfald'
```

This will download `asfald` from Github, validate the file's checksum against the checksums published on [asfaload.com](http://www.asfaload.com/asfald-checksums) and if successful, place the `asfald` binary in the container's `/usr/bin` directory.

<details>
<summary>
Example: a full `Dockerfile` letting you run `asfald` in a container
</summary>

```
FROM ubuntu

RUN apt-get update && apt-get install -y curl
RUN bash -c 'asfald_version=v0.3.0 && \
    curl --silent  -L -O https://github.com/asfaload/asfald/releases/download/${asfald_version}/asfald-x86_64-unknown-linux-musl && \
    sha256sum --ignore-missing -c <(curl --silent https://gh.checksums.asfaload.com/github.com/asfaload/asfald/releases/download/${asfald_version}/checksums.txt ) && \
    mv asfald-x86_64-unknown-linux-musl /usr/bin/asfald && chmod +x /usr/bin/asfald'

ENTRYPOINT [ "/usr/bin/asfald" ]
```
Using the image built with this `Dockerfile`, you can display the help of `asfald` with
```
docker run -it --rm 0f8748 --help
```

</details>


# Asfald's inner working

We collect checksums files in our repo at https://github.com/asfaload/checksums, which is used to publish a Github pages site at https://gh.checksums.asfaload.com and a Cloudflare pages site at https://cf.checksums.asfaload.com. The path to the checksums file is the same as its original url, without the scheme part(i.e. without `https://`).
For example, the original URL for the checksums file of asfald's v0.3.0 release, is at https://github.com/asfaload/asfald/releases/download/v0.3.0/checksums.txt, and its location on the mirror is https://gh.checksums.asfaload.com/github.com/asfaload/asfald/releases/download/v0.3.0/checksums.txt.

There's no accepted standard for naming the checksums file. For example, when downloading a neovim release with url `https://github.com/neovim/neovim/releases/download/v0.10.2/nvim-linux64.tar.gz`, the checkum file is named ` nvim-linux64.tar.gz.sha256sum`, when downloading a mise release from `https://github.com/jdx/mise/releases/tag/v2024.11.4`, the checksums file name is `SHASUMS256.txt`.

This makes it harder and more inefficient than necessary for asfald to find the expected hash of the downloaded file.
We don't want asfald to try all naming conventions until it finds a match. That's why we also create the file `asfaload.index.json` on the mirror, which holds all information of all checksums file found at the download location, and additional information like the time at which the mirror was taken.

Here is the top of that file for asfald's v0.3.0 release:
```
{
  "mirroredOn": "2024-11-08T15:50:17.5034034+00:00",
  "publishedOn": "2024-11-08T14:01:08+00:00",
  "version": 1,
  "publishedFiles": [
    {
      "fileName": "asfald-aarch64-apple-darwin",
      "algo": "Sha256",
      "source": "checksums.txt",
      "hash": "b2ad8f03807b15335dd2af367b55d6318ffe46d32462e514c272272c9aeba130"
    },
    {
      "fileName": "asfald-aarch64-apple-darwin.tar.gz",
      "algo": "Sha256",
      "source": "checksums.txt",
      "hash": "6c1cba9e7da41f9c047bd7ee58f2015fe7efc3b45c3b57c67f19ebf69629d5d1"
    },

```

This information makes the checksums mirror fully auditable.

# Building

## Dynamic linking

You can build a dynamically linked version with these pre-requisites(these instructions were tested on Linux):

* Rust toolchain, see for example [Rustup.rs](https://rustup.rs/)
* pkg-config ([how to install](https://command-not-found.com/pkg-config))
* OpenSSL lib and headers ([how to install](https://docs.rs/openssl/0.10.16/openssl/#automatic))

and then run `cargo build` (this uses the `dev` [profile](https://doc.rust-lang.org/cargo/reference/profiles.html)). The binary can then be found in `target/debug/asfald`

## Static linking on Linux

We provide a convenient Dockerfile to build a static binary. You can just run `make linux-static` and it will build a Docker image named `asfald-build` based on the official Rust Alpine container image. It will then use that image to build a static binary. You can choose the build [profile](https://doc.rust-lang.org/cargo/reference/profiles.html) with the `PROFILE` variable, eg `PROFILE=release make linux-static` and the binary will then be found at `target/debug/asfald`.

## Github Action

The `asfald` github repository also includes a Github Action to build all executables at https://github.com/asfaload/asfald/actions/workflows/build.yml, which you can manually trigger (eg on your fork of `asfald`'s github repo).

# Contributing

We welcome contributions of all kinds! See our [blog post](https://www.asfaload.com/blog/handling-outside-contributions/) on the subject.

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
