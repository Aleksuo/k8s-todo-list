use axum::{
    Json, Router,
    body::Bytes,
    extract::{self, State},
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use entities::todo;
use migration::{Migrator, MigratorTrait};
use reqwest::{
    Client, StatusCode,
    header::{self, CONTENT_TYPE},
};
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{
    ActiveValue::{NotSet, Set},
    ConnectOptions, Database, DatabaseConnection,
};
use serde::{Deserialize, Serialize};
use std::env;
pub use std::{
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::OnceCell;
use uuid::Uuid;

#[derive(Clone, Serialize)]
struct Todo {
    id: Uuid,
    value: String,
}

#[derive(Deserialize)]
struct CreateTodo {
    value: String,
}

#[derive(Clone)]
struct Config {
    timestamp_path: String,
    image_path: String,
    server_port: String,
}

fn read_config() -> Config {
    let image_path = env::var("IMAGE_PATH").unwrap_or("pic.jpeg".to_string());
    let timestamp_path = env::var("TIMESTAMP_PATH").unwrap_or("timestamp.txt".to_string());
    let server_port = env::var("SERVER_PORT").unwrap_or(8080.to_string());
    Config {
        timestamp_path,
        image_path,
        server_port,
    }
}

#[derive(Clone)]
struct AppState {
    http: Client,
    config: Config,
}
const CACHE_TIME: i64 = 3600000;

static DB_CLIENT: OnceCell<DatabaseConnection> = OnceCell::const_new();

#[tokio::main]
pub async fn main() {
    let http = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    let config = read_config();
    let server_port = config.server_port.clone();
    let server_address = format!("0.0.0.0:{}", server_port);

    DB_CLIENT
        .get_or_init(|| async {
            let database_url =
                "postgres://projectuser:projectuserpw@db-svc:5432/projectdb?currentSchema=public";
            let opt = ConnectOptions::new(database_url);
            Database::connect(opt).await.unwrap()
        })
        .await;
    Migrator::up(DB_CLIENT.get().unwrap(), None).await;
    let app_state = AppState { http, config };
    let routes = Router::new()
        .route("/hello-world", get(hello_world_handler))
        .route("/pic", get(get_pic_handler))
        .route("/todos", get(get_todos_handler))
        .route("/todos", post(create_todo_handler));

    let app = Router::new().nest("/api", routes).with_state(app_state);
    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();
    println!("Starting server at port {}", server_port);
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world_handler() -> String {
    return "Hello World!".to_string();
}

async fn get_pic_handler(State(state): State<AppState>) -> impl IntoResponse {
    let timestamp = tokio::fs::read_to_string(&state.config.timestamp_path).await;
    let timestamp = match timestamp {
        Ok(timestamp) => timestamp,
        Err(_) => {
            // File does not exist, initialize the timestamp.txt
            let start_time = UNIX_EPOCH;
            let start_dt: DateTime<Utc> = start_time.into();
            let start_str = start_dt.to_string();
            tokio::fs::write(&state.config.timestamp_path, &start_str)
                .await
                .unwrap();
            start_str
        }
    };

    let fetch_new_pic = {
        let cur_time_dt: DateTime<Utc> = SystemTime::now().into();
        let timestamp_dt: DateTime<Utc> = DateTime::from_str(&timestamp).unwrap();
        (cur_time_dt.timestamp_millis() - timestamp_dt.timestamp_millis()) > CACHE_TIME
    };

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, header::HeaderValue::from_static("image/jpeg"));

    if fetch_new_pic {
        let new_pic = get_new_pic_and_update_files(&state).await;
        return (StatusCode::OK, headers, new_pic);
    }
    // Fetch old pic from disk instead

    let img = tokio::fs::read(&state.config.image_path).await;
    let img = match img {
        Ok(img) => Bytes::from(img),
        Err(_) => get_new_pic_and_update_files(&state).await,
    };
    return (StatusCode::OK, headers, img);
}

async fn get_new_pic_and_update_files(state: &AppState) -> Bytes {
    let new_pic = get_new_pic(&state).await;
    save_pic_to_file(&state.config.image_path, &new_pic).await;
    save_current_time_to_file(&state.config.timestamp_path).await;
    new_pic
}

async fn get_new_pic(state: &AppState) -> Bytes {
    state
        .http
        .get("https://picsum.photos/200")
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .expect("unable to fetch a pic")
        .bytes()
        .await
        .unwrap()
}

async fn save_pic_to_file(fpath: &String, img: &Bytes) {
    tokio::fs::write(fpath, img).await.unwrap();
}

async fn save_current_time_to_file(fpath: &String) {
    let cur_time = SystemTime::now();
    let cur_time_dt: DateTime<Utc> = cur_time.into();
    tokio::fs::write(fpath, cur_time_dt.to_string())
        .await
        .unwrap();
}

async fn get_todos_handler(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let all_todos: Vec<Todo> = todo::Entity::find()
        .all(DB_CLIENT.get().unwrap())
        .await
        .unwrap()
        .iter()
        .map(|todo_model| Todo {
            id: todo_model.id,
            value: todo_model.value.clone(),
        })
        .collect();
    Json(all_todos)
}

async fn create_todo_handler(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<CreateTodo>,
) -> Json<Todo> {
    let new_todo = todo::ActiveModel {
        id: Set(Uuid::new_v4()),
        value: Set(payload.value.to_owned()),
    };
    let saved_todo: todo::Model = new_todo.insert(DB_CLIENT.get().unwrap()).await.unwrap();
    let todo_dto = Todo {
        id: saved_todo.id,
        value: saved_todo.value,
    };
    Json(todo_dto)
}
