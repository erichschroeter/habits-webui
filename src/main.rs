use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/api/v1/all")]
async fn api(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let csv_path = "Loop Habits CSV 2022-12-26/Habits.csv".to_string();
    let csv_data = std::fs::read_to_string(csv_path)
        .expect("Failed to read csv file");
    let mut rdr = csv::Reader::from_reader(csv_data.as_bytes());
    let headers = rdr.headers()
        .expect("No headers in csv file");
    println!("{:?}", headers);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
