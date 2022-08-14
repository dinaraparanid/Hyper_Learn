extern crate hyper;
extern crate tokio;

use hyper::{body::HttpBody, Body, Client, Method, Request};

#[inline]
async fn get_fibonacci(index: u64) -> Result<i128, Box<dyn std::error::Error>> {
    let client = Client::new();

    Ok(String::from_utf8(
        client
            .request(
                Request::builder()
                    .uri("http://127.0.0.1:1337")
                    .method(Method::GET)
                    .body(Body::from(format!("{}", index)))?,
            )
            .await?
            .into_body()
            .data()
            .await
            .unwrap()?
            .to_vec(),
    )?
    .trim()
    .parse()?)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(for ind in 0..100 {
        println!("{}", get_fibonacci(ind).await?)
    })
}
