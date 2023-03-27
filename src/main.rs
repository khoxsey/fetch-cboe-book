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
    let exchange = "bzx";
    let response = client
        .get(format!("https://www.cboe.com/json/{exchange}/book/SPY"))
        .header("if-modified-since", "19:18:46 US/Eastern")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::NOT_MODIFIED => {
             eprintln!("HTTP 304 Not modified...skipping");
             //TODO: use ansi_term to add color codes?
        }

        reqwest::StatusCode::OK => {
            // add json parsing like the Success in this https://blog.logrocket.com/making-http-requests-rust-reqwest/
            println!("{}", response.text().await?);
            //TODO: eprintln status code, exchange, and timestamp
            //TODO: capture timestamp from response to use for next call's if-modified-since
        }

        other => {
            panic!("Unhandled status code: {:?}", other);
        }

    }
    
    Ok(())
}