use udp_thermometer::Thermometer;

fn main() -> Result<(), String> {
    let addr = "127.0.0.1:8080";
    let therm = Thermometer::new("therm", addr)?;
    loop {
        println!("Current temperature: {:?}", therm.get_temperature()?);
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
