#[macro_export]
/// Generate base64 images!
///
/// return: [Vec]<[String]>
///
/// args:
///     - key: auth key
///     - secret: auth secret
///     - gen_type: [GenType]
///     - n: number of images
///     - w: image width
///     - h: image height
///     - q: your query
///     - d: duration between requests for getting work status/image
///
/// How?
///     - 1. Create Flopper instance
///     - 2. Push query
///     - 3. Await generation
///     - 4. Get images!
macro_rules! flop {
    ($key:expr, $secret:expr $(,g = $gen_type:expr)? $(,n = $n:expr)? $(,w = $w:expr)? $(,h = $h:expr)? $(,q = $q:expr)? $(,d = $duration:expr)? $(,m = $model:expr)?) => {{
        use $crate::prelude::{Flopper, GenType, Params};

        let gen_type =  GenType::Generate;
        let n = 1;
        let w = 512;
        let h = 512;
        let q = "".to_string();
        let m: Option<usize> = None;
        let d: Option<usize> = None;

        $(let gen_type = $gen_type)?;
        $(
            if $n > 1 {
                panic!("**Note**: Now available only one image!");
            }
            let n = $n
        )?;

        $(let w = $w)?;
        $(let h = $h)?;
        $(let q = $q)?;
        $(let m = Some($model))?;
        $(let d = Some($duration))?;

        let flopper = Flopper::build(
            $key.to_string(),
            $secret.to_string(),
            m,
            d,
        ).await?;

        let params = Params::new(gen_type, n, w, h, q);

        let info = flopper.push(params).await?;

        let images = flopper.try_fetch(info).await?;
        images
    }};
}
