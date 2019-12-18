/// Simple webserver.
/// 
/// ## TODO
/// - add db access stuff
/// - handle different paths
/// - etc

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTING to /echo");
        },
        (&Method::POST, "/echo") => {
            // echo request
            *response.body_mut() = req.into_body();
        },
        (&Method::POST, "/echo/reverse") => {
            let full_body = hyper::body::to_bytes(req.into_body()).await?;

            // Reverse body around!
            let reversed = full_body.iter()
                .rev()
                .cloned()
                .collect::<Vec<u8>>();

            *response.body_mut() = reversed.into();
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}

async fn shutdown_signal() {
    // Wait for ^C
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install ^C signal handler!");
}

#[tokio::main]
async fn main() {
    // Bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127,0,0,1], 3000));

    // Create service
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(echo))
    });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
