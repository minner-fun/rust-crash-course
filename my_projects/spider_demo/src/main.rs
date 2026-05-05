use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use scraper::{Html, Selector};
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    // 你要爬的目标（建议先用自己能访问的站点）
    let target = "https://rust-lang.org/";
    let base = Url::parse(target)?;

    // 构建 client + UA
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("spider_demo/0.1 (learning rust)"),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;

    // 下载 HTML
    let html = client.get(target).send().await?.error_for_status()?.text().await?;

    // 解析 HTML
    let doc = Html::parse_document(&html);
    let sel = Selector::parse("a[href]").unwrap();

    println!("Links found on {target}:");

    for a in doc.select(&sel) {
        if let Some(href) = a.value().attr("href") {
            // 把相对链接转换成绝对链接（base.join）
            if let Ok(abs) = base.join(href) {
                println!(" - {abs}");
            } else {
                // join 失败就原样输出
                println!(" - {href}");
            }
        }
    }

    Ok(())
}