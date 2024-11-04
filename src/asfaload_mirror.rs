// code regarding our checksums mirrors
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
pub static ASFALOAD_HOSTS: Lazy<Vec<AsfaloadHost<'_>>> = Lazy::new(|| {
    vec![
        AsfaloadHost {
            host: "gh.checksums.asfaload.com",
            prefix: None,
        },
        AsfaloadHost {
            host: "cf.checksums.asfaload.com",
            prefix: None,
        },
    ]
});
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
