use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

mod structs;

use reqwest::header::CONTENT_LENGTH;
use reqwest::Client;
use structs::{BasicResponse, ResourceResponse};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The URL of the resources
const DOWNLOAD_URL: &str = "https://prod-cn-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G152/10008_Pa0Q0EMFxukjEqX33pF9Uyvdc8MaGPSz/index.json";

/// Download the resources
async fn download_resources(url: &str, output_path: &str, client: &Client) -> AppResult<()> {
    println!("Fetching: {}", output_path);

    let response = client.get(url).send().await?;
    let total_length = response
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|len| len.to_str().ok())
        .and_then(|len| len.parse().ok())
        .unwrap_or(0);

    println!("Starting download");

    let mut progress = 0;
    let progress_bar_width = 40;

    let mut file = File::create(output_path).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        progress += chunk.len();

        // Simple progress bar
        let progress_percentage = (progress as f64 / total_length as f64) * 100.0;
        let progress_bar_complete =
            (progress_percentage / 100.0 * progress_bar_width as f64) as usize;
        let progress_bar_incomplete = progress_bar_width - progress_bar_complete;

        print!(
            "\r-> downloading [{}{}] {:.2}% complete",
            "=".repeat(progress_bar_complete),
            " ".repeat(progress_bar_incomplete),
            progress_percentage
        );
        io::stdout().flush().unwrap();
    }

    println!("\nDownload completed for: {}", output_path);

    Ok(())
}

/// Get the resources json
async fn get_resources_json(data: &BasicResponse, client: &Client) -> AppResult<ResourceResponse> {
    let url = format!("{}{}", data.default.cdn_list[0].url, data.default.resources);
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch resources JSON: HTTP {}", response.status()).into());
    }

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok());

    if content_type != Some("application/json") {
        return Err(format!(
            "Unexpected content type: {:?} when fetching resources JSON",
            content_type
        )
        .into());
    }

    let resource_response: ResourceResponse = response.json().await?;
    Ok(resource_response)
}

/// Get the index json
async fn get_index_json(url: &str, client: &Client) -> AppResult<BasicResponse> {
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch index JSON: HTTP {}", response.status()).into());
    }

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok());

    println!("{:}", content_type.unwrap());

    if content_type != Some("application/json") {
        return Err(format!(
            "Unexpected content type: {:?} when fetching index JSON",
            content_type
        )
        .into());
    }
    let index_response: BasicResponse = response.json().await?;
    Ok(index_response)
}

/// Get the link of the resource
fn get_link(
    basic: &BasicResponse,
    resource: &ResourceResponse,
    index: usize,
) -> HashMap<String, String> {
    let mut result = HashMap::new();
    result.insert(
        "url".to_string(),
        format!(
            "{}{}{}",
            basic.default.cdn_list[0].url,
            basic.default.resources_base_path,
            resource.resource[index].dest
        ),
    );
    result.insert("md5".to_string(), resource.resource[index].md5.clone());
    result.insert("path".to_string(), resource.resource[index].dest.clone());
    result
}

/// Fetch the data from the URL
async fn fetch_data(url: &str, client: &Client) -> AppResult<()> {
    let index = get_index_json(url, client).await?;
    let resource = get_resources_json(&index, client).await?;
    for (i, _) in resource.resource.iter().enumerate() {
        let res = get_link(&index, &resource, i);
        let output_path = Path::new(&res["path"]);
        if let Some(parent) = output_path.parent() {
            if !parent.exists() {
                println!("Creating directory: {:?}", parent);
                // fs::create_dir_all(parent)?;
            }
        }
        println!("Downloading: {}", res["path"]);
        // download_resources(&res["url"], &res["path"]).await?;
    }
    Ok(())
}

/// main function
/// here is where the magic happens
#[tokio::main]
async fn main() -> AppResult<()> {
    let client = Client::new();
    fetch_data(DOWNLOAD_URL, &client).await?;
    Ok(())
}
