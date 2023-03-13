use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444/", caps).await?;
    driver.goto("https:/127.0.0.1:8080/app/top").await?;

    Ok(())
}
