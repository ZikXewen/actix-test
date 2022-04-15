use actix_web::{
    get,
    web::{Data, Json, Path, Query},
    App, Either, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;
use std::{io::Result, sync::Mutex, time::Duration};

struct AppState {
    state: String,
    mut_state: Mutex<u8>,
}
#[derive(Deserialize)]
struct PathInfo {
    u_arg: u32,
    s_arg: String,
}
#[derive(Deserialize)]
struct QueryInfo {
    arg1: String,
    arg2: String,
}

/// Example page.
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

/// Example page with state.
#[get("/state")]
async fn state(data: Data<AppState>) -> impl Responder {
    String::from(&data.state)
}

/// Example page with mutable state.
#[get("/mut_state")]
async fn mut_state(data: Data<AppState>) -> impl Responder {
    let mut counter = data.mut_state.lock().unwrap();
    *counter += 1;
    format!("Counted: {counter} times.")
}

/// Example slow page.
#[get("/slow")]
async fn slow() -> impl Responder {
    // Do not use std::thread::sleep as it will block the thread
    // Try by exceed workers count
    tokio::time::sleep(Duration::from_secs(5)).await;
    "response"
}

/// Example page with different type responses.
#[get("/either")]
async fn either() -> impl Responder {
    if true {
        Either::Left(HttpResponse::BadRequest().body("Bad data"))
    } else {
        Either::Right("Hello")
    }
}

#[get("/path/{u_arg}/{s_arg}")]
async fn path(path: /*Path<(u32, String)>*/ Path<PathInfo>) -> impl Responder {
    // let path = path.into_inner();
    format!("{}, {}", path.u_arg, path.s_arg)
}

#[get("/query")]
async fn query(query: Query<QueryInfo>) -> impl Responder {
    format!("{}, {}", query.arg1, query.arg2)
}

#[get("/json")]
async fn json(json: Json<(String,)>) -> impl Responder {
    json.into_inner().0
}

#[actix_web::main]
async fn main() -> Result<()> {
    // App data are not shared between threads
    // so we should initialize Mutex outside the scope
    let app_data = Data::new(AppState {
        state: "rust-server".to_owned(),
        mut_state: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(hello)
            .service(state)
            .service(mut_state)
            .service(slow)
            // .service(scope("/scope").service(hello))
            .service(slow)
            .service(either)
            .service(path)
            .service(query)
            .service(json)
    })
    .workers(2)
    // .keep_alive(KeepAlive::Os)
    .bind(("127.0.0.1", 8787))?
    .run()
    .await
}
