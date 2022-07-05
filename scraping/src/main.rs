use reqwest::Url;

const QIITA_URL: &str = "https://qiita.com";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_url = Url::parse(QIITA_URL)?.join("tags/rust")?;
    let client = reqwest::Client::new();
    let html = client.get(target_url).send().await?.text().await?;

    println!("{}", html);
    Ok(())
}
