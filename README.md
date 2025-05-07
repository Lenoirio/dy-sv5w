# dy-sv5w
This is a [no-std] crate which uses a trait to handle I/O. 
See the two examples on how to implement such an interface to your UART.
Although primarily designed for embedded usage, it uses ASYNC to profit from async-frameworks like Embassy.

## Simple API
The repository also contains the very limited documentation of the sound module ([DY-SV5W Voice Playback ModuleDatasheet.pdf](DY-SV5W%20Voice%20Playback%20ModuleDatasheet.pdf))
Most of the commands have been implemented. The best way to start with the module is a FTDI serial-interface to your PC.
That's how I developed the code before I switched to an embedded system.

```rust
        dy.set_equalizer_mode(EqualizerMode::Rock).await;
        dy.set_volume(10).await;
        dy.specify_song(1).await;
        dy.play().await;
```
The above example sets the Equalizer to use the Rock settings, volume to 10 (max value is 30). Then song number 1 is selected and playback is started.

*Hint*: The module sometimes needs (e.g., after issuing play()) a short delay before it accepts the next command.

Most of the commands are fire-and-forget commands. This means there is no ack/nack sent from the module. Thus, it's not possible to provide the API caller with success information.

The naming of the files on the module is a bit awkward. See the documentation but don't expect that the file called 00001.mp3 is always the song that you address with number 1.
It looks like the module rather counts the number in the FAT directory structure.

Another important hint: Although the module needs +5V, the level for I/O-pins (including the UART) is *3.3 V*

The device can't operate as a USB storage-device. Neither Linux nor Windows can detect it as a drive.
It will be reported with 'lsusb' but not as a storage device.
If you intend to power the module via the USB-port, make sure that your USB-cable is not plugged in a PC or the USB-cable just has no data-wires.
Reason: In case the device is communicating via USB, it will not start in UART mode.

## Interfacing with your UART
As mentioned, you need to implement the trait DySv5wSerialIO and use it within the DySv5w struct. 
In the examples folder of the repository, you can find an example for the popular serialport crate as well as an implementation using console I/O for debugging.

Nevertheless, here is another example of how I use the crate on my ESP32 within the embassy environment:
```rust
struct UartSoundModule {
    uart_rx: UartRx<'static,Async>,
    uart_tx: UartTx<'static,Async>
}

impl DySv5wSerialIO for UartSoundModule {
    async fn send_data(&mut self, data: &[u8]) {
        let _ = embedded_io_async::Write::write(&mut self.uart_tx, data).await;
        let _ = embedded_io_async::Write::flush(&mut self.uart_tx).await;
    }

    async fn read_byte(&mut self) -> Option<u8> {
        let mut rbuf = [0u8; 1];

        let r = with_timeout(Duration::from_secs(1), async {
            embedded_io_async::Read::read_exact(&mut self.uart_rx, &mut rbuf).await
        }).await;

        match r {
            Ok(Ok(_)) => {
                Some(rbuf[0])
            }
            _ => { None }
        }

    }
}

```
