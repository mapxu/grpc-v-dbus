use std::{error::Error, future::pending};
use zbus::{dbus_interface, ConnectionBuilder};

struct Greeter {
    count: u64,
}

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    // Can be `async` as well.
    fn say_hello(&mut self, name: &str, incarnation: i32, id: i64) -> String {
        self.count += 1;
        println!("Received request from {name}");
        format!(
            "Hello {}! {}-{}: I have been called {} times.",
            name, incarnation, id, self.count
        )
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let greeter = Greeter { count: 0 };
    let c = ConnectionBuilder::session()?
        .name("org.zbus.MyGreeter")?
        .serve_at("/org/zbus/MyGreeter", greeter)?
        .build()
        .await?;

    println!("{:?}, {}", c.unique_name(), c.server_guid());

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}
