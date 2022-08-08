#[tokio::main]
fn main() {
    let addr = "127.0.0.1:8080";
    let sock = UdpSocket::bind(addr);
    loop {
        let temp = rand::rng(20..22);
        sock.send(temp);
        thread::sleep(Duration::from_secs(1));
    }
}