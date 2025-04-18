use std::time::Duration;
use dy_sv5w::{DySv5w, DySv5wSerialIO, EqualizerMode};

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

        println!("Current play drive {:?}", dy.query_current_play_drive().await);
        println!("Current online drive {:?}", dy.query_current_online_drive().await);
        println!("Number of songs {:?}", dy.query_number_songs().await);


        dy.set_equalizer_mode(EqualizerMode::Rock).await;
        dy.set_volume(10).await;

        dy.specify_song(1).await;
        dy.play().await;

        // play() usually blocks the device-UART for a short time. Wait before using the next commands
        tokio::time::sleep(Duration::from_millis(500)).await;


        println!("PlayStatus {:?}", dy.query_play_status().await);
        println!("Current song# {:?}", dy.query_current_song().await);
    } else {
        println!("Failed to open serial port");
    }
}
