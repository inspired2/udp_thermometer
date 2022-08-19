use std::net::ToSocketAddrs;

use tokio::time::sleep;
use udp_thermometer::Thermometer;

const PACKET_LEN: usize = 5;
#[tokio::main]
async fn main() -> Result<(), String> {
    let addr = "127.0.0.1:8080".to_socket_addrs().unwrap().next().unwrap();
    let peer = "127.0.0.1:8081".to_socket_addrs().unwrap().next().unwrap();

    let therm = Thermometer::new::<PACKET_LEN>("therm", addr, peer).await?;
    println!("Created thermometer: {:?}", therm);
    loop {
        println!("Current temperature: {:?}", therm.get_temperature().await?);
        sleep(std::time::Duration::from_secs(2)).await;
    }
}
