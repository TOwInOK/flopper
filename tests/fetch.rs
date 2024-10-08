#[cfg(test)]
mod flopper_test {
    use std::sync::LazyLock;

    use anyhow::Ok;
    use base64::Engine;
    use dotenvy_macro::dotenv;
    use flopper::prelude::*;

    const KEY: &str = dotenv!("KEY");
    const SECRET: &str = dotenv!("SECRET");
    static TS: LazyLock<()> = LazyLock::new(|| {
        use tracing::Level;
        use tracing_subscriber::FmtSubscriber;

        tracing::subscriber::set_global_default(
            FmtSubscriber::builder()
                .with_max_level(Level::INFO)
                .pretty()
                .without_time()
                .finish(),
        )
        .expect("Fail to set global default subscriber");
    });

    #[tokio::test]
    async fn fetch() -> anyhow::Result<()> {
        LazyLock::force(&TS);
        let flopper = Flopper::build(KEY.to_string(), SECRET.to_string(), None, None).await?;
        let mut params = Params::default();
        params.q("forest".to_string());
        let info = flopper.push(params).await?;
        let result = flopper.try_fetch(info).await?;
        let image = base64::engine::general_purpose::STANDARD.decode(result[0].clone())?;
        std::fs::write("test_image.png", image)?;
        Ok(())
    }

    #[tokio::test]
    async fn flop_fetch() -> anyhow::Result<()> {
        LazyLock::force(&TS);
        let images = flop!(
            KEY,
            SECRET,
            n = 1,
            w = 512,
            h = 512,
            q = "forest".to_string(),
            m = 4
        );
        let image = base64::engine::general_purpose::STANDARD.decode(images[0].clone())?;
        std::fs::write("test_image.png", image)?;
        Ok(())
    }
}
