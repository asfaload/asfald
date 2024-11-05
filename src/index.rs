// Code to user asfaload index files
#[allow(dead_code)]

const INDEX_NAME: &str = ".asfaload.index.json";

pub fn index_for(url: &url::Url) -> url::Url {
    let mirror_host = asfaload_mirror::choose();
    let on_mirror = asfalod_mirror::url_on_mirror(mirror_host, url);
    on_mirror.join(INDEX_NAME).unwrap()
}

#[cfg(test)]
mod asfaload_index_tests {

    use super::*;
    use anyhow::Result;

    fn test_index_for() -> Result<()> {
        let download_url = url::Url::parse("https://github.com/asfaload/asfald/releases/download/v0.2.0/asfald-x86_64-unknown-linux-musl.tar.gz")?;
        let expected_index = "https://gh.checksums.asfaload.com/github.com/asfaload/asfald/releases/download/v0.2.0/.asfaload.index.json";
        let host = ASFALOAD_HOSTS.first().unwrap();
        let mirror_url = url_on_mirror(host, &download_url);
        assert_eq!(mirror_url.to_string(), expected_index);
        Ok(())
    }
}
