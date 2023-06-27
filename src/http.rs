
pub async fn get(url: &str) -> String {
    let body = reqwest::get(url).await.unwrap().text().await.unwrap();
    body
}

pub async fn post(url: &str, body: &str) -> String {
    let client = reqwest::Client::new();
    let body = client
        .post(url)
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    body
}
