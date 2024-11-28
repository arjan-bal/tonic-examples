use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_service_server::{GreeterService, GreeterServiceServer};
use hello_world::{SayHelloRequest, SayHelloResponse};

pub mod hello_world {
    tonic::include_proto!("helloworld.v1");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl GreeterService for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = SayHelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
