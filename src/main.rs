use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Todo {
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let reqwest_client = reqwest::Client::new();
    let todos: Vec<Todo> = reqwest_client
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;

    println!("GET Todos {:#?}", todos);

    let new_todo = Todo {
        user_id: 1,
        id: None,
        title: "ASDF".to_owned(),
        completed: false,
    };

    let new_todo: Todo = reqwest_client
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo)
        .send()
        .await?
        .json()
        .await?;

    println!("POST new Todo {:#?}", new_todo);

    Ok(())
}
