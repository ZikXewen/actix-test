use actix_web::{
    get, post,
    web::{Data, Form, Json, Path, Query},
    App, Either, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;
use std::{io::Result, sync::Mutex, time::Duration};

struct AppState {
    state: String,
    mut_state: Mutex<u8>,
}
#[derive(Deserialize)]
struct ParamsInfo {
    u_arg: u32,
    s_arg: String,
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

/// Example page with path parameters
#[get("/path/{u_arg}/{s_arg}")]
async fn path(path: /*Path<(u32, String)>*/ Path<ParamsInfo>) -> impl Responder {
    // let path = path.into_inner();
    format!("{}, {}", path.u_arg, path.s_arg)
}

/// Example page with query parameters
#[get("/query")]
async fn query(query: Query<ParamsInfo>) -> impl Responder {
    format!("{}, {}", query.u_arg, query.s_arg)
}

/// Example page with a request body.
#[post("/json")]
async fn json(body: Json<ParamsInfo>) -> impl Responder {
    format!("{}, {}", body.u_arg, body.s_arg)
}

/// Example page with a URL-encoded form.
#[post("/form")]
async fn form(form: Form<ParamsInfo>) -> impl Responder {
    format!("{}, {}", form.u_arg, form.s_arg)
}

#[actix_web::main]
async fn main() -> Result<()> {
    // App data are not shared between threads
    // so we should initialize Mutex outside the scope
    // If we need some thread-specific states, however,
    // check https://actix.rs/docs/extractors/
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
            .service(form)
    })
    .workers(2)
    // .keep_alive(KeepAlive::Os)
    .bind(("127.0.0.1", 8787))?
    .run()
    .await
}
