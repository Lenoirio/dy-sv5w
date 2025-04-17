use std::time::Duration;
use dy_sv5w::{DySv5w, Serial2ConsoleDebug, Serial2SerialPort};

#[tokio::main]
async fn main() {

    let port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_millis(250))
        .open();

    if let Ok(mut port) = port {
        let sport = Serial2SerialPort {
            serial: port
        };
    }


    let serial = Serial2ConsoleDebug {};
    let mut dy = DySv5w::new(serial);

    dy.set_volume(128).await;
    dy.play().await;
    dy.stop_playing().await;

    println!("{:?}", dy.query_play_status().await);
}
