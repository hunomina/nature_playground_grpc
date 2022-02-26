mod nature_playground;
mod services;
mod storage;
mod utils;

use services::plant::MyPlantService;
use storage::{JsonFile, Storage};
use tonic::transport::Server;

use nature_playground::plant_service_server::PlantServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = JsonFile::new(String::from("data/plants.json"), true);
    let plant_service = MyPlantService::new(storage.all());

    let addr = utils::get_server_address();
    println!("Server starting on {}", addr);

    Server::builder()
        .add_service(PlantServiceServer::new(plant_service))
        .serve(addr.parse().unwrap())
        .await?;

    Ok(())
}
