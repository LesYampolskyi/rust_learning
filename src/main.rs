use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/custom")]
async fn get_data() -> impl Responder {
    HttpResponse::Ok().body("And this is custom")
}

#[get("/custom")]
async fn custom_request() -> impl Responder {
    HttpResponse::Ok().body("Hello, my friend!")
}

#[post("/echo")] // POST _ NOT GET
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn another_request() -> impl Responder {
    HttpResponse::Ok().body("123- hello --- ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_data)
            .service(custom_request)
            .route("/hey", web::get().to(manual_hello))
            // .route("/hi", web::get().to(custom_request))
            .route("/hi", web::get().to(another_request))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
