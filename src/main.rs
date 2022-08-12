use std::net::Ipv4Addr;

use udp_thermometer::Thermometer;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), String> {
    let addr = "127.0.0.1".parse::<Ipv4Addr>().map_err(|e| e.to_string())?;
    let port: u16  = 8080;
    let therm = Thermometer::new("therm", (addr, port)).await?;
    loop {
        println!("Current temperature: {:?}", therm.get_temperature().await?);
        sleep(std::time::Duration::from_secs(2)).await;
    }
}
