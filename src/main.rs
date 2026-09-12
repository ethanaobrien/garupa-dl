mod network;
mod game;
mod encryption;

use clap::{Parser, ValueEnum};

// This file is generated in build.rs from the proto files
#[allow(non_snake_case)]
pub mod Bandori {
    include!(concat!(env!("OUT_DIR"), "/bandori.rs"));
}

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum)]
pub enum Platform {
    Android,
    Ios,
}

// This allows us to cleanly print the platform enum to the console
impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Android => write!(f, "Android"),
            Platform::Ios     => write!(f, "iOS"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "https://api.garupa.jp/api")]
    api_url: String,

    #[arg(long, default_value = "https://content.garupa.jp/Release/")]
    cdn_url: String,

    #[arg(long)]
    version_hash: String,

    #[arg(long)]
    client_version: String,

    #[arg(short, long, default_value = "./assets/")]
    output: String,

    #[arg(long, value_enum, help = "Platform to download assets for", value_delimiter = ',', required = true)]
    platforms: Vec<Platform>,

    #[arg(long, default_value = "mikumikulukaluka")]
    aes_key: String,

    #[arg(long, default_value = "lukalukamikumiku")]
    aes_iv: String,
}

// Helper function to get the argv
fn get_args() -> Result<Args, ()> {
    let args = Args::parse();
    if args.version_hash.is_empty() {
        println!("version_hash is required!");
        return Err(());
    }
    if args.client_version.is_empty() {
        println!("client_version is required!");
        return Err(());
    }
    Ok(args)
}

// Application entry point, starts here. Parses arguments, checks for an update, then downloads everything
#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = get_args()?;

     // Check for an update before doing anything else.
    game::check_for_update(&args.api_url, &args.client_version, &args.aes_key, &args.aes_iv).await?;

    let (data_version, full_version_hash) = game::get_latest_version_info(
        &args.api_url,
        &args.aes_key,
        &args.aes_iv,
        &args.version_hash,
      &args.client_version,
    ).await?;

    println!("data_version: {data_version}");
    println!("full_version_hash: {full_version_hash}");

    game::download_assets(&args.cdn_url, &full_version_hash, &args.output, &args.platforms, &args.client_version).await?;

    Ok(())
}
