pub mod connection;
pub mod consts;
pub mod statics;

use crate::{
    clear_password,
    wm::{statics::PASSWORD, Broker, Call, Event, PamEvent},
};
use anyhow::Result;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

lazy_static::lazy_static! {
    static ref CLIENT: Arc<Mutex<HashMap<i32, Arc<Mutex<Client>>>>> = Arc::new(Mutex::new(HashMap::default()));
}

pub struct Client {
    screen_number: i32,
    sender: Arc<Mutex<Option<std::sync::mpsc::Sender<Call>>>>,
    pam: Arc<Mutex<std::sync::mpsc::Receiver<()>>>,
    pam_return: Arc<Mutex<std::sync::mpsc::Sender<PamEvent>>>,
    events: Arc<Mutex<std::sync::mpsc::Receiver<Event>>>,
}

impl Client {
    fn new(screen_number: i32) -> Result<Arc<Mutex<Self>>> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let (pam_s, pam_r) = std::sync::mpsc::channel();
        let (pam_return_s, pam_return_r) = std::sync::mpsc::channel();
        let (events_s, events_r) = std::sync::mpsc::sync_channel(100);
        let mut s = Self {
            screen_number,
            sender: Arc::new(Mutex::new(Some(sender))),
            pam: Arc::new(Mutex::new(pam_r)),
            pam_return: Arc::new(Mutex::new(pam_return_s)),
            events: Arc::new(Mutex::new(events_r)),
        };
        s.spawn(receiver, pam_s, pam_return_r, events_s);
        Ok(Arc::new(Mutex::new(s)))
    }

    pub fn get_client(screen_number: i32) -> Result<Arc<Mutex<Self>>> {
        let mut lock = CLIENT.lock().unwrap();

        if let Some(obj) = lock.get(&screen_number) {
            Ok(obj.clone())
        } else {
            let obj = Self::new(screen_number)?;
            lock.insert(screen_number, obj.clone());
            Ok(obj)
        }
    }

    fn spawn(
        &mut self,
        receiver: std::sync::mpsc::Receiver<Call>,
        pam: std::sync::mpsc::Sender<()>,
        pam_return: std::sync::mpsc::Receiver<PamEvent>,
        events: std::sync::mpsc::SyncSender<Event>,
    ) {
        let screen_number = self.screen_number;
        std::thread::spawn(move || {
            let mut connection =
                connection::Connection::init(screen_number).expect("Could not init X11");
            connection.set_receiver(Arc::new(Mutex::new(Some(receiver))));
            connection.set_pam(pam);
            connection.set_pam_return(Arc::new(Mutex::new(Some(pam_return))));
            connection.set_events(events);
            connection.listen().expect("Could not listen to events");
        });
    }
}

impl crate::wm::Client for Client {
    fn events(&self) -> Arc<Mutex<std::sync::mpsc::Receiver<Event>>> {
        self.events.clone()
    }

    fn pam_return(&self) -> Arc<Mutex<std::sync::mpsc::Sender<PamEvent>>> {
        self.pam_return.clone()
    }

    fn pam(&self) -> Arc<Mutex<std::sync::mpsc::Receiver<()>>> {
        self.pam.clone()
    }

    fn clear_password(&mut self) {
        clear_password!()
    }

    fn password(&self) -> String {
        PASSWORD.lock().unwrap().to_string()
    }

    fn call(&self) -> Option<std::sync::mpsc::Sender<Call>> {
        if let Some(sender) = self.sender.lock().unwrap().clone() {
            Some(sender)
        } else {
            None
        }
    }
}
