use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct IdRequest {
    pub id: uuid::Uuid,
}


#[derive(Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Deserialize)]
pub struct ListPage {
    pub page: i64,
    pub per_page: i64,
}

impl Default for ListPage {
    fn default() -> Self {
        ListPage {
            page: 1,
            per_page: 10,
        }
    }
}


#[derive(Serialize)]
pub struct Page {
    pub page: i64,
    pub count: i64,
    pub per_page: i64,
}


#[derive(Serialize)]
pub struct ObjectList<T: Serialize> {
    pub objects: Vec<T>,
    pub page: Page,
}
