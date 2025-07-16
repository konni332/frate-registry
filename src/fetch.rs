use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitHubRepo {
    pub(crate) name: String,
    pub(crate) full_name: String,
    pub(crate) description: Option<String>,
    pub(crate) default_branch: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
    pub(crate) tag_name: String,
    pub(crate) assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubAsset {
    pub(crate) name: String,
    pub(crate) browser_download_url: String,
}

pub fn fetch_releases(full_name: &str) -> Result<Vec<GitHubRelease>, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/releases", full_name);
    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "frate-registry-gen")
        .send()?
        .error_for_status()?;
    let releases: Vec<GitHubRelease> = response.json()?;
    Ok(releases)
}

pub fn get_repo_info(repo: &str) -> Result<GitHubRepo, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}", repo);
    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "frate-registry-gen")
        .send()?
        .error_for_status()?;

    let repo_info = response.json::<GitHubRepo>()?;
    Ok(repo_info)
}
