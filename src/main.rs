use reqwest::{self, header::{ACCEPT, ACCEPT_LANGUAGE, DNT, REFERER, USER_AGENT, self}};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    //CBOE expects all these headers, it seems
    let mut headers = header::HeaderMap::new();
    headers.insert(ACCEPT, header::HeaderValue::from_static( "*/*"));
    headers.insert(ACCEPT_LANGUAGE, header::HeaderValue::from_static( "en-US,en;q=0.9"));
    headers.insert(DNT, header::HeaderValue::from_static("1"));
    headers.insert(REFERER, header::HeaderValue::from_static("https://www.cboe.com/us/equities/market_statistics/book/SPY/"));
    headers.insert(USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36 Edg/111.0.1661.51"));
    headers.insert("x-requested-with", header::HeaderValue::from_static("XMLHttpRequest"));
    headers.insert("sec-fetch-dest", header::HeaderValue::from_static("empty"));
    headers.insert("sec-fetch-mode", header::HeaderValue::from_static("cors"));
    headers.insert("sec-fetch-site", header::HeaderValue::from_static("same-origin"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    
    //Eventually: iterate over BZY, BZX, EGX, EGY (last two unsure)
    let response = client
        .get("https://www.cboe.com/json/bzx/book/SPY")
        .header("if-modified-since", "Thu, 23 Mar 2023 19:37:24 GMT")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::NOT_MODIFIED => {
            //TODO: this should go to stderr
             println!("Not modified...skipping");
        }

        reqwest::StatusCode::OK => {
            // add json parsing like the Success in this https://blog.logrocket.com/making-http-requests-rust-reqwest/
            println!("{}", response.text().await?);
        }

        other => {
            panic!("Unhandled status code: {:?}", other);
        }

    }
    
    Ok(())
}