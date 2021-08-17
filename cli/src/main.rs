use active911::request;
use std::{
    env,
    error::Error,
    io::{self, Write},
};
use tokio::runtime::Builder as RuntimeBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let key = if let Ok(key) = env::var("ACTIVE911_KEY") {
        key
    } else {
        println!("ACTIVE911_KEY must be set");

        return Ok(());
    };

    let runtime = RuntimeBuilder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()?;
    let response = runtime.block_on(request::alarms(&key))?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for alarm in response.alarms {
        handle.write_all(alarm.description.as_bytes())?;
        handle.write_all(b" - ")?;
        handle.write_all(alarm.pretty_date.as_bytes())?;
        handle.write_all(b"\n  ")?;

        if !alarm.place.is_empty() {
            handle.write_all(alarm.place.as_bytes())?;
            handle.write_all(b" ")?;
        }

        handle.write_all(alarm.address.as_bytes())?;
        handle.write_all(b", ")?;
        handle.write_all(alarm.city.as_bytes())?;
        handle.write_all(b", ")?;
        handle.write_all(alarm.state.as_bytes())?;
        handle.write_all(b"\n")?;
    }

    Ok(())
}
