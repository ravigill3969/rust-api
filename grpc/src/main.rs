mod proto {
    tonic::include_proto!("hello");
}

use tonic::{transport::Server, Request, Response, Status};
use proto::greeter_server::{Greeter, GreeterServer};
use proto::{HelloReply, HelloRequest};

#[derive(Default)]
pub struct HelloService;

#[tonic::async_trait]
impl Greeter for HelloService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        let reply = HelloReply {
            message: format!("Hello, {}!", name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = HelloService::default();

    println!("🚀 gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
