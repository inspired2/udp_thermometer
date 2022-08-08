use std::net::{UdpSocket, ToSocketAddrs};

#[derive(Clone, Debug)]
pub struct Thermometer {
    pub name: String,
    pub state: Arc<Mutex<Temperature>>,
    socket: UdpSocket
}
enum TemperatureFormat {
    Fahrenheit,
    Celsius
}

impl Thermometer {
    pub fn new(name: &str, data_source_socket: impl ToSocketAddrs) -> Self {
        //connect to socket
        //start async task to recv temp update
        //update state
        let socket = UdpSocket::bind(data_source_socket);
        std::thread::spawn(|| {
            while let Ok(data) = socket.recv() {
                let temperature = Temperature::from_u8(data).unwrap();
                self.state.update(temperature).unwrap();
            }
        });
    }
    pub fn get_celsius(&self) -> i16 {
        self.get_temperature().as_celsius()
    }

    pub fn get_fahrenheit(&self) -> i16 {
        self.get_temperature().as_fahrenheit()
    }

    pub fn get_temperature(&self) -> Temperature {
        self.state
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
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