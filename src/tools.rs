use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::SqlitePool;

pub fn define_tools() -> Vec<Value> {
    vec![
        json!({
            "type":"function",
            "function": {
                "name":"create_item",
                "description":"Adds an item to the shopping list",
                "parameters":{
                    "type":"object",
                    "properties":{
                        "item":{
                            "type":"string",
                            "description":"The element to be added to the shopping list"
                        }
                    },
                    "required":["item"]
                }
            }
        }),
        json!({
            "type":"function",
            "function": {
                "name": "read_list",
                "description":"Return all the items in the shopping list",
                "parameters":{
                    "type":"object",
                    "properties":{}
                }
            }
        }),
        json!({
            "type":"function",
            "function": {
                "name":"update_item",
                "description":"Updates an existing item in the shopping list",
                "parameters": {
                    "type":"object",
                    "properties":{
                        "id":{
                            "type":"integer",
                            "description":"The id of the item to be updated"
                        },
                        "item":{
                            "type":"string",
                            "description":"The new value of the item",
                        }
                    },
                    "required": ["id", "item"]
                }
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "delete_item",
                "description": "Rimuove un elemento dalla lista della spesa",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {
                            "type": "integer",
                            "description": "L'id dell'elemento da rimuovere"
                        }
                    },
                    "required": ["id"]
                }
            }
        }),
    ]
}

#[derive(Deserialize)]
struct CreateItemArgs {
    item: String,
}

#[derive(Deserialize)]
struct UpdateItemArgs {
    id: i64,
    item: String,
}

#[derive(Deserialize)]
struct DeleteItemArgs {
    id: i64,
}

pub async fn dispatch_tool(name: &str, arguments: &str, pool: &SqlitePool) -> String {
    match name {
        "create_item" => {
            let args: CreateItemArgs = match serde_json::from_str(arguments) {
                Ok(a) => a,
                Err(e) => return format!("Arguments parsing Error: {}", e),
            };
            db_create_item(pool, &args.item).await
        }
        "read_list" => db_read_list(pool).await,
        "update_item" => {
            let args: UpdateItemArgs = match serde_json::from_str(arguments) {
                Ok(a) => a,
                Err(e) => return format!("Arguments parsing Error: {}", e),
            };
            db_update_item(pool, args.id, &args.item).await
        }
        "delete_item" => {
            let args: DeleteItemArgs = match serde_json::from_str(arguments) {
                Ok(a) => a,
                Err(e) => return format!("Arguments parsing Error: {}", e),
            };
            db_delete_item(pool, args.id).await
        }
        _ => format!("Unknown Tool: {}", name),
    }
}

async fn db_create_item(pool: &SqlitePool, item: &str) -> String {
    let result = sqlx::query("INSERT INTO items (name) VALUES (?)")
        .bind(item)
        .execute(pool)
        .await;

    match result {
        Ok(r) => format!("Added: '{}' (id: {})", item, r.last_insert_rowid()),
        Err(e) => format!("Input Error: {}", e),
    }
}

async fn db_read_list(pool: &SqlitePool) -> String {
    let result = sqlx::query_as::<_, (i64, String)>("SELECT id, name FROM items")
        .fetch_all(pool)
        .await;

    match result {
        Ok(rows) => {
            if rows.is_empty() {
                "Shopping List it's empty".to_string()
            } else {
                rows.iter()
                    .map(|(id, name)| format!("[{}] {}", id, name))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        }
        Err(e) => format!("Reading Error: {}", e),
    }
}

async fn db_update_item(pool: &SqlitePool, id: i64, item: &str) -> String {
    let result = sqlx::query("UPDATE items SET name = ? WHERE id = ?")
        .bind(item)
        .bind(id)
        .execute(pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => format!("Id Updated {}: '{}'", id, item),
        Ok(_) => format!("No elements matched id {}", id),
        Err(e) => format!("Update Error: {}", e),
    }
}

async fn db_delete_item(pool: &SqlitePool, id: i64) -> String {
    let result = sqlx::query("DELETE FROM items WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => format!("Id Removed {}", id),
        Ok(_) => format!("No elements matched id {}", id),
        Err(e) => format!("Removing Error: {}", e),
    }
}
