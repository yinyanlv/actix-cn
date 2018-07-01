use actix::*;
use actix_web::*;
use utils::schema::categorys;
use chrono::{Utc, NaiveDateTime};
use model::response::{CategorysMsgs, Msgs, CategoryThemePageListMsgs};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Category {
    pub id: i32,
    pub user_id: i32,
    pub category_name: String,
    pub category_name_cn: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="categorys"]
pub struct NewCategory<'a> {
    pub user_id: i32,
    pub category_name: &'a str,
    pub category_name_cn: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct CategoryNew {
    pub user_id: i32,
    pub category_name: String,
    pub category_name_cn: String,
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct Categorys;

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct CategoryThemePageList {
    pub page_id: i32,
    pub category_name: String,
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct CategoryThemeListResult {
    pub id: i32,
    pub user_id: i32,
    pub category_id: i32,
    pub theme_status: i32,
    pub title: String,
    pub content: String,
    pub view_count: i32,
    pub comment_count: i32,
    pub created_at: NaiveDateTime,
    pub username: String,
    pub rtime: String,
}


impl Message for CategoryThemePageList {
    type Result = Result<CategoryThemePageListMsgs, Error>;
}
impl Message for CategoryNew {
    type Result = Result<Msgs, Error>;
}

impl Message for Categorys {
    type Result = Result<CategorysMsgs, Error>;
}


impl CategoryThemeListResult {
    pub fn new () -> CategoryThemeListResult {
        CategoryThemeListResult {
            id: 0,
            user_id: 0,
            category_id: 0,
            theme_status: 0,
            title: "".to_string(),
            content: "".to_string(),
            view_count: 0,
            comment_count: 0,
            created_at: Utc::now().naive_utc(),
            username: "".to_string(),
            rtime: "".to_string(),
        }
    }
}
