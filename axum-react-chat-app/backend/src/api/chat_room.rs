use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    ModelTrait, QueryFilter,
};

use crate::entities::{
    chat::{self, Entity as ChatEntity},
    room::{ActiveModel, Column, Entity as RoomEntity, Model},
};

pub async fn get_room(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<NewRoom>> {
    let mut condition = Condition::all();

    if let Some(id) = params.get("id") {
        condition = condition.add(Column::Id.eq(id.parse::<i32>().unwrap()));
    }

    let rooms = RoomEntity::find()
        .filter(condition)
        .all(&conn)
        .await
        .unwrap();

    let mut new_rooms: Vec<NewRoom> = Vec::new();

    for room in rooms {
        let participants: Vec<String> = serde_json::from_str(&room.participants).unwrap();

        new_rooms.push(NewRoom {
            id: Some(room.id),
            participants,
        });
    }

    Json(new_rooms)
}

#[derive(Serialize, Deserialize)]
pub struct NewRoom {
    id: Option<i32>,
    participants: Vec<String>,
}

pub async fn post_room(
    State(conn): State<DatabaseConnection>,
    Json(room): Json<NewRoom>,
) -> Json<Model> {
    let participants = serde_json::to_string(&room.participants).unwrap();

    let room = ActiveModel {
        id: ActiveValue::not_set(),
        participants: ActiveValue::Set(participants),
    };

    Json(room.insert(&conn).await.unwrap())
}

pub async fn put_room(
    State(conn): State<DatabaseConnection>,
    Json(room): Json<NewRoom>,
) -> Json<Model> {
    let result = RoomEntity::find_by_id(room.id.unwrap())
        .one(&conn)
        .await
        .unwrap()
        .unwrap();

    let mut participants: Vec<String> = serde_json::from_str(&result.participants).unwrap();
    participants.push(room.participants[0].clone());

    let new_room = ActiveModel {
        id: ActiveValue::Set(result.id),
        participants: ActiveValue::Set(serde_json::to_string(&participants).unwrap()),
    };

    Json(new_room.update(&conn).await.unwrap())
}

pub async fn delete_room(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<&'static str> {
    let id = params.get("id").unwrap().parse::<i32>().unwrap();

    let chats = ChatEntity::find()
        .filter(chat::Column::RoomId.eq(id))
        .all(&conn)
        .await
        .unwrap();

    for chat in chats {
        chat.delete(&conn).await.unwrap();
    }

    let room = RoomEntity::find_by_id(id)
        .one(&conn)
        .await
        .unwrap()
        .unwrap();

    room.delete(&conn).await.unwrap();

    Json("Deleted")
}
