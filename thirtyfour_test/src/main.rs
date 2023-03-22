use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444/", caps).await?;

    // Sign in
    driver.goto("https:/127.0.0.1:8080/app/top").await?;

    let signin_form = driver.find(By::Id("signInForm")).await?;
    let signin_email = signin_form.find(By::Id("email-2")).await?;
    //input each mail address
    signin_email.send_keys("seiieshima831@gmail.com").await?;

    let signin_password = signin_form.find(By::Id("password-2")).await?;
    //input each password
    signin_password.send_keys("123456").await?;

    let signin_buttom = driver.find(By::Id("button-2")).await?;
    signin_buttom.click().await?;

    Ok(())
}
