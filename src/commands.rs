use crate::get_sitemap;
use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn autocomplete_docs_slug<'a>(ctx: Context<'a>, partial: &'a str) -> Vec<String> {
    let sitemap = &ctx.data().sitemap.read().await;

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

async fn autocomplete_api_slug<'a>(ctx: Context<'a>, partial: &'a str) -> Vec<String> {
    let sitemap = &ctx.data().sitemap.read().await;

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

/// Get a link to the Motion Canvas documentation
#[poise::command(slash_command)]
pub async fn docs(
    ctx: Context<'_>,
    #[description = "The slug of the url"]
    #[autocomplete = "autocomplete_docs_slug"]
    mut slug: String,
) -> Result<(), Error> {
    if !slug.starts_with("/") {
        slug = "/".to_owned() + &slug;
    }

    if !ctx.data().sitemap.read().await.iter().any(|s| {
        s.starts_with(
            format!(
                "https://motioncanvas.io/docs{}",
                &slug[0..slug.find("#").unwrap_or(slug.len())]
            )
            .as_str(),
        )
    }) {
        ctx.send(|m| {
            m.ephemeral(true)
                .content("Invalid slug. That doesn't look like a page on the current version of the docs. Try using one of the provided options.")
        })
        .await?;

        return Ok(());
    }
    ctx.say(format!("https://motioncanvas.io/docs{}", slug))
        .await?;

    Ok(())
}

/// Get a link to the Motion Canvas API
#[poise::command(slash_command)]
pub async fn api(
    ctx: Context<'_>,
    #[description = "The slug of the api"]
    #[autocomplete = "autocomplete_api_slug"]
    mut slug: String,
) -> Result<(), Error> {
    if !slug.starts_with("/") {
        slug = "/".to_owned() + &slug;
    }

    if !ctx.data().sitemap.read().await.iter().any(|s| {
        s.starts_with(
            format!(
                "https://motioncanvas.io/api{}",
                &slug[0..slug.find("#").unwrap_or(slug.len())]
            )
            .as_str(),
        )
    }) {
        ctx.send(|m| {
            m.ephemeral(true)
                .content("Invalid slug. That doesn't look like a page on the current version of the docs. Try using one of the provided options.")
        })
        .await?;

        return Ok(());
    }
    ctx.say(format!("https://motioncanvas.io/api{}", slug))
        .await?;

    Ok(())
}

/// Regenerates the sitemap
#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
pub async fn generate_sitemap(ctx: Context<'_>) -> Result<(), Error> {
    let mut lock = ctx.data().sitemap.write().await;
    *lock = get_sitemap().await?;

    drop(lock);

    ctx.send(|m| m.ephemeral(true).content("Sucessfully updates sitemap"))
        .await?;

    Ok(())
}
