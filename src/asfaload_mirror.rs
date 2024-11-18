use anyhow::Context;
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;

pub static ASFALOAD_HOSTS: Lazy<Vec<AsfaloadHost<'_>>> = Lazy::new(|| {
    vec![
        AsfaloadHost {
            host: "gh.checksums.asfaload.com",
            prefix: None,
        },
        // Disable cloudflare mirror as it doesn't build sites with more than 20000 files
        //AsfaloadHost {
        //    host: "cf.checksums.asfaload.com",
        //    prefix: None,
        //},
    ]
});

#[derive(Clone)]
pub struct AsfaloadHost<'a> {
    // Host on which our checksums are available, eg asfaload.github.io
    pub host: &'a str,
    // The prefix to add to the path to the checksums file compared to the original path, eg
    // /checksums
    pub prefix: Option<&'a str>,
}

pub fn choose<'a>() -> &'a AsfaloadHost<'a> {
    ASFALOAD_HOSTS.choose(&mut rand::thread_rng()).unwrap()
}
pub fn path_on_mirror(host: &AsfaloadHost<'_>, url: &url::Url) -> String {
    host
    // Tke the mirror's prefix
    .prefix
    // Put the `/` in front of it
    .map(|p| p.to_string() + "/")
    // And get it out of the option, or the empty string
    .unwrap_or_default()
    // Put the host in the path
    + &url.host().unwrap().to_string()
    // Followed by the full original path
    + url.path()
}

pub fn url_on_mirror(host: &AsfaloadHost<'_>, url: &url::Url) -> url::Url {
    let path = path_on_mirror(host, url);
    url::Url::parse(format!("https://{}/{}", host.host, path).as_str())
        .context("Problem constructing url on mirror")
        .unwrap()
}

#[cfg(test)]
mod asfaload_mirror_tests {

    use anyhow::Result;

    use super::*;

    #[test]
    fn test_path_on_mirror() -> Result<()> {
        let download_url = url::Url::parse("https://github.com/asfaload/asfald/releases/download/v0.2.0/asfald-x86_64-unknown-linux-musl.tar.gz")?;
        let expected_on_mirror = "github.com/asfaload/asfald/releases/download/v0.2.0/asfald-x86_64-unknown-linux-musl.tar.gz";
        let host = ASFALOAD_HOSTS.first().unwrap();
        let mirror_path = path_on_mirror(host, &download_url);
        assert_eq!(mirror_path, expected_on_mirror);
        Ok(())
    }

    #[test]
    fn test_url_on_mirror() -> Result<()> {
        let download_url = url::Url::parse("https://github.com/asfaload/asfald/releases/download/v0.2.0/asfald-x86_64-unknown-linux-musl.tar.gz")?;
        let expected_on_mirror = "https://gh.checksums.asfaload.com/github.com/asfaload/asfald/releases/download/v0.2.0/asfald-x86_64-unknown-linux-musl.tar.gz";
        let host = ASFALOAD_HOSTS.first().unwrap();
        let mirror_url = url_on_mirror(host, &download_url);
        assert_eq!(mirror_url.to_string(), expected_on_mirror);
        Ok(())
    }
}
