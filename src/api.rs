use actix_web::{dev::Server, get, rt, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use crate::collaboration::manager::Manager;

use self::contracts::WsConnectionQuery;

mod contracts;
mod handlers;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/ws")]
async fn echo_ws(
    req: HttpRequest,
    stream: web::Payload,
    query: web::Query<WsConnectionQuery>, // more detailed analysis in function body maybe needed
    session_manager: web::Data<Manager>,
) -> impl Responder {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream).unwrap();
    let q = query.0;
    let (client_name, document_id) = (q.client, q.document);

    let (cur_client_name, cur_document_id) = (client_name.clone(), document_id.clone());
    let cur_session_manager = session_manager.clone();
    let (sender, receiver) = tokio::task::spawn_blocking(move || {
        cur_session_manager
            .as_mut()
            .connect(cur_client_name, cur_document_id)
    })
    .await
    .unwrap();

    {
        let session = session.clone();
        rt::spawn(async move {
            rt::spawn(handlers::websocket_reader(session, msg_stream, sender))
                .await
                .unwrap();
        });
        session_manager
            .as_mut()
            .disconnect(client_name, document_id)
    }
    rt::spawn(handlers::websocket_writer(session, receiver)); // TODO maybe rt::spawn => tokio::spawn?

    res
}

pub fn get_server_future() -> Server {
    let manager = web::Data::new(Manager::new());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&manager))
            .service(hello)
            .service(echo_ws)
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
}
