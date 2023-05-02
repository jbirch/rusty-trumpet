use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn hello_trumpet(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("🎺 🎺 🎺 🎺 🎺");
    Ok(Response::new(Body::from("Hello, 🎺!")))
}

#[tokio::main]
async fn main() {
    println!("commencing doot");

    let svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_trumpet))
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let server = Server::bind(&addr).serve(svc);

    if let Err(e) = server.await {
        eprintln!("doot doot error: {}", e);
    }

}

