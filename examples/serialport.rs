use std::time::Duration;
use dy_sv5w::{DySv5w, DySv5wSerialIO};

pub struct Serial2SerialPort {
    pub serial: Box<dyn serialport::SerialPort>
}

impl DySv5wSerialIO for Serial2SerialPort {
    async fn send_data(&mut self, data: &[u8]) {
        let _ = self.serial.write_all(data);
        let _ = self.serial.flush();
    }

    async fn read_byte(&mut self) -> Option<u8> {
        let mut rcv_buffer = [0u8; 1];
        let result = self.serial.read_exact(&mut rcv_buffer);
        match result {
            Ok(_) => Some(rcv_buffer[0]),
            Err(_) => None
        }
    }
}

#[tokio::main]
async fn main() {

    let port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_millis(250))
        .open();

    if let Ok(mut port) = port {
        let sport = Serial2SerialPort {
            serial: port
        };

        let mut dy = DySv5w::new(sport);
        dy.set_volume(128).await;
        println!("{:?}", dy.query_play_status().await);
    
    } else {
        println!("Failed to open serial port");
    }
}
