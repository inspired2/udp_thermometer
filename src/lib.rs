use std::{
    net::{ToSocketAddrs, UdpSocket},
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub struct Thermometer {
    pub name: String,
    pub state: Arc<Mutex<Temperature>>,
    //socket: UdpSocket
}
// enum TemperatureFormat {
//     Fahrenheit,
//     Celsius
// }

impl Thermometer {
    pub fn new(name: &str, addr: impl ToSocketAddrs) -> Result<Self, String> {
        let state = Arc::new(Mutex::new(Temperature::default()));
        let addr = addr
            .to_socket_addrs()
            .map_err(|_| "unable to parse provided socket addr".to_string())?
            .next()
            .ok_or_else(|| "ToSocketAddrs iterator is empty".to_owned())?;
        let socket = UdpSocket::bind(addr)
            .map_err(|_| format!("unable to connect to socket addr: {}", addr))?;
        let cloned_state = Arc::clone(&state);
        std::thread::spawn(move || {
            let mut buf = [0_u8; 5];
            while socket.recv(&mut buf).is_ok() {
                println!("received data from temperature broadcaster: {:?}", &buf);
                let new_temp = Temperature::from(buf);
                let mut temp = cloned_state
                    .lock()
                    .expect("unable to acquire mutex lock on Thermometer.state");
                *temp = new_temp;
            }
        });
        Ok(Self {
            name: name.to_owned(),
            state,
        })
    }
    pub fn get_temperature(&self) -> Result<Temperature, String> {
        let temperature_ref = self
            .state
            .lock()
            .map_err(|_| "unable to acquire mutex lock on Thermometer.state".to_string())?;
        Ok(*temperature_ref)
    }
    pub fn get_celsius(&self) -> Result<i16, String> {
        Ok(self.get_temperature()?.as_celsius())
    }

    pub fn get_fahrenheit(&self) -> Result<i16, String> {
        Ok(self.get_temperature()?.as_fahrenheit())
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
impl From<[u8; 5]> for Temperature {
    fn from(mut arr: [u8; 5]) -> Self {
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
