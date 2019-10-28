use warp::{self, path, Filter, reject};
use serde::{Deserialize, Serialize};
use env_logger::Env;
use base64;
use log;
use ::warpdemo::{ models::*, schema::{posts}, db::{PoolPg, PooledPg}};
use diesel::prelude::*;

#[derive(Serialize, Deserialize)]
struct GreetingResponse {
    name: String
}

#[derive(Serialize, Deserialize)]
struct B64EncodeResponse {
    data: String
}

#[derive(Serialize, Deserialize)]
struct B64EncodeRequest {
    data: String
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String
}

fn main() {
    let env = Env::default()
        .filter_or("WARPSVC_LOG_LEVEL", "info")
        .write_style_or("WARPSVC_LOG_STYLE", "always");            
    env_logger::init_from_env(env);
    log::info!("Starting warp-demo-services");

    let pool = ::warpdemo::db::pg_pool();
    // setup the the connection pool to get a connection on each request
    let pg = warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PoolPg| match pool.get() {
            Ok(conn) => Ok(conn),
            Err(_) => Err(reject::server_error()),
        });

    let hello = path!("greetings" / String)
        .map(|name| warp::reply::json(&GreetingResponse{name}));

    let posts = warp::path("posts");
    let posts_index = posts.and(warp::path::end());
    let posts_list = warp::get2()
        .and(posts_index)
        .and(pg)
        .map(move |db: PooledPg|{
            
            let posts = posts::table
                .filter(posts::published.eq(true))
                .limit(5)
                .load::<Post>(&db)
                .unwrap();

            warp::reply::json(&posts)
        });


    let b64 = path!("base64" / "encode")
        .and(warp::post2())
        .and(warp::body::json())
        .map(|data :B64EncodeRequest| {
            warp::reply::json(&B64EncodeResponse{data: base64::encode(&data.data)})
        });

    warp::serve(hello
                .or(b64)
                .or(posts_list)
                .with(warp::log("warp-demo"))
            )
        .run(([127,0,0,1], 8080));
}