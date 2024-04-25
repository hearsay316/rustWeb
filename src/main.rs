use std::io::{Error, ErrorKind};
use std::str::FromStr;
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
#[derive(Debug)]
struct Question{
    id:QuestionId,
    title:String,
    content:String,
    tags:Option<Vec<String>>
}
#[derive(Debug)]
struct QuestionId(String);

impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
      match id.is_empty() {
          false => Ok(QuestionId(id.to_string())),
          true=> Err(Error::new(ErrorKind::InvalidInput,"no id provided"))
      }
    }
}
impl Question {
    fn new (id:QuestionId,title:String,content:String,tags:Option<Vec<String>>)->Self{
        Question{
            id,
            title,
            content,
            tags
        }
    }
}
async fn get_questions()->Result<impl warp::Reply,warp::Rejection>{
    let  question = Question::new()
    Ok("hello,世界 !  你说ssss".to_string())
}
#[tokio::main]
async fn main(){
    let hello = warp::get().map(|| "hello,世界 !  你说ssss".to_string());
    warp::serve(hello).run(([127,0,0,1],9090)).await;
}

//http://10.203.67.230/weboffice/office/w/00d4929e86460d4ec73c3c36d951ed1f2708912bd5?_w_appid=jichengyingyong&_w_third_appid=jichengyingyong&_w_third_file_id=00d4929e86460d4ec73c3c36d951ed1f2708912bd5&route_key=0&token=&simple&hidecmb&readonly&__doc_route_key=0
//http://10.203.67.230/weboffice/office/w/00d4929e86460d4ec73c3c36d951ed1f2708912bd5?_w_appid=jichengyingyong&_w_third_appid=jichengyingyong&_w_third_file_id=00d4929e86460d4ec73c3c36d951ed1f2708912bd5&route_key=0&token=&wpsCachePreview&hideHeader&readonly&__doc_route_key=0
// http://10.203.67.230/weboffice/office/w/00d4929e86460d4ec73c3c36d951ed1f2708912bd5?_w_appid=jichengyingyong&_w_third_appid=jichengyingyong&_w_third_file_id=00d4929e86460d4ec73c3c36d951ed1f2708912bd5&route_key=0&token=&wpsPreview=1110110&simple&hidecmb&readonly&__doc_route_key=0