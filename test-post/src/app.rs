
use std::time::Duration;

use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer, Responder};

const AGGREGATE_INTERVAL: Duration = Duration::from_secs(30);

#[derive(Clone)]
pub struct TestApp{
    test_value: u32,
}

impl TestApp {
    pub fn new() -> Self {
        TestApp {
            test_value: 0,
        }
    
    }

    pub async fn start(&mut self) {
        println!("start");
        loop {
            // self.aggregate_proofs();

            tokio::time::sleep(AGGREGATE_INTERVAL).await;
        }
    }
}


async fn testing(
    service: web::Data<TestApp>,
) -> impl Responder {
    println!("testing");
    // let test_value = service.test_value.lock().unwrap();
    // test_value += 1;
    // println!("app.test_value: {}", app_state.test_value);
    HttpResponse::Ok().json(service.test_value+1)

}

async fn testing_post(
    service: web::Data<TestApp>,
    params: web::Json<u32>,
) -> impl Responder {
    println!("testing_post");
    println!("params: {}", params);
    // let test_value = service.test_value.lock().unwrap();
    // test_value += 1;
    // println!("app.test_value: {}", app_state.test_value);
    HttpResponse::Ok().json(service.test_value)

}

pub async fn start_rpc_server(shared_service: TestApp) -> impl Send {
    println!("start_rpc_server");

    let json_cfg = web::JsonConfig::default().limit(180000000000);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_service.clone()))
            .app_data(json_cfg.clone())
            .route("/testing_get", web::get().to(testing))
            .route("/testing_post", web::post().to(testing_post))
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await;
}