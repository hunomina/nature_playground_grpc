mod nature_playground;
mod utils;

use nature_playground::plant_service_client::PlantServiceClient;
use nature_playground::{Empty, Plant};
use tonic::{Request as TonicRequest, Status};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request as HyperRequest, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use tonic::transport::Channel;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8888));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(app)) });
    let server = Server::bind(&addr).serve(make_svc);

    println!("Client started at {}", addr);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn app(req: HyperRequest<Body>) -> Result<Response<Body>, Infallible> {
    println!("Request {} {}", req.method(), req.uri().path());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => get_plants(req).await,
        (&Method::GET, "/status") => Ok(ok()),
        _ => Ok(not_found()),
    }
}

async fn get_plants(_req: HyperRequest<Body>) -> Result<Response<Body>, Infallible> {
    let plants = get_plants_rpc().await;

    if let Err(e) = plants {
        return Ok(Response::new(format!("{:?}", e).into()));
    }

    let plants = plants.unwrap();

    let response =
        plants
            .into_iter()
            .enumerate()
            .fold(String::new(), |mut response, (i, plant)| {
                response.push_str(format!("Received ({}): {:?}\n", i, plant).as_str());
                response
            });

    Ok(Response::new(response.into()))
}

async fn get_plants_rpc() -> Result<Vec<Plant>, ServerError> {
    let mut client = get_plant_service()
        .await
        .map_err(|e| ServerError::TransportError(e))?;

    let mut stream = client
        .get_plants(TonicRequest::new(Empty {}))
        .await
        .map_err(|s| ServerError::TonicStatus(s))?
        .into_inner();

    let mut plants = vec![];
    while let Some(plant) = stream
        .message()
        .await
        .map_err(|s| ServerError::TonicStatus(s))?
    {
        plants.push(plant);
    }

    Ok(plants)
}

async fn get_plant_service() -> Result<PlantServiceClient<Channel>, tonic::transport::Error> {
    let dest = utils::get_server_http_endpoint();
    let client = PlantServiceClient::connect(dest).await?;
    //println!("Connected to client {}", dest);
    Ok(client)
}

#[derive(Debug)]
enum ServerError {
    TransportError(tonic::transport::Error),
    TonicStatus(Status),
}

fn not_found() -> Response<Body> {
    let mut response = Response::new("Not found".into());
    *response.status_mut() = StatusCode::NOT_FOUND;
    response
}

fn ok() -> Response<Body> {
    Response::new("Ok".into())
}
