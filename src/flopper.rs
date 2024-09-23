mod data_set;
pub mod gen_type;
mod info;
mod model;
pub mod params;
mod query;
mod status;
mod work;

use std::sync::Arc;

use crate::{error, result::Result};
use info::Info;
use miniserde::json;
use model::Model;
use params::Params;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    multipart::{self, Form},
    Client,
};
use status::Status;
use tracing::{debug, info, instrument, trace};
use work::Work;

pub struct Flopper {
    client: Arc<Client>,
    model: usize,
}

impl Flopper {
    /// Standard model by default
    const DEFAULT_MODEL: usize = 4;
    /// Url for getting available models
    const URL_MODEL: &'static str = "https://api-key.fusionbrain.ai/key/api/v1/models";
    /// Url for generating images
    const URL_RUN: &'static str = "https://api-key.fusionbrain.ai/key/api/v1/text2image/run";
    /// Url for getting work status
    const URL_WORK_STATUS: &'static str =
        "https://api-key.fusionbrain.ai/key/api/v1/text2image/status";
    /// Duration between requests for getting work status
    const DURATION: usize = 60;

    #[instrument(skip(key, secret))]
    /// Create static client for requests
    pub async fn build(key: String, secret: String, model: Option<usize>) -> Result<Self> {
        let key = format!("Key {}", key);
        let secret = format!("Secret {}", secret);
        let mut headers = HeaderMap::new();
        headers.insert("X-Key", HeaderValue::from_str(&key)?);
        headers.insert("X-Secret", HeaderValue::from_str(&secret)?);
        trace!("Create client with headers: {:#?}", &headers);
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;
        Ok(Self {
            client: Arc::new(client),
            model: model.unwrap_or(Self::DEFAULT_MODEL),
        })
    }

    #[instrument(skip(self))]
    /// Return all available models
    pub async fn get_all_model(&self) -> Result<Vec<Model>> {
        debug!("Send request");
        let r = self.client.get(Self::URL_MODEL).send().await?;
        debug!("Fetch text");
        let text = r.text().await?;
        trace!("Parse text to Vec<Model>: {:#?}", text);
        let models: Vec<Model> = miniserde::json::from_str(&text)?;
        Ok(models)
    }

    #[instrument(skip(self))]
    /// Make request to api for generate image
    /// Return: uuid of image request
    pub async fn push(&self, params: Params) -> Result<Info> {
        debug!("Crate form");
        let form = Form::new()
            .part("model_id", multipart::Part::text(self.model.to_string()))
            .part(
                "params",
                multipart::Part::text(json::to_string(&params)).mime_str("application/json")?,
            );
        debug!("Create and Send Response");
        let r = self
            .client
            .post(Self::URL_RUN)
            .multipart(form)
            .send()
            .await?;
        debug!("Fetch text");
        let text = r.text().await?;
        trace!("Parse text to Info\n{:#?}", text);
        let pull_out: Info = miniserde::json::from_str(&text)?;
        Ok(pull_out)
    }

    #[instrument(skip(self, pull))]
    /// Create loop for fetching work status and wait for [DURATION] seconds
    pub async fn try_fetch(&self, pull: Info) -> Result<Vec<String>> {
        info!("Generate request");
        let r = self
            .client
            .get(&format!("{}/{}", Self::URL_WORK_STATUS, pull.uuid));

        loop {
            info!("Wait for {} seconds", Self::DURATION);
            tokio::time::sleep(std::time::Duration::from_secs(Self::DURATION as u64)).await;
            debug!("Clone and Send request");
            let r = r
                .try_clone()
                .ok_or(error::Errors::CloneError)?
                .send()
                .await?;

            debug!("Fetch text");
            let text = r.text().await?;

            trace!("Parse text to Work {:#?}", text);
            let work: Work = miniserde::json::from_str(&text)?;

            debug!("Check status");
            match work.status {
                Status::Initial => {
                    info!("Initial status: {}", work.uuid);
                    continue;
                }
                Status::Processing => {
                    info!("Processing status: {}", work.uuid);
                    continue;
                }
                Status::Fail => {
                    return Err(error::Errors::WorkFail(format!(
                        "error: {:#?}, censored: {}, uuid: {}",
                        work.error, work.censored, work.uuid
                    )))
                }
                Status::Done => {
                    info!("Done status: {}", work.uuid);
                    return Ok(if let Some(images) = work.images {
                        images
                    } else {
                        return Err(error::Errors::WorkFailNoImage(format!(
                            "uuid: {}",
                            work.uuid
                        )));
                    });
                }
            }
        }
    }
}
