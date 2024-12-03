use anyhow::Context;
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;

#[derive(Clone)]
pub enum MirrorProtocol {
    Https,
    // We only allow the http protocol for tests.
    // This variant is not available in code not running in tests
    #[cfg(any(test, feature = "testing"))]
    Http,
}

impl std::fmt::Display for MirrorProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let s = match self {
            MirrorProtocol::Https => "https",
            // As the Http variant is only available in tests, we mark this
            // branch of the match as only compiled in tests
            #[cfg(any(test, feature = "testing"))]
            MirrorProtocol::Http => "http",
        };
        write!(f, "{}", s)
    }
}

// This is the definition of asfaload hosts in production. It should not be
// available in tests.
// Note that I get the warning "code is inactive due to #[cfg] directives: test is enabled"
// because the rust analyzer needs to enable tests IIUC.
// This code is only available when compiling asfald outside of tests.
// Not setting the cfg prevents compiling and achieving our goal to have different hosts in tests
// as well as preventing the use of the HTTP protocol in production.
#[cfg(not(any(test, feature = "testing")))]
pub static ASFALOAD_HOSTS: Lazy<Vec<AsfaloadHost<'_>>> = Lazy::new(|| {
    vec![
        AsfaloadHost {
            protocol: MirrorProtocol::Https,
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

// This is the definition of asfaload hosts for tests, allowing to test all functionality
// against a test-specific mirror on localhost
#[cfg(any(test, feature = "testing"))]
pub static ASFALOAD_HOSTS: Lazy<Vec<AsfaloadHost<'_>>> = Lazy::new(|| {
    vec![
        AsfaloadHost {
            protocol: MirrorProtocol::Http,
            host: "localhost:9898",
            prefix: None,
        },
        AsfaloadHost {
            protocol: MirrorProtocol::Http,
            host: "localhost:9899",
            prefix: None,
        },
    ]
});

#[derive(Clone)]
pub struct AsfaloadHost<'a> {
    pub protocol: MirrorProtocol,
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
    url::Url::parse(format!("{}://{}/{}", host.protocol, host.host, path).as_str())
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
        let expected_on_mirror = "http://localhost:9898/github.com/asfaload/asfald/releases/download/v0.2.0/asfald-x86_64-unknown-linux-musl.tar.gz";
        let host = ASFALOAD_HOSTS.first().unwrap();
        let mirror_url = url_on_mirror(host, &download_url);
        assert_eq!(mirror_url.to_string(), expected_on_mirror);
        Ok(())
    }
}
