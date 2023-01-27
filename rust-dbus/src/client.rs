use tokio::time::{interval, sleep, Duration};
use zbus::{dbus_proxy, Connection, Result as ZResult};

#[dbus_proxy(
    interface = "org.zbus.MyGreeter1",
    default_service = "org.zbus.MyGreeter",
    default_path = "/org/zbus/MyGreeter"
)]
trait MyGreeter {
    async fn say_hello(&self, name: &str, incarnation: i32, id: i64) -> ZResult<String>;
}

async fn send_request(conn: &Connection, incarnation: i32, id: i64) -> ZResult<()> {
    // `dbus_proxy` macro creates `MyGreaterProxy` based on `Notifications` trait.
    let proxy = MyGreeterProxy::new(conn).await?;
    let reply = proxy
        .say_hello(format!("Mapo-{incarnation}-{id}").as_str(), incarnation, id)
        .await?;
    // println!("{reply}");
    Ok(())
}

async fn request_thread(id: i64, num_requests: u64, time_between_requests: u64) -> ZResult<()> {
    let connection = Connection::session().await?;

    let mut request_interval = interval(Duration::from_millis(time_between_requests));

    for incarnation in 100..100 + std::convert::TryInto::<i32>::try_into(num_requests).unwrap() {
        request_interval.tick().await;
        send_request(&connection, incarnation, id).await?;
    }

    Ok(())
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_requests_per_sec = 500;
    let num_threads = 10;
    let time_between_requests: u64 = 1000 * num_threads / num_requests_per_sec;
    let total_time_in_secs = 20;
    let num_requests_per_thread = total_time_in_secs * num_requests_per_sec / num_threads;

    let mut threads = vec![];

    println!("Pausing...");
    sleep(Duration::from_millis(10000)).await;
    println!("Starting.");

    for id in 0..num_threads.try_into().unwrap() {
        threads.push(tokio::spawn(async move {
            request_thread(id.clone(), num_requests_per_thread, time_between_requests)
                .await
                .unwrap();
        }));
    }

    for thread in threads {
        thread.await?;
    }

    Ok(())
}
