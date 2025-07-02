use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result, params};
use actix_cors::Cors;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: Option<i32>,
    title: String,
    completed: bool,
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;
    println!("Table 'todos' checked/created.");
    Ok(())
}

async fn get_todos() -> impl Responder {
    let conn = Connection::open("todo.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, title, completed FROM todos").unwrap();
    let todos_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
        })
    }).unwrap();

    let todos: Vec<Todo> = todos_iter.filter_map(|t| t.ok()).collect();
    println!("Fetched todos: {:?}", todos);
    HttpResponse::Ok().json(todos)
}

async fn create_todo(todo: web::Json<Todo>) -> impl Responder {
    let conn = Connection::open("todo.db").unwrap();
    conn.execute(
        "INSERT INTO todos (title, completed) VALUES (?1, ?2)",
        params![todo.title, todo.completed],
    ).unwrap();
    println!("Inserted todo: {} (completed: {})", todo.title, todo.completed);
    let last_id = conn.last_insert_rowid() as i32;
    let new_todo = Todo {
        id: Some(last_id),
        title: todo.title.clone(),
        completed: todo.completed,
    };
    HttpResponse::Created().json(new_todo)
}

async fn update_todo(path: web::Path<i32>, todo: web::Json<Todo>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("todo.db").unwrap();
    conn.execute(
        "UPDATE todos SET title = ?1, completed = ?2 WHERE id = ?3",
        params![todo.title, todo.completed, id],
    ).unwrap();
    HttpResponse::Ok().json(todo.0)
}

async fn delete_todo(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("todo.db").unwrap();
    conn.execute("DELETE FROM todos WHERE id = ?1", &[&id.to_string()]).unwrap();
    HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("todo.db")?;
    create_table(&conn)?;

    use actix_files::{Files, NamedFile};

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(web::resource("/api/todos")
                .route(web::get().to(get_todos))
                .route(web::post().to(create_todo)))
            .service(web::resource("/api/todos/{id}").route(web::put().to(update_todo)))
            .service(web::resource("/api/todos/{id}").route(web::delete().to(delete_todo)))
            .service(Files::new("/", "./static").index_file("index.html"))
            .default_service(web::get().to(|| async {
                NamedFile::open_async("./static/index.html").await
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}