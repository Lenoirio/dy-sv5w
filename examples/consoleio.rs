use std::io;
use dy_sv5w::{DySv5w, DySv5wSerialIO};

pub struct Serial2ConsoleDebug {}

impl DySv5wSerialIO for Serial2ConsoleDebug {
    async fn send_data(&mut self, data: &[u8]) {
        print!("Serial OUT: ");
        for b in data.iter() {
            print!("0x{:02x} ", b);
        }
        println!();
    }

    async fn read_byte(&mut self) -> Option<u8> {
        println!("Enter hex value: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        let input = input.trim();

        let hex_str = input.strip_prefix("0x")
            .or_else(|| input.strip_prefix("0X"))
            .unwrap_or(input);

        u8::from_str_radix(hex_str, 16).ok()
    }
}

#[tokio::main]
async fn main() {
    let serial = Serial2ConsoleDebug {};
    let mut dy = DySv5w::new(serial);

    dy.set_volume(128).await;
    dy.play().await;
    dy.stop_playing().await;

    println!("{:?}", dy.query_play_status().await);
}
