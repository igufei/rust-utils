use std::collections::HashMap;

pub async fn get(url: &str) -> String {
    let body = reqwest::get(url).await.unwrap().text().await.unwrap();
    body
}

pub async fn post(url: &str,map:&HashMap<String,String>) -> String {
    let client = reqwest::Client::new();
    let body = client
        .post(url)
        .json(&map)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    body
}
