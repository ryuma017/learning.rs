use anyhow::Context as _;
use futures::future::try_join_all;
use reqwest::header;
use serde_json::Value;

use std::iter::zip;
use std::sync::Arc;

// 調査対象のリポジトリ
const REPOSITORIES: [&str; 3] = ["yewstack/yew", "rui314/mold", "actix/actix-web"];

async fn get_stargazers_count(client: &reqwest::Client, repository: String) -> anyhow::Result<u64> {
    let response: Value = client
        .get(format!("https://api.github.com/repos/{}", repository))
        .send()
        .await?
        .json()
        .await?;

    let count = response
        .get("stargazers_count")
        .context("Not found")?
        .as_u64()
        .context("Value error")?;

    Ok(count)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    let client = reqwest::Client::builder()
        .user_agent("rust requwst")
        .default_headers(headers)
        .build()?;

    let client = Arc::new(client);

    let handlers = REPOSITORIES
        .into_iter()
        .map(|repository| {
            let client = client.clone();

            tokio::spawn(async move { get_stargazers_count(&client, repository.to_string()).await })
        })
        .collect::<Vec<_>>();

    let stargazers: Vec<u64> = try_join_all(handlers)
        .await?
        .into_iter()
        .collect::<anyhow::Result<Vec<u64>>>()?;

    zip(REPOSITORIES.into_iter(), stargazers.iter())
        .for_each(|(repo, star_count)| println!("repo: {}, star_count: {}", repo, star_count));

    Ok(())
}
