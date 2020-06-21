//use crate::config::config_center::ConfigCenter;
use crate::hawk_api::model::redis::CacheInfo;
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use bytes::{Bytes, BytesMut};
use futures::future::join_all;
use futures::StreamExt;
use json::JsonValue;
use redis_async::resp::RespValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use hawk_api::model::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
    pub name: String,
    pub number: i32,
}

/// This handler uses json extractor
async fn index(item: web::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

/// This handler uses json extractor
/// http://localhost:8099/test
async fn get_index(req: HttpRequest, item: web::Path<String>) -> HttpResponse {
    //get param from url
    let a = req.query_string();
    println!("query: {:?}", &a);
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.into_inner()) // <- send response
}

/// This handler uses json extractor with limit and header
async fn extract_item(item: web::Json<MyObj>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    println!("model: {:?}", item);
    let head = req.head();
    let a = head.headers.get("mytest");
    println!("{:?}", a);
    HttpResponse::Ok().json(item.0) // <- send json response
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// This handler manually load request payload and parse json object
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<MyObj>(&body)?;
    let v: Value = serde_json::from_slice(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

/// This handler manually load request payload and parse json-rust test
async fn index_mjsonrust(body: Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(injson.dump()))
}

///use redis cache
///
async fn cache_stuff(
    info: web::Json<CacheInfo>,
    redis: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    let info = info.into_inner();

    let one = redis.send(Command(resp_array!["SET", "mydomain:one", info.one]));
    let two = redis.send(Command(resp_array!["SET", "mydomain:two", info.two]));
    let three = redis.send(Command(resp_array!["SET", "mydomain:three", info.three]));

    // Creates a future which represents a collection of the results of the futures
    // given. The returned future will drive execution for all of its underlying futures,
    // collecting the results into a destination `Vec<RespValue>` in the same order as they
    // were provided. If any future returns an error then all other futures will be
    // canceled and an error will be returned immediately. If all futures complete
    // successfully, however, then the returned future will succeed with a `Vec` of
    // all the successful results.
    let res: Vec<Result<RespValue, Error>> = join_all(vec![one, two, three].into_iter())
        .await
        .into_iter()
        .map(|item| {
            item.map_err(Error::from)
                .and_then(|res| res.map_err(Error::from))
        })
        .collect();

    // successful operations return "OK", so confirm that all returned as so
    if !res.iter().all(|res| match res {
        Ok(RespValue::SimpleString(x)) if x == "OK" => true,
        _ => false,
    }) {
        Ok(HttpResponse::InternalServerError().finish())
    } else {
        Ok(HttpResponse::Ok().body("successfully cached values"))
    }
}

/// use redis del
async fn del_stuff(redis: web::Data<Addr<RedisActor>>) -> Result<HttpResponse, Error> {
    let res = redis
        .send(Command(resp_array![
            "DEL",
            "mydomain:one",
            "mydomain:two",
            "mydomain:three"
        ]))
        .await?;

    match res {
        Ok(RespValue::Integer(x)) if x == 3 => {
            Ok(HttpResponse::Ok().body("successfully deleted values"))
        }
        _ => {
            println!("---->{:?}", res);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[cfg(windows)]
pub async fn main() -> std::io::Result<()> {
    Ok(())
}

///
/// http main with actix_web
///
//#[cfg(unix)]
#[actix_rt::main]
#[cfg(unix)]
pub async fn main() -> std::io::Result<()> {
    let config_option = hawk_config::read_config();

    let uds_path = get_unix_sock_path(config_option);

    HttpServer::new(|| {
        // use redis config
        let redis_addr = RedisActor::start("192.168.101.33:6379");
        App::new()
            .data(redis_addr)
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/extractor").route(web::post().to(index)))
            .service(
                web::resource("/extractor2")
                    .data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(web::post().to(extract_item)),
            )
            .service(web::resource("/manual").route(web::post().to(index_manual)))
            .service(web::resource("/mjsonrust").route(web::post().to(index_mjsonrust)))
            .service(
                web::resource("/stuff")
                    .route(web::post().to(cache_stuff))
                    .route(web::delete().to(del_stuff)),
            )
            .service(
                web::resource("/{name}")
                    .route(web::get().to(get_index)),
            )
            .service(
                web::resource("/")
                    .route(web::post().to(index))
                    .route(web::get().to(get_index)),
            )
    })
        .bind_uds(uds_path)?
        .run()
        .await
}

    fn get_unix_sock_path(config_option: Option<Config>) ->String{
    if config_option.is_some() {
        if let Some(config) = config_option {
            if let Some(server) = config.server {
                return server.uds;
            }
        }
        return "/home/ithawk/server.uds".to_string();
    } else {
        "/home/ithawk/server.uds".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let mut app =
            test::init_service(App::new().service(web::resource("/").route(web::post().to(index))))
                .await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&MyObj {
                name: "my-name".to_owned(),
                number: 43,
            })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"{"name":"my-name","number":43}"##);

        Ok(())
    }
}
