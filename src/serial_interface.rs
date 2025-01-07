use serialport;
use std::io::{Read, Write};

pub struct LinuxInterfaceDriver {
    serial: serialport::TTYPort,
}

impl LinuxInterfaceDriver {
    pub fn new(serial: serialport::TTYPort) -> LinuxInterfaceDriver {
        LinuxInterfaceDriver { serial }
    }
}

impl embedded_serial::MutBlockingTx for LinuxInterfaceDriver {
    type Error = ();

    fn putc(&mut self, ch: u8) -> Result<(), Self::Error> {
        self.serial.write(&[ch]).unwrap();
        Ok(())
    }
}

impl embedded_serial::MutNonBlockingRx for LinuxInterfaceDriver {
    type Error = ();

    fn getc_try(&mut self) -> Result<Option<u8>, Self::Error> {
        let mut buf = [0u8];
        match self.serial.read(&mut buf) {
            Ok(_) => Ok(Some(buf[0])),
            Err(_) => Ok(None),
        }
    }
}
