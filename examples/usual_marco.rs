//! Example for using Flopper

use base64::Engine;
// for dotenv macro
use dotenvy_macro::dotenv;
// main package
use flopper::flop;
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

    // Use flop macro to fetch images.
    info!("Fetch images using flop!");
    let images = flop!(
        key,
        secret,
        n = 1,                    // optional!
        w = 512,                  // optional but required
        h = 512,                  // optional but required
        q = "forest".to_string(), // optional but required
        m = 4                     // optional!
    );
    info!("convert from base64 to png");
    let image = base64::engine::general_purpose::STANDARD.decode(images[0].clone())?;
    info!("write to test_image.png");
    std::fs::write("test_image.png", image)?;
    info!("Done!");
    Ok(())
}
