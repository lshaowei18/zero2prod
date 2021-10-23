use actix_web::{App, HttpRequest, HttpServer, Responder, web};

/* 
Takes an HttpRequest as input and returns something that implements the Responder trait.
A type implements the Responder trait if it can be converted into a HttpReponse.
It is implemented off the shelf for a variety of common types.
We can rollour own implmeentations if needed.
*/
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

// Derive attributes attaches itself to an item and allow manipulation
#[actix_web::main]
async fn main() -> std::io::Result<()>   {
    // || is a zero-argument closure
    HttpServer::new( || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
