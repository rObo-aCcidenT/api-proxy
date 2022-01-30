use dotenv::dotenv;
use std::env;
use reqwest;
use serde::{Serialize,Deserialize};
// use actix_web::{http,web,App,HttpServer,Result};
use actix_web::{web,App,HttpServer,Result};
use actix_cors::Cors;

#[derive(Serialize,Deserialize,Debug)]
struct Games{
    id:i32,
    name:String,
    summary:String,
}
impl Games{
    fn new(id:i32,name:String,summary:String)->Self{
        Games{id,name,summary}
    }
}
#[derive(Serialize,Debug)]
struct GameVec{
    games:Vec<Games>
}
impl GameVec{
    fn new(g_vec:Vec<Games>)->Self{
        GameVec{games:g_vec}
    }
}
#[derive(Debug)]
enum ApiErr{
    EnvParseErr(std::env::VarError),
    ReqSendErr(reqwest::Error),
}
impl From<std::env::VarError> for ApiErr{
    fn from(err:std::env::VarError)->Self{
        ApiErr::EnvParseErr(err)
    }
}
impl From<reqwest::Error> for ApiErr{
    fn from(err:reqwest::Error)->Self{
        ApiErr::ReqSendErr(err)
    }
}

#[derive(Deserialize)]
struct Game{
	gname:String,
}

#[tokio::main]
async fn api_call(game:&str)->Result<GameVec,ApiErr>{      
    let url=env::var("URL")?;
    let client_id=env::var("Client_ID")?;
    let authorization=env::var("Authorization")?;
    let content_type=env::var("Content_Type")?;
    let query=format!("search \"{}\"; fields name,summary;where version_parent = null;limit 3;",game);
    let client = reqwest::Client::new();
    let games=client.post(url)
    .header("Authorization",authorization)
    .header("Content-Type",content_type)
    .header("Client-ID",client_id)
    .body(query)
    .send()
    .await?
    .json::<Vec<Games>>()
    .await?;
    Ok(GameVec::new(games))
}

async fn game_info(info:web::Json<Game>)->Result<web::Json::<GameVec>>{
    let game_res=api_call(&info.gname);
    match game_res{
        Ok(games)=> Ok(web::Json(games)),
        Err(err)=>{
            println!("{:?}",err);
            Ok(web::Json(GameVec::new(vec![Games::new(0,"".into(),"".into())])))}
    }
}

#[actix_web::main]
async fn main() ->std::io::Result<()>{
    dotenv().ok(); 
    let port=env::var("PORT").unwrap();
    let host=env::var("HOST").unwrap();
	let ip_port=format!("{}:{}",host,port);
	println!("server running on : {}",ip_port);
	HttpServer::new(| | { 
                    let cors = Cors::permissive();
                                //  .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                                //  .allowed_header(http::header::CONTENT_TYPE).allow_any_origin();         
                    App::new()
                        .wrap(cors)
						.route("/",web::post().to(game_info)) 
					})
					.bind(ip_port)?
					.run()
					.await
	
}