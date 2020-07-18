extern crate urlencoding;
use actix_web::{web, App, HttpServer};
use urlencoding::decode;

use std::sync::Mutex;

use super::player::Player;

async fn load(player: web::Data<Mutex<Player>>, uri: web::Path<String>) -> &'static str {
    let decoded = decode(&uri).unwrap();
    match player.lock() {
        Ok(mut player) => match player.load(&decoded) {
            true => "true",
            _ => "false",
        },
        _ => "false",
    }
}

async fn play(player: web::Data<Mutex<Player>>) -> &'static str {
    match player.lock() {
        Ok(mut player) => match player.empty() {
            false => {
                player.play();
                "true"
            }
            _ => "false",
        },
        _ => "false",
    }
}

async fn pause(player: web::Data<Mutex<Player>>) -> &'static str {
    match player.lock() {
        Ok(mut player) => match player.empty() {
            false => {
                player.pause();
                "true"
            }
            _ => "false",
        },
        _ => "false",
    }
}

async fn stop(player: web::Data<Mutex<Player>>) -> &'static str {
    match player.lock() {
        Ok(mut player) => {
            player.stop();
            "true"
        }
        _ => "false",
    }
}

async fn volume(player: web::Data<Mutex<Player>>) -> String {
    match player.lock() {
        Ok(player) => player.volume().to_string(),
        _ => "".to_string(),
    }
}

async fn set_volume(player: web::Data<Mutex<Player>>, level: web::Path<String>) -> &'static str {
    let level: f32 = level.parse().unwrap_or(100.0);
    let level: f32 = level / 100.0;
    match player.lock() {
        Ok(player) => {
            player.set_volume(level);
            "true"
        }
        _ => "false",
    }
}

async fn is_paused(player: web::Data<Mutex<Player>>) -> &'static str {
    match player.lock() {
        Ok(player) => match player.is_paused() {
            true => "true",
            _ => "false",
        },
        _ => "false",
    }
}

async fn empty(player: web::Data<Mutex<Player>>) -> &'static str {
    match player.lock() {
        Ok(player) => match player.empty() {
            true => "true",
            _ => "false",
        },
        _ => "false",
    }
}

async fn position(player: web::Data<Mutex<Player>>) -> String {
    match player.lock() {
        Ok(player) => player.position().to_string(),
        _ => "".to_string(),
    }
}

async fn state(player: web::Data<Mutex<Player>>) -> String {
    match player.lock() {
        Ok(player) => format!(
            "{{ \"empty\": {}, \"playing\": {}, \"position\": {} }}",
            !player.empty(),
            !player.is_paused(),
            player.position()
        ),
        _ => "".to_string(),
    }
}

#[actix_rt::main]
pub async fn init_server(port: String) -> std::io::Result<()> {
    let player = web::Data::new(Mutex::new(Player::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(player.clone())
            .service(web::resource("/load/{uri}").route(web::get().to(load)))
            .service(web::resource("/play").route(web::get().to(play)))
            .service(web::resource("/pause").route(web::get().to(pause)))
            .service(web::resource("/stop").route(web::get().to(stop)))
            .service(web::resource("/volume").route(web::get().to(volume)))
            .service(web::resource("/set_volume/{level}").route(web::get().to(set_volume)))
            .service(web::resource("/is_paused").route(web::get().to(is_paused)))
            .service(web::resource("/empty").route(web::get().to(empty)))
            .service(web::resource("/position").route(web::get().to(position)))
            .service(web::resource("/state").route(web::get().to(state)))
    })
    .keep_alive(1200)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
