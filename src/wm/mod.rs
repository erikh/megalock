mod pam;

use crate::{utils::get_username, wm::pam::authenticate_password};
use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};
use tracing::{debug, trace};

lazy_static::lazy_static! {
    pub static ref PASSWORD: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

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

#[derive(Debug, Clone)]
pub enum Event {
    KeyPressed,
    UnlockAttempted,
    UnlockSuccessful,
    UnlockFailure,
}

#[derive(Debug, Clone)]
pub enum PamEvent {
    Authenticated,
    AuthenticationFailed,
}

pub trait Broker {
    fn set_pam_return(&mut self, pam: Arc<Mutex<Option<std::sync::mpsc::Receiver<PamEvent>>>>);
    fn pam_return(&self) -> Arc<Mutex<Option<std::sync::mpsc::Receiver<PamEvent>>>>;
    fn set_pam(&mut self, pam: std::sync::mpsc::Sender<()>);
    fn pam(&self) -> Option<std::sync::mpsc::Sender<()>>;
    fn set_events(&mut self, events: std::sync::mpsc::SyncSender<Event>);
    fn events(&self) -> Option<std::sync::mpsc::SyncSender<Event>>;
    fn set_receiver(&mut self, call: Arc<Mutex<Option<std::sync::mpsc::Receiver<Call>>>>);
    fn receiver(&self) -> Arc<Mutex<Option<std::sync::mpsc::Receiver<Call>>>>;
    fn listen(&mut self) -> Result<()>;
}

pub trait Client {
    fn events(&self) -> Arc<Mutex<std::sync::mpsc::Receiver<Event>>>;
    fn pam_return(&self) -> Arc<Mutex<std::sync::mpsc::Sender<PamEvent>>>;
    fn pam(&self) -> Arc<Mutex<std::sync::mpsc::Receiver<()>>>;
    fn call(&self) -> Option<std::sync::mpsc::Sender<Call>>;
    fn clear_password(&mut self);
    fn password(&self) -> String;

    fn loop_until_pam(&mut self, name: &str) -> Result<()> {
        if let Ok(_) = self
            .pam()
            .lock()
            .unwrap()
            .recv_timeout(std::time::Duration::new(0, 50))
        {
            trace!(
                "PAM authentication attempt: username: '{}', password: '{}'",
                get_username()?,
                self.password()
            );

            match authenticate_password(name) {
                Ok(_) => {
                    debug!("PAM authentication successful");
                    self.pam_return()
                        .lock()
                        .unwrap()
                        .send(PamEvent::Authenticated)
                        .unwrap();
                    return Ok(());
                }
                Err(e) => {
                    debug!("PAM authentication failed: {}", e);
                    self.pam_return()
                        .lock()
                        .unwrap()
                        .send(PamEvent::AuthenticationFailed)
                        .unwrap();
                    self.clear_password();
                    return Err(anyhow!("invalid password"));
                }
            }
        }

        Err(anyhow!("No reception"))
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
