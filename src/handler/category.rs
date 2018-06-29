use diesel;
use diesel::prelude::*;
use diesel::sql_types::Integer;
use diesel::dsl::sql_query;
use actix::*;
use actix_web::*;
use chrono::Utc;
use std::collections::HashSet;
use model::response::{CategorysMsgs, Msgs, CategoryThemePageListMsgs};
use model::db::ConnDsl;
use model::theme::Theme;
use model::user::User;
use utils::{time, state::PAGE_SIZE};
use model::category::{Category, Categorys, NewCategory, CategoryNew, CategoryThemePageList, CategoryThemeListResult};


impl Handler<CategoryNew> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, category_new: CategoryNew, _: &mut Self::Context) -> Self::Result {
        use utils::schema::categorys::dsl::*;
        let new_category = NewCategory {
            user_id: category_new.user_id,
            category_name: &category_new.category_name,
            category_name_cn: &category_new.category_name_cn,
            created_at: Utc::now().naive_utc(),
        };
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        diesel::insert_into(categorys).values(&new_category).execute(conn).map_err(error::ErrorInternalServerError)?;
        Ok(Msgs { 
                status: 200,
                message : "Article Publish Successful.".to_string(),
        })        
    }
}

impl Handler<Categorys> for ConnDsl {
    type Result = Result<CategorysMsgs, Error>;

    fn handle(&mut self, categorys: Categorys, _: &mut Self::Context) -> Self::Result {
        use utils::schema::categorys::dsl::*;
        let mut categorys_list: Vec<String> = vec![];
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let category_list = categorys.load::<Category>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(CategorysMsgs { 
                status: 200,
                message : "TypeNamesMsgs.".to_string(),
                categorys: category_list,
        })        
    }
}

impl Handler<CategoryThemePageList> for ConnDsl {
    type Result = Result<CategoryThemePageListMsgs, Error>;

    fn handle(&mut self, category_theme_page_list: CategoryThemePageList, _: &mut Self::Context) -> Self::Result {
        use utils::schema::themes::dsl::*;
        use utils::schema::users;
        use utils::schema::categorys;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let theme_category_one = categorys::table.filter(&categorys::category_name.eq(&category_theme_page_list.category_name)).load::<Category>(conn).map_err(error::ErrorInternalServerError)?.pop();
        let the_category_id = match theme_category_one {
            Some(theme_category_id) => theme_category_id.id,
            None => 0,
        };
        let mut themes_result = themes.filter(category_id.eq(the_category_id)).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
        let theme_category_count = themes_result.len() as i32;
        let theme_category_page_count = (theme_category_count + PAGE_SIZE - 1) / PAGE_SIZE ;
        let mut themes_page_result = sql_query("SELECT * FROM themes WHERE themes.category_id = $1 ORDER BY themes.id DESC limit $2 OFFSET $3")
            .bind::<Integer, _>(the_category_id)
            .bind::<Integer, _>(PAGE_SIZE)
            .bind::<Integer, _>((category_theme_page_list.page_id - 1) * PAGE_SIZE)
            .load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
        let mut category_themes_list: Vec<CategoryThemeListResult> = vec![];
        for theme_one in themes_page_result {
            let mut category_themes_list_one = CategoryThemeListResult::new();
            category_themes_list_one.id = theme_one.id;
            category_themes_list_one.user_id = theme_one.user_id;
            category_themes_list_one.category_id = theme_one.category_id;
            category_themes_list_one.theme_status = theme_one.theme_status;
            category_themes_list_one.title = theme_one.title;
            category_themes_list_one.content = theme_one.content;
            category_themes_list_one.view_count = theme_one.view_count;
            category_themes_list_one.comment_count = theme_one.comment_count;
            category_themes_list_one.created_at = theme_one.created_at;
            category_themes_list_one.rtime = time( Utc::now().naive_utc(), theme_one.created_at);
            let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
            match theme_user {
                Some(user) => {
                    category_themes_list_one.username = user.username;
                    category_themes_list.push(category_themes_list_one);
                },            
                None => { println!("No user result"); },
            }
        }
        Ok(CategoryThemePageListMsgs { 
            status: 200,
            message : "theme_list result.".to_string(),
            category_theme_list: category_themes_list,
            theme_category_page_count: theme_category_page_count,
        })       
    }
}
