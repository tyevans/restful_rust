use crate::common::models::{ListPage, ListQuery};

pub fn list_query_to_page(query: ListQuery) -> ListPage {
    let page;
    let per_page;

    match query.page {
        Some(x) => {
            page = x;
        },
        None    => {
            page = 1;
        },
    }
    match query.per_page {
        Some(x) => {
            per_page = x;
        },
        None    => {
            per_page = 10;
        },
    }

    ListPage {
        page,
        per_page
    }
}