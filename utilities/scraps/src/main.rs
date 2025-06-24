use scraps::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logline::init_with_level(INFO);

    let target_url = "https://scrapeme.live/shop/";
    let content = get::html_content(target_url).await?;
    let document = get::html_document(content)?;

    // let products = data::products(document);
    // scrape::products(document);
    // warn!("{:#?}", &document);
    warn!("{:#?}", "pop");
    info!("{:#?}", "pop");

    Ok(())
}
