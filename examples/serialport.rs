use std::time::Duration;
use dy_sv5w::{Drive, DySv5w, DySv5wSerialIO};

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

    if let Ok(port) = port {
        let sport = Serial2SerialPort {
            serial: port
        };

        let mut dy = DySv5w::new(sport);
        dy.set_volume(128).await;
        println!("PlayStatus {:?}", dy.query_play_status().await);

        dy.switch_specified_drive(Drive::Flash).await;

        println!("Current play drive {:?}", dy.query_current_play_drive().await);
        println!("Current online drive {:?}", dy.query_current_online_drive().await);
    } else {
        println!("Failed to open serial port");
    }
}
