use std::collections::HashMap;
#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
    // let resp = reqwest::get("https://www.baidu.com/").await?.json::<HashMap<String,String>>().await?;
    let resp = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    println!("{:#?}",resp);
    Ok(())
}
