//se std::collections::HashMap;
// #[tokio::main]
// async fn main()->Result<(),Box<dyn std::error::Error>> {
//     // let resp = reqwest::get("https://www.baidu.com/").await?.json::<HashMap<String,String>>().await?;
//     let resp = reqwest::get("https://www.rust-lang.org")
//         .await?
//         .text()
//         .await?;
//     println!("{:#?}",resp);
//     Ok(())
// }
use warp::Filter;
#[tokio::main]
async fn main(){
    let hello = warp::get().map(|| "hello,世界 !  你说ssss".to_string());
    warp::serve(hello).run(([127,0,0,1],9090)).await;
}