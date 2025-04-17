use std::io;
use std::io::Read;

const CMD_START: u8 = 0xaa;


#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PlayState {
    Stop,
    Play,
    Pause,
    Unknown
}
pub struct DySv5w<T> {
    serial: T
}

impl<T> DySv5w<T>
where T: DySv5wSerialIO
{
    pub fn new(serial: T) -> DySv5w<T> {
        DySv5w {
            serial
        }
    }

    pub async fn set_volume(&mut self, volume: u8) {
        let mut cmd = [CMD_START, 0x13, 0x01, volume, 0x00];
        self.send_with_crc(&mut cmd).await;
    }

    pub async fn play(&mut self) {
        let mut cmd = [CMD_START, 0x02, 0x00, 0];
        self.send_with_crc(&mut cmd).await;
    }

    pub async fn pause(&mut self) {
        let mut cmd = [CMD_START, 0x03, 0x00, 0];
        self.send_with_crc(&mut cmd).await;
    }

    pub async fn stop(&mut self) {
        let mut cmd = [CMD_START, 0x04, 0x00, 0];
        self.send_with_crc(&mut cmd).await;
    }

    pub async fn previous(&mut self) {
        let mut cmd = [CMD_START, 0x05, 0x00, 0];
        self.send_with_crc(&mut cmd).await;
    }

    pub async fn volume_inc(&mut self) {
        let mut cmd = [CMD_START, 0x14, 0x00, 0];
        self.send_with_crc(&mut cmd).await;
    }

    pub async fn volume_dec(&mut self) {
        let mut cmd = [CMD_START, 0x15, 0x00, 0];
        self.send_with_crc(&mut cmd).await;
    }

    pub async fn next(&mut self) {
        let mut cmd = [CMD_START, 0x06, 0x00, 0];
        self.send_with_crc(&mut cmd).await;
    }

    pub async fn stop_playing(&mut self) {
        let mut cmd = [CMD_START, 0x10, 0x00, 0];
        self.send_with_crc(&mut cmd).await;
    }

    pub async fn query_play_status(&mut self) -> Option<PlayState> {
        let mut cmd = [CMD_START, 0x01, 0x00, 0];
        self.send_with_crc(&mut cmd).await;
        let mut rcv_buffer = [0u8; 1];
        if self.receive_answer(0x01, &mut rcv_buffer).await {
            match rcv_buffer[0] {
                0 => Some(PlayState::Stop),
                1 => Some(PlayState::Play),
                2 => Some(PlayState::Pause),
                _ => None
            }
        } else {
            None
        }
    }

    async fn send_with_crc(&mut self, cmd: &mut [u8]) {
        fill_in_crc(cmd);
        self.serial.send_data(cmd).await
    }
    async fn receive_answer(&mut self, cmd: u8, buffer: &mut [u8]) -> bool {
        let Some(start) = self.serial.read_byte().await else {
            return false;
        };
        if start != CMD_START {
            return false;
        }
        let Some(cmd_in) = self.serial.read_byte().await else {
            return false;
        };
        if cmd_in != cmd {
            return false;
        }
        let Some(mut len) = self.serial.read_byte().await else {
            return false;
        };
        if len as usize != buffer.len() {
            return false;
        }
        let mut pos = 0;
        while len > 0 {
            let Some(data) = self.serial.read_byte().await else {
                return false;
            };
            buffer[pos] = data;
            len -= 1;
            pos += 1;
        }
        let _ = self.serial.read_byte().await;  // ignore CRC for now
        true
    }
}

fn fill_in_crc(bytes: &mut [u8]) {
    if bytes.is_empty() {
        return;
    }
    let mut crc: u16 = 0;
    for b in bytes.iter() {
        crc += *b as u16;
    }
    bytes[bytes.len()-1] = (crc&0xff) as u8;
}

pub trait DySv5wSerialIO {
    fn send_data(&mut self, data: &[u8]) -> impl Future<Output = ()> + Send;
    fn read_byte(&mut self) -> impl Future<Output = Option<u8>> + Send;
}

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
