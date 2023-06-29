
pub async fn get(url: &str) -> String {
    let body = reqwest::get(url).await.unwrap().text().await.unwrap();
    
    body
}

pub async fn post(url: &str, body: &str) -> String {
    //let res = surf::post(url).body_string(body.to_string()).recv_string().await.unwrap();
    //hyper::Client::new();

    //let res = "".to_string();
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


