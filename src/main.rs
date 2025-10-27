use deezer_rs::DeezerClient;
use deezer_rs::objects::*;

#[tokio::main]
async fn main() {
    let deezer = DeezerClient::new();

    let object = deezer.get::<Chart>(0).await.unwrap();

    println!("{object:?}");
}
