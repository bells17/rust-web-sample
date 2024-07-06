use actix_web::{get, web, App, HttpResponse, HttpServer};
use deadpool_postgres::Pool;
use redis::{Commands, Client};

mod postgres;
mod user;

#[get("/")]
async fn index() -> HttpResponse {
    log::info!("root endpoint was called");
    HttpResponse::Ok().body("Hello world!!!")
}

#[get("/users")]
async fn list_users(pool: web::Data<Pool>) -> HttpResponse {
    log::info!("list_users was called");
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::info!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match user::User::all(&**client).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => {
            log::info!("unable to fetch users: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch users");
        }
    }
}

#[get("/inc")]
async fn inc(_pool: web::Data<Pool>, redis_client: web::Data<Client>) -> HttpResponse {
    log::info!("inc was called");
    let mut conn = match redis_client.get_connection() {
        Ok(conn) => conn,
        Err(err) => {
            log::error!("unable to get redis connection: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get redis connection");
        }
    };

    let key = "access_count";
    let count: i64 = match conn.incr(key, 1) {
        Ok(count) => count,
        Err(err) => {
            log::error!("unable to increment key {}: {:?}", key, err);
            return HttpResponse::InternalServerError().json("unable to increment redis key");
        }
    };
    HttpResponse::Ok().body(format!("Redis count: {}", count))
}

fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pg_pool = postgres::create_pool();
    postgres::migrate_up(&pg_pool).await;

    let redis_client = redis_client().unwrap();

    let address = address();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_pool.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .service(index)
            .service(list_users)
            .service(inc)
    })
    .bind(&address)?
    .run()
    .await
}

fn redis_client() -> redis::RedisResult<Client> {
    let cnf: String = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".into());
    redis::Client::open(cnf)
}
