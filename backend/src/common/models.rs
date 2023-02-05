use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct IdRequest {
    pub id: i32,
}


#[derive(Serialize)]
pub struct ObjectList<T: Serialize> {
    pub objects: Vec<T>
}
