use actix_ws::{self, Message};
use futures_util::StreamExt;
use tokio::sync::{broadcast, mpsc};

use crate::{
    api::contracts::OperationJSONContract,
    ot::operations::{ArcOperation, Operation},
};

use super::contracts::OperationJSONFreeCopy;

pub async fn websocket_writer(
    mut session: actix_ws::Session,
    mut operation_receiver: broadcast::Receiver<ArcOperation>,
) {
    while let Ok(operation) = operation_receiver.recv().await {
        session
            .text(
                serde_json::to_string(&OperationJSONFreeCopy::from_operation(&operation)).unwrap(),
            )
            .await
            .unwrap();
    }
}

pub async fn websocket_reader(
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
    operation_sender: mpsc::Sender<Operation>,
) {
    loop {
        match msg_stream.next().await {
            Some(Ok(msg)) => match msg {
                Message::Ping(bytes) => {
                    session.pong(&bytes).await.unwrap();
                }
                Message::Text(bytes) => {
                    // or we will use binary?
                    let input_operation =
                        serde_json::from_str::<OperationJSONContract>(&bytes).unwrap(); // TODO need error handler
                    let input_operation = input_operation.as_operation().unwrap();
                    operation_sender.send(input_operation).await.unwrap();
                }
                Message::Close(_reason) => break,

                _ => {}
            },
            Some(Err(err)) => {
                panic!("{}", err)
            }

            None => break,
        }
    }
}
