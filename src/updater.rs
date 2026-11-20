use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub html_url: String,
    #[serde(default)]
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseInfo {
    pub version: String,
    pub title: String,
    pub changelog: String,
    pub download_url: String,
    pub html_url: String,
    pub asset_size: u64,
}

pub fn clean_version_tag(tag: &str) -> &str {
    tag.trim().trim_start_matches(['v', 'V']).trim()
}

pub fn check_for_updates(repo: &str, current_version_str: &str) -> Result<Option<ReleaseInfo>, String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    
    let resp: GitHubRelease = ureq::get(&url)
        .set("User-Agent", &format!("Docmost-Desktop/{}", current_version_str))
        .set("Accept", "application/vnd.github.v3+json")
        .timeout(std::time::Duration::from_secs(10))
        .call()
        .map_err(|e| format!("Fehler beim Abrufen der GitHub Releases: {}", e))?
        .into_json()
        .map_err(|e| format!("Fehler beim Parsen der Release-Informationen: {}", e))?;

    let clean_tag = clean_version_tag(&resp.tag_name).to_string();
    let remote_ver = Version::parse(&clean_tag)
        .map_err(|e| format!("Ungültige Remote-Version '{}': {}", clean_tag, e))?;
    
    let current_ver = Version::parse(current_version_str)
        .map_err(|e| format!("Ungültige lokale Version '{}': {}", current_version_str, e))?;

    if remote_ver > current_ver {
        // Suche nach passender .exe Datei im Release (bevorzuge Setup/Installer)
        let exe_asset = resp.assets.iter().find(|a| {
            let lower = a.name.to_lowercase();
            lower.ends_with(".exe") && (lower.contains("setup") || lower.contains("installer"))
        }).or_else(|| {
            resp.assets.iter().find(|a| a.name.to_lowercase().ends_with(".exe"))
        });

        if let Some(asset) = exe_asset {
            Ok(Some(ReleaseInfo {
                version: resp.tag_name,
                title: resp.name.unwrap_or_else(|| format!("Version {}", clean_tag)),
                changelog: resp.body.unwrap_or_default(),
                download_url: asset.browser_download_url.clone(),
                html_url: resp.html_url,
                asset_size: asset.size,
            }))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub fn download_and_run_installer(download_url: &str) -> Result<(), String> {
    let temp_dir = std::env::temp_dir();
    let target_file = temp_dir.join("Docmost-Update-Setup.exe");

    let response = ureq::get(download_url)
        .set("User-Agent", "Docmost-Desktop-Updater")
        .timeout(std::time::Duration::from_secs(60))
        .call()
        .map_err(|e| format!("Download fehlgeschlagen: {}", e))?;

    let mut reader = response.into_reader();
    let mut file = File::create(&target_file)
        .map_err(|e| format!("Konnte Zieldatei {:?} nicht erstellen: {}", target_file, e))?;

    let mut buffer = [0u8; 8192];
    loop {
        let n = reader.read(&mut buffer).map_err(|e| format!("Download-Lesefehler: {}", e))?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n]).map_err(|e| format!("Schreibfehler: {}", e))?;
    }
    file.flush().map_err(|e| format!("Flush-Fehler: {}", e))?;
    drop(file);

    // Starte den heruntergeladenen Installer
    Command::new(&target_file)
        .spawn()
        .map_err(|e| format!("Konnte Installer {:?} nicht starten: {}", target_file, e))?;

    // Beende die aktuelle Anwendung, damit der Installer die Dateien überschreiben kann
    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_version_tag() {
        assert_eq!(clean_version_tag("v1.2.3"), "1.2.3");
        assert_eq!(clean_version_tag("V0.1.0"), "0.1.0");
        assert_eq!(clean_version_tag("  v2.0.0  "), "2.0.0");
        assert_eq!(clean_version_tag("1.0.0"), "1.0.0");
    }

    #[test]
    fn test_semver_comparison() {
        let v1 = Version::parse(clean_version_tag("v0.1.0")).unwrap();
        let v2 = Version::parse(clean_version_tag("v0.2.0")).unwrap();
        assert!(v2 > v1);
    }

    #[test]
    fn test_github_api_check() {
        let res = check_for_updates("LegoPuck-orginal/docmost-desktop", "0.1.0");
        println!("Result: {:?}", res);
        assert!(res.is_ok());
        let info = res.unwrap();
        println!("Release info: {:?}", info);
        assert!(info.is_some());
    }
}
