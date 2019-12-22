/// Simple webserver.
/// 
/// ## TODO
/// - add db access stuff
/// - handle different paths
/// - etc
/// 

use std::convert::Infallible;
use std::net::SocketAddr;
use std::error::Error;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use webserver_db::models::Page;


mod parser;

// bad naming
fn get_page(path: Vec<String>) -> Result<Page, std::io::Error> {
    let connection = webserver_db::establish_connection();
    let root_page = webserver_db::find_page(&connection, -1, "/")?;

    let mut parent = root_page.id;

    for name in path.iter().next() {
        let page = webserver_db::find_page(&connection, parent, name)?;

        // end of url, so finish return
        if name.next() == None {
            page
        }
    }

    Err();
}


async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    // This is needed because if you take the path directly,
    // you are borrowing, and doing body::from() on a borrow
    // means the string is destroyed when you return the function
    let path = String::from(req.uri().path());

    match (req.method(), path) {
        (&Method::GET, path) => {
            let url = parser::parse_url(path.as_str());
            match get_page(url) {
                Ok(page) => {
                    *response.body_mut() = Body::from(page.name);
                }
                _ => {
                    *response.status_mut() = StatusCode::NOT_FOUND;
                },
            };
        },
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
