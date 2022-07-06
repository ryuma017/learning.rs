use reqwest::Url;
use scraper::{Html, Selector};

const GITHUB_URL: &str = "https://github.com";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_url = Url::parse_with_params(
        Url::parse(GITHUB_URL)?.join("trending/rust")?.as_str(),
        &[("since", "daily")],
    )?;
    let client = reqwest::Client::new();
    let html = client.get(target_url).send().await?.text().await?;
    println!("{}", html);
    let document = Html::parse_document(&html);
    let article_selector = Selector::parse(".Box > article.Box-row").unwrap();
    let result = document
        .select(&article_selector)
        .filter_map(|article| {
            let title_selector = Selector::parse("h3").unwrap();
            let title = article.select(&title_selector).next()?.inner_html();
            Some(title)
        })
        .collect::<Vec<String>>();
    println!("{:?}", result);
    Ok(())
}
