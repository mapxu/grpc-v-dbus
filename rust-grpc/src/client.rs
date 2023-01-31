use greeter::greeter_client::GreeterClient;
use greeter::HelloRequest;
use greeter::InnerRequest;
use tokio::time::{interval, sleep, Duration};
use std::{fs, sync::Arc};

const num_requests_per_sec: u64 = 1000;
const num_threads: u64 = 50;

// Import the generated proto-rust file into a module
pub mod greeter {
    tonic::include_proto!("greeter");
}

async fn send_request(
    id: i64,
    payload: Arc<String>,
    num_requests: u64,
    time_between_requests: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?; // Need a new client for each thread
    let mut request_interval = interval(Duration::from_millis(time_between_requests));

    for incarnation in 100..100 + std::convert::TryInto::<i32>::try_into(num_requests).unwrap() {
        request_interval.tick().await;
        let request = tonic::Request::new(HelloRequest {
            name: "Tonic test".into(),
            id,
            incarnation,
            inner: Some(InnerRequest {
                secret: format!("secret-{}-{}", incarnation, id),
            }),
            payload: (*payload).clone(),
        });

        // println!("Sending request to gRPC Server...");
        let response = client.sayhello(request).await.unwrap();

        // println!("{}", response.into_inner().message);
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time_between_requests: u64 = 1000 * num_threads / num_requests_per_sec;
    let total_time_in_secs = 20;
    let num_requests_per_thread = total_time_in_secs * num_requests_per_sec / num_threads;

    let mut threads = vec![];

    let payload = Arc::new(fs::read_to_string("../payload.txt")?);

    println!("Pausing...");
    sleep(Duration::from_millis(10000)).await;
    println!("Starting.");

    for id in 0..num_threads.try_into().unwrap() {
        let new_payload = payload.clone();
        threads.push(tokio::spawn(async move {
            send_request(id.clone(), new_payload, num_requests_per_thread, time_between_requests)
                .await
                .unwrap();
        }));
    }

    for thread in threads {
        thread.await?;
    }

    Ok(())
}
