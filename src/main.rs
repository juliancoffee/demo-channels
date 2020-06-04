use std::str::FromStr;
use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Responder};
use crossbeam_channel::{select, unbounded, Receiver, Sender};

mod pairing;

#[derive(Debug)]
pub enum Choice {
    Black,
    White,
}

impl FromStr for Choice {

}

#[derive(Debug)]
pub enum Answer {
    Ok
}

#[get("/api/new_game/{choice}")]
async fn new_game(
    front_events: web::Data<Sender<Choice>>,
    pairing_events: web::Data<Receiver<Answer>>,
) -> impl Responder {
    println!("Send new_game event: {:?}", front_events.send(Choice::White));
    select! {
        recv(pairing_events) -> peer => format!("Found {:?}", peer.unwrap()),
    }
}

#[get("/")]
async fn index() -> impl Responder {
    dbg!("index");
    NamedFile::open("./index.html")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let (sender_events, receiver_events): (Sender<Choice>, Receiver<Choice>) = unbounded();
    let (sender_pairing, receiver_pairing): (Sender<Answer>, Receiver<Answer>) = unbounded();
    let front_events = web::Data::new(sender_events);
    let pairing_events = web::Data::new(receiver_pairing);

    pairing::pairing_loop(receiver_events.clone(), sender_pairing.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(front_events.clone())
            .app_data(pairing_events.clone())
            .service(index)
            .service(fs::Files::new("/static", "./static"))
            .service(new_game)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
