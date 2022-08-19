mod udp_connection;

use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
pub use udp_connection::{ThermConnection, ThermConnectionBuilder};

#[derive(Debug)]
pub struct Thermometer {
    pub name: String,
    pub state: Arc<Mutex<Temperature>>,
}

impl Thermometer {
    pub async fn new<const N: usize>(
        name: &str,
        addr: SocketAddr,
        peer_addr: SocketAddr,
    ) -> Result<Self, String> {
        let state = Arc::new(Mutex::new(Temperature::default()));
        let mut connection: ThermConnection<N> = ThermConnectionBuilder::new()
            .with_local_addr(addr)
            .await
            .build::<N>();
        let cloned_state = Arc::clone(&state);

        tokio::spawn(async move {
            loop {
                println!("enter Loop");
                let mut buf = [0_u8; N];
                let res = connection.recv_from(&mut buf, peer_addr).await;
                println!("received: {:?}", res);
                let new_temp = Temperature::from(buf);
                let mut temp = cloned_state.lock().await;
                *temp = new_temp;
            }
        });

        Ok(Self {
            name: name.to_owned(),
            state,
        })
    }
    pub async fn get_temperature(&self) -> Result<Temperature, String> {
        let temperature_ref = self.state.lock().await;
        Ok(*temperature_ref)
    }
    pub async fn get_celsius(&self) -> Result<i16, String> {
        Ok(self.get_temperature().await?.as_celsius())
    }

    pub async fn get_fahrenheit(&self) -> Result<i16, String> {
        Ok(self.get_temperature().await?.as_fahrenheit())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
}
impl Default for Temperature {
    fn default() -> Self {
        Self::Celsius(f32::default())
    }
}
impl<const N: usize> From<[u8; N]> for Temperature {
    fn from(mut arr: [u8; N]) -> Self {
        let temp_kind = arr[0];
        let data = unsafe {
            let ptr = arr.as_mut_ptr().add(1);
            let data: [u8; 4] = std::slice::from_raw_parts(ptr, 4).try_into().unwrap();
            data
        };
        let temp = f32::from_le_bytes(data);
        match temp_kind {
            1 => Temperature::Fahrenheit(temp),
            0 => Temperature::Celsius(temp),
            _ => panic!(
                "Invalid temperature format (0 - Celsius, 1 - Fahrenheit). Received - {}",
                temp_kind
            ),
        }
    }
}
impl Temperature {
    pub fn as_celsius(&self) -> i16 {
        match *self {
            Temperature::Celsius(c) => c.round() as i16,
            Temperature::Fahrenheit(f) => (((f - 32.0) * 5.0) / 9.0).round() as i16,
        }
    }

    pub fn as_fahrenheit(&self) -> i16 {
        match *self {
            Temperature::Fahrenheit(f) => f.round() as i16,
            Temperature::Celsius(c) => (c * 1.8 + 32.0).round() as i16,
        }
    }
}
