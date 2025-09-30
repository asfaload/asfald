# About

`asfald` is a command line downloader for Github Release files  which validates the integrity of the downloaded file against the checksum published by Github.



# Downloading asfald

`asfald` automates the validation of checksums of downloaded files, but when your are downloading `asfald` for the first time you need to do it manually.
You can check it manually or more easily using our checksums mirror.

## Checking manually

Download the file from the github release, then run the command `sha256sum` passing as argument the path to the file you just downloaded.
The value printed should be the same as the one displayed by github on the release page.

## Using our checksums mirror

To increase your security, we also publish the checksums files of `asfald` releases on our checksums mirror.
But it also makes it easier to automate the check.

On Linux and MacOS, you can do it with the following command to be run in the directory in which you downloaded the file:

```
curl -L -O https://github.com/asfaload/asfald/releases/download/v0.8.0/asfald-x86_64-unknown-linux-musl
sha256sum --ignore-missing -c <(curl --silent  https://gh.checksums.asfaload.com/github.com/asfaload/asfald/releases/download/v0.8.0/checksums.txt)
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
Subsequent downloads can be done with `asfald` itself.

# Using asfald

## On the command line
As seen above, using `asfald` is easy: just call it with the file to be downloaded's URL as argument. For example:
```
asfald https://github.com/asfaload/asfald/releases/download/v0.8.0/asfald-x86_64-unknown-freebsd.tar.gz
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

You can safely download and install `asfald` in your linux containers by adding this snippet to your `Dockerfile` (you can choose the version to install by modifying the value of `asfald_version` on the first line), the only requirement is to have `curl` installed:
```
RUN bash -c 'asfald_version=v0.8.0 && \
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
RUN bash -c 'asfald_version=v0.8.0 && \
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
