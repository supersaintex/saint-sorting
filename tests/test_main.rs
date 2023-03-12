use thirtyfour::prelude::*;

#[actix_rt::test]
async fn test_main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    //caps.add_chrome_arg("--enable-automation")?;

    println!("a");
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    println!("b");
    //driver.get("https://localhost:8080/app/top").await?;
    driver.goto("https://wikipedia.org").await?;

    Ok(())
}
