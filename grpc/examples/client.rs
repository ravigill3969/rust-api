use tonic::Request;
use hello::greeter_client::GreeterClient;
use hello::HelloRequest;

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = Request::new(HelloRequest {
        name: "Engineer".into(),
    });

    let response = client.say_hello(request).await?;

    println!("Response: {:?}", response.into_inner().message);
    Ok(())
}
