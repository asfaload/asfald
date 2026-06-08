// Backend url
// -----------
#[derive(clap::Args, Debug)]
pub struct BackendUrlArgs {
    /// Backend API URL (optional, defaults to DEFAULT_BACKEND)
    #[arg(short = 'u', long)]
    pub backend_url: Option<String>,
}

// Forge Type
// ----------

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ForgeType {
    Github,
    Gitlab,
    Fileserver,
}

impl ForgeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ForgeType::Github => "github",
            ForgeType::Gitlab => "gitlab",
            ForgeType::Fileserver => "fileserver",
        }
    }
}

#[derive(clap::Args, Debug)]
pub struct ForgeTypeArgs {
    /// Override forge type detection.
    #[arg(long, short = 'F')]
    pub forge_type: Option<ForgeType>,
}
