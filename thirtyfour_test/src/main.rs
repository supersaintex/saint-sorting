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
    signin_email.send_keys("seiieshima831@gmail.com").await?;

    let signin_password = signin_form.find(By::Id("password-2")).await?;
    signin_password.send_keys("123456").await?;

    let signin_buttom = driver.find(By::Id("button-2")).await?;
    signin_buttom.click().await?;

    // go to clothing page
    driver.goto("/app/clothing").await?;
   
    //write test
    let pop_up_label = driver.find(By::Id("pop-up-label")).await?;
    pop_up_label.click().await?;
    
    let w_document_id = driver.find(By::Id("w_document_id")).await?;
    w_document_id.send_keys("Test-shirts").await?;
    
    let brand = driver.find(By::Id("brand")).await?;
    brand.send_keys("American Beauty").await?;
    
    let year = driver.find(By::Id("year")).await?;
    year.send_keys("2023").await?;
    
    let month = driver.find(By::Id("month")).await?;
    month.send_keys("4").await?;
    
    let season = driver.find(By::Id("season")).await?;
    season.send_keys("Spring").await?;

    let shop = driver.find(By::Id("shop")).await?;
    shop.send_keys("koenji").await?;
    
    let category = driver.find(By::Id("category")).await?;
    category.send_keys("tops").await?;

    let write_buttom = driver.find(By::Id("write_button")).await?;
    write_buttom.click().await?;

    Ok(())
}
