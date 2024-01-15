use super::sessions::Session;
use crate::ot::operations::{ArcOperation, Operation};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::{broadcast::Receiver, mpsc::Sender};

pub struct Manager {
    sessions: Mutex<HashMap<String, Arc<Mutex<Session>>>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    pub fn as_mut(&self) -> &mut Self {
        unsafe { &mut *(self as *const Manager as *mut Manager) }
    }

    pub fn connect(
        &mut self,
        _subscriber_name: String,
        document_id: String,
    ) -> (Sender<Operation>, Receiver<ArcOperation>) {
        let session = {
            Arc::clone(
                self.sessions
                    .lock()
                    .unwrap()
                    .entry(document_id)
                    .or_insert_with(|| {
                        let session = Arc::new(Mutex::new(Session::new()));

                        session
                    }),
            )
        };

        let need_listner_start: bool;

        let ans = {
            let mut session = session.lock().unwrap();
            let input = session.get_input();
            let output = session.subscribe();

            need_listner_start = !session.listner_work();

            (input, output)
        };

        if need_listner_start {
            Session::start_listen(session);
        }

        ans
    }

    pub fn disconnect(&mut self, _subscriber_name: String, document_id: String) {
        let session = {
            self.sessions
                .lock()
                .unwrap()
                .get(&document_id)
                .and_then(|session| Some(Arc::clone(session)))
        };
        if let Some(session) = session {
            let _connections = session.lock().unwrap().unsubscribe();
            // TODO? if connections == 0 then delete session from sessions
        }
    }
}

unsafe impl Sync for Manager {}
unsafe impl Send for Manager {}
