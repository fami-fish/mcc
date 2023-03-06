use poise::{serenity_prelude::GatewayIntents, FrameworkBuilder};
use std::fmt;
mod commands;
use commands::{api, docs};
use serde::Deserialize;
use shuttle_secrets::SecretStore;

pub struct Data {}
pub struct PoiseService {
    discord_bot:
        FrameworkBuilder<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>,
}

#[shuttle_service::async_trait]
impl shuttle_service::Service for PoiseService {
    async fn bind(
        mut self: Box<Self>,
        _addr: std::net::SocketAddr,
    ) -> Result<(), shuttle_service::error::Error> {
        tokio::select!(
            _ = self.discord_bot.run() => {}
        );
        Ok(())
    }
}

#[shuttle_service::main]
async fn init(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> Result<PoiseService, shuttle_service::Error> {
    let discord_api_key = secrets.get("TOKEN").unwrap();

    let discord_bot = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![docs(), api()],
            ..Default::default()
        })
        .token(discord_api_key)
        .intents(GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    Ok(PoiseService { discord_bot })
}

#[derive(Debug, Deserialize)]
struct UrlSet {
    url: Vec<UrlEntry>,
}

#[derive(Debug, Deserialize)]
struct UrlEntry {
    loc: String,
}

#[derive(Debug, Clone)]
pub struct XmlFetchError(String);

impl fmt::Display for XmlFetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing sitemap: {}", self.0)
    }
}

impl From<reqwest::Error> for XmlFetchError {
    fn from(value: reqwest::Error) -> Self {
        XmlFetchError(value.to_string())
    }
}

impl From<serde_xml_rs::Error> for XmlFetchError {
    fn from(value: serde_xml_rs::Error) -> Self {
        XmlFetchError(value.to_string())
    }
}

impl From<&str> for XmlFetchError {
    fn from(value: &str) -> Self {
        XmlFetchError(value.to_owned())
    }
}

pub async fn get_sitemap() -> Result<Vec<String>, XmlFetchError> {
    let res = reqwest::Client::new()
        .get("https://motioncanvas.io/sitemap.xml")
        .header("Accept", "application/xml")
        .send()
        .await?
        .text()
        .await?;

    let set: UrlSet = serde_xml_rs::from_str(&res).map_err(XmlFetchError::from)?;

    let entries: Vec<UrlEntry> = set.url;

    let endpoints: Vec<String> = entries.iter().map(|x| x.loc.to_string()).collect();

    Ok(endpoints)
}
