//! Example for using Flopper

use base64::Engine;
// for dotenv macro
use dotenvy_macro::dotenv;
// main package
use flopper::prelude::*;
// tracing
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // parse .env
    let key: &str = dotenv!("KEY");
    let secret: &str = dotenv!("SECRET");

    // Initialize tracing
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .pretty()
            .without_time()
            .finish(),
    )
    .expect("Fail to set global default subscriber");

    // Init Flopper instance
    let flopper = Flopper::build(key.to_string(), secret.to_string(), None, None).await?;

    // Make params by default and add your params
    let mut params = Params::default();
    params.q("cat".to_string());
    // or create params with new()
    let params = Params::new(GenType::Generate, 1, 512, 512, "cat".to_string());

    // Push request to generate image
    let info = flopper.push(params).await?;
    // Start fetching
    let images = flopper.try_fetch(info).await?;
    let image = images[0].clone(); // we need to clone

    // Convert image from base64 to image
    let image = base64::engine::general_purpose::STANDARD.decode(image)?;
    // Save image to file
    std::fs::write("examples/usual/test_image.png", image)?;

    info!("Done!");
    Ok(())
}
