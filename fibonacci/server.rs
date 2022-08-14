extern crate futures;
extern crate hyper;
extern crate tokio;
extern crate tokio_stream;

use hyper::{
    body::HttpBody,
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};

use std::{convert::Infallible, sync::Arc};
use tokio::sync::RwLock;

pub struct HttpFibonacciServer {
    mem: Vec<i128>,
}

impl HttpFibonacciServer {
    #[inline]
    pub fn new() -> Self {
        Self { mem: vec![0, 1] }
    }

    #[inline]
    pub async fn start(this: Arc<RwLock<Self>>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(Server::bind(&([127, 0, 0, 1], 1337).into())
            .serve(make_service_fn(move |_conn: &AddrStream| {
                let this = this.clone();

                async move {
                    Ok::<_, Infallible>(service_fn(move |req| {
                        let this = this.clone();
                        async move { this.write().await.handle(req).await }
                    }))
                }
            }))
            .await?)
    }

    async fn handle(&mut self, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let fib = self.get_fibonacci(
            String::from_utf8(req.into_body().data().await.unwrap().unwrap().to_vec())
                .unwrap()
                .trim()
                .parse()
                .unwrap(),
        );

        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(format!("{}", fib)))
            .unwrap())
    }

    fn get_fibonacci(&mut self, index: usize) -> i128 {
        match self.mem.get(index) {
            Some(&number) => number,
            None => match index {
                0 => 0,
                1 => 1,
                _ => {
                    let number = self.get_fibonacci(index - 1) + self.get_fibonacci(index - 2);
                    self.mem.push(number);
                    number
                }
            },
        }
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    HttpFibonacciServer::start(Arc::new(RwLock::new(HttpFibonacciServer::new()))).await
}
