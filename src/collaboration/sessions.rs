use std::sync::{Arc, Mutex};

use tokio::{
    sync::{broadcast, mpsc},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use crate::ot::{
    document::{self, DocumentTrait},
    operations::{ArcOperation, Operation},
};

pub struct Session {
    document: Arc<Mutex<document::DocumentMem>>,
    input_sender: mpsc::Sender<Operation>,
    input_receiver: Mutex<Option<mpsc::Receiver<Operation>>>,
    //if we want to interact with each client separately
    // it is better to take mpsc channels in hashmap
    output_sender: broadcast::Sender<ArcOperation>,
    subscribers: Mutex<usize>,
    listner_cancelled_token: Option<CancellationToken>,
}

impl Session {
    pub fn new() -> Self {
        let (mpsc_sender, mpsc_receiver) = mpsc::channel(128);
        let (broadcast_sender, _) = broadcast::channel(64);
        Session {
            document: Arc::new(Mutex::new(document::DocumentMem::new())),
            input_sender: mpsc_sender,
            input_receiver: Mutex::new(Some(mpsc_receiver)),
            output_sender: broadcast_sender,
            subscribers: Mutex::new(0),
            listner_cancelled_token: None,
        }
    }
    pub fn subscribe(&mut self) -> broadcast::Receiver<ArcOperation> {
        *self.subscribers.lock().unwrap() += 1;
        self.output_sender.subscribe()
    }
    pub fn subscribers(&self) -> usize {
        self.subscribers.lock().unwrap().clone()
    }

    pub fn listner_work(&self) -> bool {
        if let Some(handler) = &self.listner_cancelled_token {
            !handler.is_cancelled()
        } else {
            false
        }
    }

    pub fn unsubscribe(&mut self) -> usize {
        let mut subscribers = self.subscribers.lock().unwrap();
        *subscribers -= 1;
        if *subscribers == 0 {
            self.listner_cancelled_token.take().unwrap().cancel(); // TODO panics if unsubscribe was called after lisner was turned off and not turned on again
        }
        *subscribers
    }

    pub fn get_input(&self) -> mpsc::Sender<Operation> {
        self.input_sender.clone()
    }
    pub fn start_listen(session: Arc<Mutex<Self>>) -> JoinHandle<()> {
        let cancelled_token = CancellationToken::new();
        {
            session.lock().unwrap().listner_cancelled_token = Some(cancelled_token);
        };
        tokio::spawn(Self::listen(session))
    }

    async fn listen(session: Arc<Mutex<Self>>) {
        let cancelled_token;
        let (mut rec, document) = {
            let session = session.lock().unwrap();
            cancelled_token = session.listner_cancelled_token.clone().unwrap();

            let x = (
                session.input_receiver.lock().unwrap().take().unwrap(),
                Arc::clone(&session.document),
            );
            x
        };

        loop {
            tokio::select! {
                biased;

                _ = cancelled_token.cancelled() => {
                    let session = session.lock().unwrap();
                    *session.input_receiver.lock().unwrap() = Some(rec);
                    return;
                },
                operation = rec.recv() => {
                    if let Some(operation) = operation {
                        let ans;
                        {
                            let mut document = document.lock().unwrap(); // TODO I think we can remove mutex here if provide internal mutability in Document
                            // But I don't know if we need it
                            ans = document.apply(operation);
                        };
                        session.lock().unwrap().output_sender.send(ans).unwrap();
                    }
                }
            }
        }
    }
}
