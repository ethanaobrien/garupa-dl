use http::HeaderMap;

// The game server expects these headers. They don't need to be valid, just present.
pub fn garupa_headers(platform: &str, client_version: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("x-unity-version", "2022.3.62f1".parse().unwrap());
    headers.insert("x-clientplatform", platform.parse().unwrap());
    headers.insert("x-clientversion", client_version.parse().unwrap());
    headers.insert("x-signature", "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa".parse().unwrap());
    headers.insert("x-token", "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa".parse().unwrap());
    headers.insert("accept", "application/octet-stream".parse().unwrap());
    headers.insert("content-type", "application/octet-stream".parse().unwrap());
    headers.insert("user-agent", "UnityPlayer/2022.3.62f1 (UnityWebRequest/1.0, libcurl/8.10.1-DEV)".parse().unwrap());
    headers
}


// Downloads a url with the supplied headers and returns the body, or the error if something is wrong
pub async fn download_bytes(headers: &HeaderMap, url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let resp = reqwest::Client::new().get(url).headers(headers.clone()).send().await?;
    Ok(resp.bytes().await?.to_vec())
}

// Like download_bytes but issues a PUT with a request body
pub async fn put_bytes(headers: &HeaderMap, url: &str, body: &[u8]) -> Result<Vec<u8>, reqwest::Error> {
    let resp = reqwest::Client::new().put(url).body(body.to_vec()).headers(headers.clone()).send().await?;
    Ok(resp.bytes().await?.to_vec())
}
