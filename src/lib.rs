#![no_std]
use core::future::Future;
const CMD_START: u8 = 0xaa;


#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PlayState {
    Stop,
    Play,
    Pause,
    Unknown
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Drive {
    USB,
    SD,
    Flash,
    NoDevice
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum EqualizerMode {
    Normal,
    Pop,
    Rock,
    Jazz,
    Classic
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

    /// volume is in the range of 0..30
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
    
    pub async fn specify_song(&mut self, song: u16) {
        let high = (song >> 8) as u8;
        let low = (song & 0xFF) as u8;
        let mut cmd = [CMD_START, 0x07, 0x02, high, low, 0];
        self.send_with_crc(&mut cmd).await;
    }

    pub async fn set_cycle_times(&mut self, times: u16) {
        let high = (times >> 8) as u8;
        let low = (times & 0xFF) as u8;
        let mut cmd = [CMD_START, 0x19, 0x02, high, low, 0];
        self.send_with_crc(&mut cmd).await;
    }

    pub async fn set_equalizer_mode(&mut self, mode: EqualizerMode) {
        let mut cmd = [CMD_START, 0x1a, 0x01, mode.to_u8(), 0];
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

    pub async fn switch_specified_drive(&mut self, drive: Drive) {
        let mut cmd = [CMD_START, 0x0b, 0x01, drive.to_u8(), 0];
        self.send_with_crc(&mut cmd).await;
    }

    pub async fn query_current_play_drive(&mut self) -> Option<Drive> {
        let mut cmd = [CMD_START, 0x0a, 0x00, 0];
        self.send_with_crc(&mut cmd).await;
        let mut rcv_buffer = [0u8; 1];
        if self.receive_answer(0x0a, &mut rcv_buffer).await {
            Some(Drive::from_u8(rcv_buffer[0]))
        } else {
            None
        }
    }

    pub async fn query_current_online_drive(&mut self) -> Option<Drive> {
        let mut cmd = [CMD_START, 0x09, 0x00, 0];
        self.send_with_crc(&mut cmd).await;
        let mut rcv_buffer = [0u8; 1];
        if self.receive_answer(0x09, &mut rcv_buffer).await {
            Some(Drive::from_u8(rcv_buffer[0]))
        } else {
            None
        }
    }

    pub async fn query_number_songs(&mut self) -> Option<u16> {
        let mut cmd = [CMD_START, 0x0c, 0x00, 0];
        self.send_with_crc(&mut cmd).await;
        let mut rcv_buffer = [0u8; 2];
        if self.receive_answer(0x0c, &mut rcv_buffer).await {
            Some(get_word(&rcv_buffer))
        } else {
            None
        }
    }

    pub async fn query_current_song(&mut self) -> Option<u16> {
        let mut cmd = [CMD_START, 0x0d, 0x00, 0];
        self.send_with_crc(&mut cmd).await;
        let mut rcv_buffer = [0u8; 2];
        if self.receive_answer(0x0d, &mut rcv_buffer).await {
            Some(get_word(&rcv_buffer))
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

fn get_word(bytes: &[u8; 2]) -> u16 {
    let mut word: u16 = (bytes[0] as u16) << 8;
    word |= bytes[1] as u16;
    word
}

pub trait DySv5wSerialIO {
    fn send_data(&mut self, data: &[u8]) -> impl Future<Output = ()>;
    fn read_byte(&mut self) -> impl Future<Output = Option<u8>>;
}

impl Drive {
    fn from_u8(value: u8) -> Drive {
        match value {
            0 => Drive::USB,
            1 => Drive::SD,
            2 => Drive::Flash,
            _ => Drive::NoDevice,
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            Drive::USB => { 0 }
            Drive::SD => { 1 }
            Drive::Flash => { 2 }
            Drive::NoDevice => { 0xff }
        }
    }
}

impl EqualizerMode {
    fn to_u8(self) -> u8 {
        match self {
            EqualizerMode::Normal => { 0 }
            EqualizerMode::Pop => { 1 }
            EqualizerMode::Rock => { 2 }
            EqualizerMode::Jazz => { 3 }
            EqualizerMode::Classic => { 4 }
        }
    }
}