use crate::{get_sitemap, Data};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn autocomplete_docs_slug<'a>(_ctx: Context<'a>, partial: &'a str) -> Vec<String> {
    let sitemap = get_sitemap().await.unwrap();

    sitemap
        .iter()
        .filter_map(|endpoint| {
            if endpoint.contains(partial)
                && endpoint.starts_with("https://motioncanvas.io/docs")
                && !endpoint.ends_with("docs/")
            {
                return Some(
                    endpoint
                        .strip_prefix("https://motioncanvas.io/docs")
                        .unwrap()
                        .to_owned(),
                );
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}

async fn autocomplete_api_slug<'a>(_ctx: Context<'a>, partial: &'a str) -> Vec<String> {
    let sitemap = get_sitemap().await.unwrap();

    sitemap
        .iter()
        .filter_map(|endpoint| {
            if endpoint.contains(partial)
                && endpoint.starts_with("https://motioncanvas.io/api")
                && !endpoint.ends_with("api/")
            {
                return Some(
                    endpoint
                        .strip_prefix("https://motioncanvas.io/api")
                        .unwrap()
                        .to_owned(),
                );
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}

/// Get a url to the motion canvas documentation
#[poise::command(slash_command)]
pub async fn docs(
    ctx: Context<'_>,
    #[description = "The slug of the url"]
    #[autocomplete = "autocomplete_docs_slug"]
    slug: String,
) -> Result<(), Error> {
    ctx.say(format!("https://motioncanvas.io/docs{}", slug))
        .await?;

    Ok(())
}

/// Get a url to the motion canvas api
#[poise::command(slash_command)]
pub async fn api(
    ctx: Context<'_>,
    #[description = "The slug of the api"]
    #[autocomplete = "autocomplete_api_slug"]
    slug: String,
) -> Result<(), Error> {
    ctx.say(format!("https://motioncanvas.io/api{}", slug))
        .await?;

    Ok(())
}
