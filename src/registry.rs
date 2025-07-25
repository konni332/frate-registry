use std::collections::{BTreeMap};
use std::io::Read;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use crate::fetch::GitHubRelease;
use crate::util::extract_target_triple;

#[derive(Debug, Deserialize, Serialize)]
pub struct Registry {
    pub registered: Vec<ToolInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ToolInfo {
    pub name: String,
    pub repo: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegistryTool {
    pub name: String,
    pub repo: String,
    pub releases: BTreeMap<String, ReleaseInfo>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReleaseInfo {
    pub url: String,
    pub hash: String,
}

pub fn generate_registry(
    releases: Vec<GitHubRelease>,
    full_name: &str,
    max_releases: Option<usize>
) -> Result<RegistryTool, Box<dyn std::error::Error>> {
    let mut result = BTreeMap::new();

    let iter = match max_releases {
        Some(max) => releases.into_iter().take(max).collect::<Vec<_>>(),
        None => releases,
    };

    for release in iter {
        println!("{}:", &release.tag_name);
        let version = release.tag_name.trim_start_matches("v");

        for asset in release.assets {
            let name = &asset.name;
            if !(name.ends_with(".tar.gz") || name.ends_with(".zip")) {
                continue;
            }

            if let Some(triple) = extract_target_triple(name){
                let full_version = format!("{}-{}", version, triple);

                println!("  Downloading {}", asset.browser_download_url);
                let client = Client::new();
                let mut resp = client
                    .get(&asset.browser_download_url)
                    .send()?
                    .error_for_status()?;
                let mut buf = Vec::new();
                resp.read_to_end(&mut buf)?;

                let mut hasher = sha2::Sha256::new();
                hasher.update(&buf);
                let hash = format!("sha256:{}", hex::encode(hasher.finalize()));

                result.insert(full_version, ReleaseInfo {
                    url: asset.browser_download_url,
                    hash,
                });
            }
        }
    }
    Ok(RegistryTool {
        name: full_name.split("/").last().unwrap_or("unknown").to_string(),
        repo: format!("https://github.com/{}", full_name),
        releases: result,
    })
}

pub fn save_registry(tool: &RegistryTool, filename: &str) -> std::io::Result<()> {
    let path = std::env::current_dir()?.join("tools").join(filename);
    let tool_str = serde_json::to_string_pretty(&tool)?;
    std::fs::write(path, tool_str)?;
    Ok(())
}
