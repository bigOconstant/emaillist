//! Actix Web Diesel integration example
//!
//! Diesel does not support tokio, so we have to run it in separate threads using the web::block
//! function which offloads blocking code (like Diesel's) in order to not block the server's thread.

#[macro_use]
extern crate diesel;
use actix_web::body::BoxBody;
use actix_web::{web::Data};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use regex::Regex;

use actix_web::{get, error,middleware, web, App, Error, HttpResponse, HttpServer,Result};
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;
use tera::Tera;
mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Finds user by UID.
#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
    context:web::Data<models::AppContext>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();

    let user = web::block(move || {
        let conn = pool.get()?;
        actions::find_user_by_id(user_uid.clone().to_string(), &conn)
    }).await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut email:String = "".to_string();
    let mut first_name:String = "".to_string();
    let mut last_name:String = "".to_string();
    let mut checked:bool = false;
    let mut id = "".to_string();

    if let Some(user) = user {
        email = user.email;
        checked = user.subscribed;
        id = user.id;
        first_name = user.first_name;
        last_name = user.last_name;
        
    }else{
        //Do something here
    }

    let mut ctx = tera::Context::new();
        ctx.insert("email", &email);
        ctx.insert("first_name", &first_name);
        ctx.insert("last_name", &last_name);
        ctx.insert("checked",&checked);
        ctx.insert("UUID",&id);
        ctx.insert("text", "Welcome!");
        ctx.insert("app_name",&context.app_name.clone());
       let s= tmpl.render("user.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;

     Ok(HttpResponse::Ok().content_type("text/html").body(s))

}

async fn join(
    tmpl: web::Data<tera::Tera>,
    context:web::Data<models::AppContext>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("app_name",&context.app_name.clone());
    ctx.insert("error",&false);
    let s = tmpl.render("new_user.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn subscribe_post(
    params: web::Form<models::SubscribeForm>,
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error>{
   let id = params.id.clone();
   let mut checked = false;
   
   match params.subscribed.clone() {
       Some(_x)=> {
           checked = true;
       },
       _x=> {}
   }
    
   let _ = web::block(move || {
        let conn = pool.get()?;
        actions::update_user_subscription_status(checked, &conn,&id)
    }).await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    let s:String = tmpl.render("subbed.html", &tera::Context::new())
    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    
   
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
  
}

async fn add_user(
    params: web::Form<models::NewUser>,
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    context:web::Data<models::AppContext>,
)-> Result<HttpResponse, Error> {

    let mut ctx = tera::Context::new();
    
    ctx.insert("app_name",&context.app_name.clone());
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

    let is_email_valid = email_regex.is_match(&params.email);
    println!("email:{} name:{} {}",params.email,params.firstname,params.lastname);
    let s:String;
    if params.firstname.is_empty() || params.lastname.is_empty() || !is_email_valid {
        ctx.insert("error",&true);

    s = tmpl.render("new_user.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    }
    
    

    else {
        let n = models::NewUser {
            email:params.email.clone(),
            firstname:params.firstname.clone(),
            lastname:params.lastname.clone(),
        };
        let _ = web::block(move || {
            let conn = pool.get()?;
            actions::insert_new_user(&n, &conn)
        }).await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
        
    s = tmpl.render("subbed.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let conn_ip = std::env::var("IPBIND").expect("IPBIND");
    let conn_port = std::env::var("PORT").expect("PORT").parse::<u16>().unwrap();
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    log::info!("starting HTTP server at http://localhost:8080");
    let context = models::AppContext{
        app_name:std::env::var("APP_NAME").expect("APP_NAME"),
    };
    // Start HTTP server
    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(web::Data::new(pool.clone()))
            .app_data("wol")
            .app_data(web::Data::new(tera))
            .app_data(Data::new(context.clone()))
            .wrap(middleware::Logger::default())
            .service(get_user)
            .service(web::resource("/")
                .route(web::get().to(join)))
            .service(web::resource("/join")
                .route(web::post().to(add_user)))
            .service(web::resource("/subscribe").route(web::post().to(subscribe_post)))
            .service(web::scope("").wrap(error_handlers()))
    })
    .bind((conn_ip, conn_port))?
    .run()
    .await
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(e.to_string())
    };

    let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());
    match tera {
        Some(tera) => {
            let mut context = tera::Context::new();
            context.insert("error", error);
            context.insert("status_code", res.status().as_str());
            let body = tera.render("error.html", &context);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}