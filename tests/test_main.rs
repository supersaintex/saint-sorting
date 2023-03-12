use thirtyfour::prelude::*;

#[actix_rt::test]
async fn test_main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    //caps.add_chrome_arg("--enable-automation")?;

    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("https://localhost:8080/app/top").await?;
    //driver.goto("https://wikipedia.org").await?;

    Ok(())
}
