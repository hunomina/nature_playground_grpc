use crate::nature_playground::{
    plant_service_server::PlantService, Empty, NameCriteria, Plant, SizeCriteria,
};

use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use tonic::{Request, Response, Status};

pub struct MyPlantService {
    plants: Arc<Vec<Plant>>,
}

#[tonic::async_trait]
impl PlantService for MyPlantService {
    type GetPlantsStream = ReceiverStream<Result<Plant, Status>>;
    async fn get_plants(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<Self::GetPlantsStream>, Status> {
        let (tx, rx) = mpsc::channel(5);
        let plants = self.plants.clone();

        tokio::spawn(async move {
            for plant in &plants[..] {
                tx.send(Ok(plant.clone())).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type GetPlantsBySizeStream = ReceiverStream<Result<Plant, Status>>;
    async fn get_plants_by_size(
        &self,
        request: Request<SizeCriteria>,
    ) -> Result<Response<Self::GetPlantsBySizeStream>, Status> {
        unimplemented!()
    }

    async fn get_plant_by_name(
        &self,
        request: Request<NameCriteria>,
    ) -> Result<Response<Plant>, Status> {
        unimplemented!()
    }

    async fn create_plant(
        &self,
        request: tonic::Request<Plant>,
    ) -> Result<Response<Empty>, Status> {
        let plant = request.get_ref();
        println!("Received plant {:?}", plant);
        Ok(Response::new(Empty {}))
    }
}

impl MyPlantService {
    pub fn new(plants: Vec<Plant>) -> Self {
        Self {
            plants: Arc::new(plants),
        }
    }
}
