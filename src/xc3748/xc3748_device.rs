use super::output_device::OutputDevice;

#[cfg(not(feature = "std"))]
use embedded_hal_nb::serial::Write;

#[cfg(feature = "x64")]
use serialport::SerialPort;

#[cfg(feature = "x64")]
pub struct Xc3748Device {
    serial_port: Box<dyn SerialPort>,
}

#[cfg(feature = "x64")]
impl Xc3748Device {
    pub fn new(serialport: Box<dyn SerialPort>) -> Self {
        Self {
            serial_port: serialport,
        }
    }
}

#[cfg(feature = "x64")]
impl OutputDevice for Xc3748Device {
    fn write_data(&mut self, data: &[u8]) -> () {
        self.serial_port.write(data).expect("Write failed");
    }
}

#[cfg(feature = "embedded")]
pub struct Xc3748Device<T: Write> {
    uart: T,
}

#[cfg(feature = "embedded")]
impl<T: embedded_hal_nb::serial::Write> Xc3748Device<T> {
    pub fn new(uart: T) -> Self {
        Self { uart }
    }
}

#[cfg(feature = "embedded")]
impl<T: embedded_hal_nb::serial::Write> OutputDevice for Xc3748Device<T> {
    fn write_data(&mut self, data: &[u8]) -> () {
        for &byte in data {
            self.uart.write(byte).unwrap();
        }
    }
}
