use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use uuid::Uuid;

static DB: LazyLock<Mutex<HashMap<Uuid, Item>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Serialize)]
pub struct Inventory {
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub(crate) id: Option<Uuid>,
    pub(crate) name: String,
    pub(crate) stock: i32,
}

pub async fn get_inventory() -> Json<Inventory> {
    Json(Inventory {
        items: DB.lock().unwrap().values().cloned().collect(),
    })
}

pub async fn post_inventory_item(item: Json<Item>) -> Json<Item> {
    let id = item.id.unwrap_or_else(Uuid::now_v7);

    let item_with_id = Item {
        id: Some(id),
        ..item.0.clone()
    };

    DB.lock().unwrap().insert(id, item_with_id.clone());

    Json(item_with_id)
}
