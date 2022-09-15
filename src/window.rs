use winsafe::{prelude::*, gui, ErrResult};

use crate::id;

#[derive(Clone)]
pub struct MyWindow {
    wnd:     gui::WindowMain,
    btn_0 : gui::Button,
    hot_0 : gui::Edit,
}

impl MyWindow {
    pub fn new() -> ErrResult<MyWindow> {
        let dont_move = (gui::Horz::None, gui::Vert::None);

        let wnd = gui::WindowMain::new_dlg(id::DLG_MAIN, Some(id::ICO_MAIN), None);
        let btn_0  = gui::Button::new_dlg(&wnd, id::BTN_0, dont_move);
        let hot_0  = gui::Edit::new_dlg(&wnd, id::HOT_0, dont_move);

        let new_self = Self { wnd, btn_0, hot_0 };
        new_self.events();
        Ok(new_self)
    }

    pub fn run(&self) -> ErrResult<i32> {
        self.wnd.run_main(None)
    }

    fn events(&self) {

        self.btn_0.on().bn_clicked({
            let self2 = self.clone();

            move || {
                //self2.hot_0.hwnd().SendMessage();
                let input_text = self2.hot_0.text()?;
                println!("Input: {}", input_text);
                Ok(())
            }
        })
        // self.btn_show.on().bn_clicked({
        //     let self2 = self.clone();
        //     move || {
        //         let input_text = self2.txt_input.text()?;
        //
        //         let my_modal = MyModal::new(&self2.wnd, &input_text);
        //         let returned_text = my_modal.show()?;
        //
        //         if let Some(text) = &returned_text {
        //             // If user clicked OK on the modal, a text is returned,
        //             // so we replace our current text with the new one.
        //             self2.txt_input.set_text(text)?;
        //         }
        //         Ok(())
        //     }
        // });
    }
}