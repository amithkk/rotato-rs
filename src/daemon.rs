use crate::{win_disp_fns, KeyBind, ROT0, ROT180, ROT270, ROT90};
use hotkey::modifiers::{ALT, CONTROL};
use std::alloc::handle_alloc_error;
use std::thread;
use std::thread::JoinHandle;

pub struct RotatoDaemon {
    pub handle: Option<JoinHandle<()>>,
}

impl RotatoDaemon {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn thread_exec(&mut self) {
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
            hk.register_hotkey(key.get_or_modifiers(), key.key as u32, move || {
                win_disp_fns::rotate_display(None, key.rot_type.clone())
            })
            .unwrap();
        }

        hk.listen();
    }

    pub fn start_or_restart_daemon(&mut self) {
        match self.handle {
            Some(h) => {}
            None => {
                self.handle = Some(thread::spawn(|| self.thread_exec()));
            }
        }
    }
}
