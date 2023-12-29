mod pam;
pub mod statics;

use crate::{utils::get_username, wm::pam::authenticate_password};
use anyhow::Result;
use std::sync::{Arc, Mutex};
use tracing::debug;

#[derive(Debug, Clone)]
pub enum Call {
    SelectFocusedWindow,
    OpenFullscreenWindow(String),
    GrabPointerAndKeyboard,
    UnlockSleep,
    RaiseWindow,
    RedrawScreen,
    FlushCommands,
}

pub trait Broker {
    fn set_pam_return(&mut self, pam: Arc<Mutex<Option<std::sync::mpsc::Receiver<()>>>>);
    fn pam_return(&self) -> Arc<Mutex<Option<std::sync::mpsc::Receiver<()>>>>;
    fn set_pam(&mut self, pam: std::sync::mpsc::Sender<()>);
    fn pam(&self) -> Option<std::sync::mpsc::Sender<()>>;
    fn set_receiver(&mut self, call: Arc<Mutex<Option<std::sync::mpsc::Receiver<Call>>>>);
    fn receiver(&self) -> Arc<Mutex<Option<std::sync::mpsc::Receiver<Call>>>>;
    fn listen(&mut self) -> Result<()>;
}

pub trait Client {
    fn pam_return(&self) -> Arc<Mutex<std::sync::mpsc::Sender<()>>>;
    fn pam(&self) -> Arc<Mutex<std::sync::mpsc::Receiver<()>>>;
    fn call(&self) -> Option<std::sync::mpsc::Sender<Call>>;
    fn clear_password(&mut self);
    fn password(&self) -> String;

    fn loop_until_pam(&mut self, name: &str) -> Result<()> {
        while let Ok(_) = self.pam().lock().unwrap().recv() {
            debug!(
                "PAM authentication attempt: username: '{}', password: '{}'",
                get_username()?,
                self.password()
            );

            match authenticate_password(name) {
                Ok(_) => {
                    debug!("PAM authentication successful");
                    return Ok(());
                }
                Err(e) => {
                    debug!("PAM authentication failed: {}", e);
                    self.clear_password();
                }
            }

            self.pam_return().lock().unwrap().send(()).unwrap();
        }

        Ok(())
    }

    fn select_focused_window(&self) -> Result<()> {
        self.call().unwrap().send(Call::SelectFocusedWindow)?;
        Ok(())
    }

    fn open_fullscreen_window(&self, program_name: String) -> Result<()> {
        self.call()
            .unwrap()
            .send(Call::OpenFullscreenWindow(program_name))?;
        Ok(())
    }

    fn grab_pointer_and_keyboard(&self) -> Result<()> {
        self.call().unwrap().send(Call::GrabPointerAndKeyboard)?;
        Ok(())
    }

    fn unlock_sleep(&self) -> Result<()> {
        self.call().unwrap().send(Call::UnlockSleep)?;
        Ok(())
    }

    fn raise_window(&self) -> Result<()> {
        self.call().unwrap().send(Call::RaiseWindow)?;
        Ok(())
    }

    fn redraw_screen(&self) -> Result<()> {
        self.call().unwrap().send(Call::RedrawScreen)?;
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.call().unwrap().send(Call::FlushCommands)?;
        Ok(())
    }
}
