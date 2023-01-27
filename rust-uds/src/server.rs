use tonic::{transport::{Server, server::UdsConnectInfo}, Request, Response, Status};

use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;

use std::path::Path;

use greeter::greeter_server::{Greeter, GreeterServer};
use greeter::{HelloRequest, HelloResponse};

// Import the generated proto-rust file into a module
pub mod greeter {
    tonic::include_proto!("greeter");
}

// Implement the service skeleton for the "Greeter" service
// defined in the proto
#[derive(Debug, Default)]
pub struct MyGreeter {}

// Implement the service function(s) defined in the proto
// for the Greeter service (SayHello...)
#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn sayhello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        // let conn_info = request.extensions().get::<UdsConnectInfo>().unwrap();
        // println!("Got a request {:?} with info {:?}", request, conn_info);
        let message = request.into_inner();
        let response = greeter::HelloResponse {
            message: format!(
                "Hello {}! Your id-incarnation is {}-{} and your secret is <{}>",
                message.name,
                message.id,
                message.incarnation,
                message.inner.as_ref().map_or("", |inner| &inner.secret)
            ),
        };

        Ok(Response::new(response))
    }
}

// Use the tokio runtime to run our server
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "[::1]:50051".parse()?;

    // println!("Starting gRPC Server...");
    // Server::builder()
    //     .add_service(GreeterServer::new(greeter))
    //     .serve(addr)
    //     .await?;

    let path = "/tmp/tonic/greeter4";

    std::fs::create_dir_all(Path::new(path).parent().unwrap())?;

    let greeter = MyGreeter::default();

    let uds = UnixListener::bind(path)?;
    let uds_stream = UnixListenerStream::new(uds);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve_with_incoming(uds_stream)
        .await?;

    Ok(())
}
