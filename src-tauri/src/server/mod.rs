use std::fs::File;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::{
    conf::{self, model::AppConfig},
    env,
    proxy::set_proxy,
    utils::tail_from_file,
    v2fly::get_v2ray_instance,
};

#[get("/env/is-dev")]
async fn hello() -> impl Responder {
    env::is_dev().to_string()
}

#[post("/config/save")]
async fn save_config(req_body: web::Json<AppConfig>) -> impl Responder {
    conf::save(&req_body);

    set_proxy(&req_body);
    HttpResponse::Ok()
}

#[get("/config")]
async fn get_config() -> impl Responder {
    let config = conf::read();

    HttpResponse::Ok().json(config)
}

#[get("/v2ray/start")]
async fn start_v2ray() -> impl Responder {
    let v2ray = get_v2ray_instance();

    let app_conf = conf::read();

    if let Some(err) = v2ray.start(&app_conf).err() {
        return err.to_string();
    }

    "".to_owned()
}

#[get("/v2ray/stop")]
async fn stop_v2ray() -> impl Responder {
    let v2ray = get_v2ray_instance();

    v2ray.stop();
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct LogPathQuery {
    p: String,
}

#[get("/v2ray/log")]
async fn read_v2ray_log(query: web::Query<LogPathQuery>) -> impl Responder {
    let f = File::open(query.p.clone());

    let result: Vec<String> = Vec::new();

    if f.is_err() {
        return HttpResponse::Ok().json(result);
    }

    let logs = match f {
        Ok(x) => tail_from_file(&x, 1000),
        Err(_) => result,
    };

    HttpResponse::Ok().json(logs)
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(save_config)
            .service(get_config)
            .service(start_v2ray)
            .service(stop_v2ray)
            .service(read_v2ray_log)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
