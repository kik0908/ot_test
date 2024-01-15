use std::time::Duration;

use rust_live_server::{
    collaboration::*,
    ot::operations::{InsertOperation, Operation},
};
use tokio::task::JoinHandle;

fn connect_user(
    m: &mut manager::Manager,
    name: String,
) -> (tokio::sync::mpsc::Sender<Operation>, JoinHandle<()>) {
    let (sender, rec) = m.connect("User ".to_string() + &name, "doc1".to_string());

    (
        sender,
        tokio::spawn(async move {
            let mut rec = rec;
            while let Ok(op) = rec.recv().await {
                println!("User {name} receive operation: {:?}", op);
            }
        }),
    )
}

async fn start_test(m: &mut manager::Manager, cur_iteration: usize) {
    // m.connect(subscriber_name, document_id)
    let users = 5;
    let (sender, sender_task) = connect_user(m, (users * cur_iteration).to_string());
    let mut tasks = vec![];
    for i in 1 + cur_iteration * users..=users * (cur_iteration + 1) {
        let (_, task) = connect_user(m, i.to_string());
        tasks.push(task);
    }

    println!("Send operation...");
    for i in 1..5 {
        sender
            .send(Box::new(InsertOperation::new(
                0,
                1,
                format!("text {i}").to_string(),
            )))
            .await
            .unwrap();
    }
    tokio::time::sleep(Duration::from_secs(1)).await;

    sender_task.abort();
    for i in tasks {
        i.abort();
    }

    for i in cur_iteration * users..=users * (cur_iteration + 1) {
        m.disconnect("User ".to_string() + &i.to_string(), "doc1".to_string())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut m = manager::Manager::new();
    start_test(&mut m, 0).await;
    tokio::time::sleep(Duration::from_secs(3)).await;
}
