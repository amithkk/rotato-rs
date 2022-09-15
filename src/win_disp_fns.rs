use winsafe::{ChangeDisplaySettingsEx, co, DEVMODE, DISPLAY_DEVICE, EnumDisplayDevices, EnumDisplaySettingsEx};
use winsafe::co::CDS;
use winsafe::prelude::NativeBitflag;
use crate::{ROT0, ROT180, ROT270, ROT90, RotationType};

pub fn enumerate_all_displays() -> Vec<DISPLAY_DEVICE> {
    let mut devices: Vec<DISPLAY_DEVICE> = Vec::new();
    for i in 0..u32::MAX {
        let mut dispDevice = DISPLAY_DEVICE::default();
        match EnumDisplayDevices(
            None,
            i,
            &mut dispDevice,
            co::EDD::EDD_GET_DEVICE_INTERFACE_NAME,
        ) {
            Ok(_) => devices.push(dispDevice),
            Err(_) => {
                break;
            }
        }
    }
    println!("Got {} devices", devices.len());
    println!(
        "{}",
        devices.iter().fold(String::new(), |acc, arg| acc
            + "\n"
            + &*format!(
            "{} - {} - {} - {} - {} ",
            arg.DeviceName(),
            arg.DeviceID(),
            arg.DeviceString(),
            arg.DeviceKey(),
            arg.StateFlags.has(co::STATE_FLAGS::DISPLAY_DEVICE_ACTIVE)
        ))
    );
    devices
}


pub fn rotate_display(displayDeviceIdentifier: Option<&str>, rotationType: RotationType) {
    let mut dm = DEVMODE::default();
    match EnumDisplaySettingsEx(
        displayDeviceIdentifier,
        co::ENUM_SETTINGS::REGISTRY,
        &mut dm,
        Default::default(),
    ) {
        Ok(_) => {
            println!("Display is currently {}", dm.dmDisplayOrientation());
            let fromRotationType = RotationType::from(dm.dmDisplayOrientation());
            match fromRotationType {
                ROT0 | ROT180 => {
                    if rotationType == ROT270 || rotationType == ROT90 {
                        println!("Flip");
                        (dm.dmPelsWidth, dm.dmPelsHeight) = (dm.dmPelsHeight, dm.dmPelsWidth)
                    }
                }
                ROT90 | ROT270 => {
                    if rotationType == ROT0 || rotationType == ROT180 {
                        println!("Flip");
                        (dm.dmPelsWidth, dm.dmPelsHeight) = (dm.dmPelsHeight, dm.dmPelsWidth)
                    }
                }
            }
            dm.set_dmDisplayOrientation(rotationType.clone().into());
            match ChangeDisplaySettingsEx(displayDeviceIdentifier, &mut dm, CDS::DYNAMICALLY) {
                Ok(res) => {
                    println!(
                        "Succeeded: {:?} for {:?}",
                        rotationType,
                        dm.dmDeviceName(),
                    )
                }
                Err(err) => {
                    println!("Could not apply rotation Error {}", err)
                }
            }
        }
        Err(e) => {
            eprintln!("Could not get display details. Error {}", e)
        }
    }
}
