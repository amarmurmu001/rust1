use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use tokio;

// Function to handle incoming requests
async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response = Response::new(Body::from("Hello, World!"));
    Ok(response)
}

#[tokio::main]
async fn main() {
    // Address to bind to
    let addr = ([127, 0, 0, 1], 3000).into();

    // Create a service to handle requests
    let make_svc = make_service_fn(|_conn| {
        // service_fn converts the function into a `Service`
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    // Create the server and bind it to the address
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http://{}", addr);

    // Run the server
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
