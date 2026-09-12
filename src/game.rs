use std::fs;
use std::path::Path;

use crate::Bandori::{AppGetResponse, AssetBundleInfo, AppUpdateCheck};
use crate::encryption::decrypt;
use crate::network::{download_bytes, garupa_headers, put_bytes};
use prost::Message;

// Max attempts for failed/missing api errors
const MAX_ATTEMPTS: usize = 3;

// Gets the current asset version from the game server with the game version supplied
pub async fn get_latest_version_info(
    api_url: &str,
    aes_key: &str,
    aes_iv: &str,
    version_hash: &str,
    client_version: &str,
) -> Result<(String, String), ()> {
    let url = format!("{api_url}/application");
    println!("Querying API: {url}");

    let headers = garupa_headers("Android", client_version);
    let ciphertext = download_bytes(&headers, &url).await.map_err(|e| {
        println!("API error: {e}");
        ()
    })?;

    let decrypted = decrypt(aes_key.as_bytes(), aes_iv.as_bytes(), &ciphertext).map_err(|e| {
        println!("Decrypt error: {e}");
        ()
    })?;

    let info = AppGetResponse::decode(decrypted.as_slice()).map_err(|e| {
        println!("Could not parse AppGetResponse. Your supplied version / hash is invalid: {e}");
        ()
    })?;

    let data_version = info.data_version;
    let full_version_hash = format!("{data_version}_{version_hash}");

    Ok((data_version, full_version_hash))
}

// Checks and notifies the user if the application version being downloaded is not the latest
pub async fn check_for_update(
    api_url: &str,
    client_version: &str,
    aes_key: &str,
    aes_iv: &str,
) -> Result<bool, ()> {
    let url = format!("{api_url}/user/123/auth");
    println!("Checking for application updates: {url}");

    // The request body is a raw, unencrypted "15" — only the response is encrypted
    let headers = garupa_headers("Android", client_version);
    let ciphertext = put_bytes(&headers, &url, b"15").await.map_err(|e| {
        println!("Update check error: {e}");
        ()
    })?;

    if ciphertext.is_empty() {
        println!("Update check response empty.");
        return Ok(false);
      }

    let decrypted = decrypt(aes_key.as_bytes(), aes_iv.as_bytes(), &ciphertext).map_err(|e| {
        println!("Decrypt error: {e}");
        ()
    })?;

    let info = AppUpdateCheck::decode(decrypted.as_slice()).map_err(|e| {
        println!("Could not parse update check response: {e}");
        ()
    })?;

    let update_required = info.status == "application_update_required";
    if update_required {
        println!("NOTICE: Application update detected!");
    }
    Ok(update_required)
}

// Downloads assets for a single platform.
// Fetches the AssetBundleInfo file so we know where everything is
// and then iterates through it and downloads everything.
async fn download_platform(base_url: &str, output_dir: &std::path::PathBuf, version: &str, platform: &crate::Platform) -> Result<(), ()> {

    // Fetch, save, and parse the manifest — it's part of the CDN, so keep it on disk.
    println!("Fetching {platform} AssetBundleInfo...");
    let headers = garupa_headers(&platform.to_string(), version);
    let manifest = download_bytes(&headers, &format!("{base_url}AssetBundleInfo")).await.map_err(|e| {
        println!("{platform} manifest error: {e}");
        ()
    })?;

    fs::create_dir_all(&output_dir).map_err(|e| {
        println!("{platform}: could not create output dir: {e}");
        ()
    })?;
    fs::write(output_dir.join("AssetBundleInfo"), &manifest).map_err(|e| {
        println!("{platform}: could not save manifest: {e}");
        ()
    })?;

    let info = AssetBundleInfo::decode(manifest.as_slice()).map_err(|e| {
        println!("{platform}: could not parse manifest: {e}");
        ()
    })?;

    // Download each bundle, skipping ones already present at the expected size.
    // A bundle that keeps failing is retried a few times, then logged and skipped.
    let mut downloaded: usize = 0;
    let mut skipped: usize = 0;
    let total: usize = info.bundles.len();
    let mut failed: Vec<(String, String)> = Vec::new();

    for (name, bundle) in info.bundles {
        let dest = output_dir.join(&name);
        let current = downloaded + skipped + 1;

        if let Ok(meta) = fs::metadata(&dest) {
            if meta.len() == bundle.file_size as u64 {
                println!("[{current}/{total}] {platform}/{name} already present, skipping");
                skipped += 1;
                continue;
            }
        }

        let mut ok = false;
        let mut last_error = String::new();

        for attempt in 1..=MAX_ATTEMPTS {
            let bytes = match download_bytes(&headers, &format!("{base_url}{name}")).await {
                Ok(bytes) => bytes,
                Err(e) => {
                    last_error = e.to_string();
                    println!("{platform}/{name}: attempt {attempt}/{MAX_ATTEMPTS} failed: {e}");
                    continue;
                }
            };

            if bytes.len() as u64 != bundle.file_size as u64 {
                last_error = format!("size {} != {}", bytes.len(), bundle.file_size);
                println!("{platform}/{name}: attempt {attempt}/{MAX_ATTEMPTS} {last_error}");
                continue;
            }

            if let Some(parent) = dest.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    last_error = format!("could not create dir: {e}");
                    println!("{platform}/{name}: attempt {attempt}/{MAX_ATTEMPTS} {last_error}");
                    continue;
                }
            }
            if let Err(e) = fs::write(&dest, &bytes) {
                last_error = format!("write failed: {e}");
                println!("{platform}/{name}: attempt {attempt}/{MAX_ATTEMPTS} {last_error}");
                continue;
            }

            println!("[{current}/{total}] {platform}/{name}");
            downloaded += 1;
            ok = true;
            break;
        }

        if !ok {
            failed.push((name, last_error));
        }
    }

    println!(
        "{platform} complete. {downloaded} downloaded, {skipped} skipped, {} failed.",
        failed.len()
    );
    Ok(())
}

// Downloads all assets for all platforms
pub async fn download_assets(
    cdn_url: &str,
    full_version_hash: &str,
    output: &str,
    platforms: &Vec<crate::Platform>,
    client_version: &str,
) -> Result<(), ()> {
    for platform in platforms {
        let base_url = format!("{cdn_url}{full_version_hash}/{platform}/");
        let output_dir = Path::new(output).join(platform.to_string());
        download_platform(&base_url, &output_dir, client_version, platform).await?
    }

    Ok(())
}
