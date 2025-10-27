use deezer_rs::DeezerClient;
use deezer_rs::objects::*;

#[tokio::main]
async fn main() {
    let deezer = DeezerClient::new();

    let object = deezer.get::<Album>(302127).await.unwrap();

    println!("{:?}", object.artist.picture);
}
