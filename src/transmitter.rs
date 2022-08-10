use std::mem::MaybeUninit;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use rand::*;

//const TEMPERATURE_FORMAT: u8 = TempFormat::Fahrenheit u8;

const TEMPERATURE_FORMAT: u8 = TempFormat::Celsius as u8;
fn main() {
    let addr = "127.0.0.1:8081";
    let sock = UdpSocket::bind(addr).unwrap();
    sock.connect("127.0.0.1:8080").expect("can't connect to receiver");
    let mut temperature_gen = TemperatureGenerator::new();
    loop {
        let current_temp = temperature_gen.get_current();
        //SAFETY:
        // f32 is always 4 bytes
        let temp_data: [u8;4] = unsafe {std::mem::transmute(current_temp)};
        let data: [u8; 5] = concat_helper([TEMPERATURE_FORMAT], temp_data);
        sock.send(&data).expect("failed to send data from temperature transmitter");

        thread::sleep(Duration::from_secs(1));
    }
}
struct TemperatureGenerator {
    current: f32,
    rng: rand::rngs::ThreadRng
}

impl TemperatureGenerator {
    fn new() -> Self {
        Self { current: 10., rng: rand::thread_rng() }
    }

    fn get_current(&mut self) -> f32 {
        let delta: f32 = self.rng.gen_range(-1. .. 1.);
        self.current += delta;
        self.current
    }
}

enum TempFormat {
    Celsius = 0,
    #[allow(unused)]
    Fahrenheit = 1
}

fn concat_helper<const N: usize, const L: usize, const NL: usize, T>(arr1: [T; N], arr2: [T;L]) -> [T; NL] {
    let mut result = MaybeUninit::uninit();
    let dest = result.as_mut_ptr() as *mut T;
    unsafe {
        std::ptr::copy_nonoverlapping(arr1.as_ptr(), dest, N);
        std::ptr::copy_nonoverlapping(arr2.as_ptr(), dest.add(N), L);
        std::mem::forget(arr1);
        std::mem::forget(arr2);
        result.assume_init()
    }
}