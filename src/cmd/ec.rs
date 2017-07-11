use ecflash::{Ec, EcFlash, EcFile};
use uefi::status::{Error, Result};

use fs::{find, load};
use io::wait_key;
use shell::shell;

pub fn main() -> Result<()> {
    let uefi = unsafe { &mut *::UEFI };

    let (fs, _) = find("\\res\\firmware.nsh")?;

    (uefi.ConsoleOut.ClearScreen)(uefi.ConsoleOut)?;

    println!("Verifying EC");

    let (e_p, e_v, e_s) = match EcFlash::new(1) {
        Ok(mut ec) => {
            (ec.project(), ec.version(), ec.size())
        },
        Err(err) => {
            println!("EC Error: {}", err);
            return Err(Error::NotFound);
        }
    };

    println!("Flash Project: {}", e_p);
    println!("Flash Version: {}", e_v);
    println!("Flash Size: {} KB", e_s/1024);

    let (f_p, f_v, f_s) = {
        let mut file = EcFile::new(load("res\\firmware\\ec.rom")?);
        (file.project(), file.version(), file.size())
    };

    println!("File Project: {}", f_p);
    println!("File Version: {}", f_v);
    println!("File Size: {} KB", f_s/1024);

    if e_p != f_p {
        println!("Project Mismatch");
        return Err(Error::DeviceError);
    }

    if e_s != f_s {
        println!("Size Mismatch");
        return Err(Error::DeviceError);
    }

    println!("Press enter key to flash EC, any other to cancel");
    let c = wait_key()?;

    if c == '\r' || c == '\n' {
        let status = shell(&format!("fs{}:\\res\\firmware.nsh ec flash", fs))?;
        if status != 0 {
            println!("Failed to flash EC: {}", status);
            return Err(Error::DeviceError);
        }

        println!("Flashed EC successfully");
    } else {
        println!("Cancelled EC flashing");
    }

    Ok(())
}