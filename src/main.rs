#![feature(destructuring_assignment)]

use std::fmt::{Display, Formatter};
use std::ops::BitOr;
use std::thread;

use hotkey;
use hotkey::modifiers::{ALT, CONTROL};
use winsafe::{ChangeDisplaySettings, ChangeDisplaySettingsEx, co, DEVMODE, DISPLAY_DEVICE, EnumDisplayDevices, EnumDisplaySettingsEx, ErrResult, HWND, WinResult};
use winsafe::co::{CDS, DISP_CHANGE, DMDO};
use winsafe::prelude::{Handle, NativeBitflag, UserHwnd};

use enums::RotationType;

use crate::RotationType::{ROT0, ROT180, ROT270, ROT90};
use crate::window::MyWindow;

mod enums;
mod win_disp_fns;
mod window;
mod id;
mod daemon;

#[derive(Clone)]
struct KeyBind {
    modifiers: Vec<u32>,
    pub key: char,
    pub rot_type: RotationType,
}

impl KeyBind {
    pub fn get_or_modifiers(&self) -> u32 {
        if self.modifiers.len() == 0 {
            panic!("No modifiers in keybind, unsupported");
        }
        self.modifiers.iter().fold(0, |cur, next| cur | next) as u32
    }
}

fn run_app() -> ErrResult<i32> {
    MyWindow::new()?.run() // create our main window and run it
}

fn spawnUI() -> () {
    if let Err(e) = run_app() {
        HWND::NULL.MessageBox(
            &e.to_string(), "Uncaught error", co::MB::ICONERROR).unwrap();
    }
}

fn main() {
    let disps = win_disp_fns::enumerate_all_displays();
    let handle = thread::spawn(|| {
        spawnUI()
    });
    let keybinds = vec![
        KeyBind {
            modifiers: vec![CONTROL, ALT],
            key: 'S',
            rot_type: ROT180,
        },
        KeyBind {
            modifiers: vec![CONTROL, ALT],
            key: 'W',
            rot_type: ROT0,
        },
        KeyBind {
            modifiers: vec![CONTROL, ALT],
            key: 'A',
            rot_type: ROT90,
        },
        KeyBind {
            modifiers: vec![CONTROL, ALT],
            key: 'D',
            rot_type: ROT270,
        },
    ];

    let mut hk = hotkey::Listener::new();

    for key in keybinds {
        hk.register_hotkey(
            key.get_or_modifiers(),
            key.key as u32,
            move || win_disp_fns::rotate_display(None, key.rot_type.clone()),
        )
            .unwrap();
    }

    hk.listen();
}
