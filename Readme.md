# Flopper
[![Crates.io](https://img.shields.io/crates/v/flopper.svg)](https://crates.io/crates/flopper)
[![Documentation]( https://img.shields.io/badge/docs-doc.rs-red)](https://docs.rs/flopper)
[![License](https://img.shields.io/crates/l/flopper.svg)](LICENSE)

Flopper is an async Rust library for generating images using [fusionbrain api](https://fusionbrain.ai).

**Note**: still in development. Please, use it with caution.

If you have any questions or suggestions, open an issue.

## Examples

Best folder ever => [examples](examples)

**Note**: create [`.env`](example.env) 

### Usual example
```rust
use flopper::prelude::*; 
use dotenvy_macro::dotenv;

#[tokio::main]
async main () -> anyhow::Result<()>{
    // parse .env vars
    let key: &str = dotenv!("KEY");
    let secret: &str = dotenv!("SECRET");
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
    std::fs::write("test_image.png", image)?;

    Ok(())
}
```

### Macro example
```rust
use flopper::prelude::*; 
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // parse .env
    let key: &str = dotenv!("KEY");
    let secret: &str = dotenv!("SECRET");

    // Use flop macro to fetch images.
    let images = flop!(
        key,
        secret,
        n = 1,                    // optional!
        w = 512,                  // optional but required
        h = 512,                  // optional but required
        q = "forest".to_string(), // optional but required
        m = 4                     // optional!
    );
    // convert from base64 to png
    let image = base64::engine::general_purpose::STANDARD.decode(images[0].clone())?;
    // write to test_image.png
    std::fs::write("test_image.png", image)?;

    Ok(())
}
```