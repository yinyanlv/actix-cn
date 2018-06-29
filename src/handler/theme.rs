use diesel;
use diesel::*;
use diesel::sql_types::Integer;
use actix::*;
use actix_web::*;
use timeago;
use chrono::{Utc, Datelike, Timelike, NaiveDateTime};
use model::response::{ThemeListMsgs, Msgs, ThemeAndCommentsMsgs, ThemePageListMsgs};
use model::theme::{Theme, ThemeList,ThemePageList, ThemeListResult, ThemeId, NewTheme, ThemeNew, Comment, CommentReturn, NewComment, ThemeComment};
use model::category::Category;
use model::db::ConnDsl;
use model::user::User;
use utils::{time, markdown_to_html, state::PAGE_SIZE};

impl Handler<ThemeList> for ConnDsl {
    type Result = Result<ThemeListMsgs, Error>;

    fn handle(&mut self, theme_list: ThemeList, _: &mut Self::Context) -> Self::Result {
        use utils::schema::themes::dsl::*;
        use utils::schema::users;
        use utils::schema::categorys;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let mut themes_result = themes.order(id).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
        let theme_count = themes_result.len() as i32;
        let theme_page_count = (theme_count + PAGE_SIZE - 1) / PAGE_SIZE ;
        let mut themes_page_result = sql_query("SELECT * FROM themes ORDER BY themes.id ASC limit $1 OFFSET $2")
            .bind::<Integer, _>(PAGE_SIZE)
            .bind::<Integer, _>(0)
            .load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
        let mut themes_list: Vec<ThemeListResult> = vec![];
        for theme_one in themes_page_result {
                let mut themes_list_one = ThemeListResult::new();
                themes_list_one.id = theme_one.id;
                themes_list_one.user_id = theme_one.user_id;
                themes_list_one.category_id = theme_one.category_id;
                themes_list_one.theme_status = theme_one.theme_status;
                themes_list_one.title = theme_one.title;
                themes_list_one.content = theme_one.content;
                themes_list_one.view_count = theme_one.view_count;
                themes_list_one.comment_count = theme_one.comment_count;
                themes_list_one.created_at = theme_one.created_at;
                let rtime = time( Utc::now().naive_utc(), theme_one.created_at);
                themes_list_one.rtime = rtime;
                let category_result =  categorys::table.filter(categorys::id.eq(theme_one.category_id)).load::<Category>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match category_result {
                    Some(category_one) => { themes_list_one.category_name = category_one.category_name; },
                    None => { println!("No category result");},
                }
                let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match theme_user {
                    Some(user) => { themes_list_one.username = user.username;},
                    None => { println!("No theme_user result");},
                }
                themes_list.push(themes_list_one);
            }
            Ok(ThemeListMsgs { 
                status: 200,
                message : "theme_list result.".to_string(),
                theme_list: themes_list,
                theme_page_count: theme_page_count
            })
    }
}

impl Handler<ThemePageList> for ConnDsl {
    type Result = Result<ThemePageListMsgs, Error>;

    fn handle(&mut self, theme_page_list: ThemePageList, _: &mut Self::Context) -> Self::Result {
        use utils::schema::themes::dsl::*;
        use utils::schema::users;
        use utils::schema::categorys;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let mut themes_result = themes.order(id).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
        let theme_count = themes_result.len() as i32;
        let theme_page_count = (theme_count + PAGE_SIZE - 1) / PAGE_SIZE ;
        let mut themes_page_result = sql_query("SELECT * FROM themes WHERE themes.category_id <> 1 ORDER BY themes.id DESC limit $1 OFFSET $2")
            .bind::<Integer, _>(PAGE_SIZE)
            .bind::<Integer, _>((theme_page_list.page_id - 1) * PAGE_SIZE)
            .load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
        let mut themes_list: Vec<ThemeListResult> = vec![];
        for theme_one in themes_page_result {
                let mut themes_list_one = ThemeListResult::new();
                themes_list_one.id = theme_one.id;
                themes_list_one.user_id = theme_one.user_id;
                themes_list_one.category_id = theme_one.category_id;
                themes_list_one.theme_status = theme_one.theme_status;
                themes_list_one.title = theme_one.title;
                themes_list_one.content = theme_one.content;
                themes_list_one.view_count = theme_one.view_count;
                themes_list_one.comment_count = theme_one.comment_count;
                themes_list_one.created_at = theme_one.created_at;
                let rtime = time( Utc::now().naive_utc(), theme_one.created_at);
                themes_list_one.rtime = rtime;
                let category_result =  categorys::table.filter(categorys::id.eq(theme_one.category_id)).load::<Category>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match category_result {
                    Some(category_one) => { themes_list_one.category_name = category_one.category_name; },
                    None => { println!("No category result");},
                }
                let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match theme_user {
                    Some(user) => { themes_list_one.username = user.username;},
                    None => { println!("No theme_user result");},
                }
                themes_list.push(themes_list_one);
            }
            Ok(ThemePageListMsgs { 
                status: 200,
                message : "theme_list result.".to_string(),
                theme_list: themes_list,
                theme_page_count: theme_page_count
            })
    }
}

impl Handler<ThemeId> for ConnDsl {
    type Result = Result<ThemeAndCommentsMsgs, Error>;
    fn handle(&mut self, theme_one: ThemeId, _: &mut Self::Context) -> Self::Result {
        use utils::schema::themes::dsl::*;
        use utils::schema::users;
        use utils::schema::categorys;
        use utils::schema::comments;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        diesel::update(themes).filter(&id.eq(&theme_one.theme_id)).set((view_count.eq(view_count + 1),)).execute(conn).map_err(error::ErrorInternalServerError)?;
        let the_theme =  themes.filter(&id.eq(&theme_one.theme_id)).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?.pop();
        let mut theme_and_comments = ThemeAndCommentsMsgs::new();
        match the_theme {
            Some(mut themeid) => {
                themeid.content = markdown_to_html(&themeid.content);
                theme_and_comments.theme = themeid.clone();
                let mut theme_comment = comments::table.filter(&comments::theme_id.eq(&themeid.id)).load::<Comment>(conn).map_err(error::ErrorInternalServerError)?;
                for comment in &mut theme_comment {
                    let mut comment_list_one = CommentReturn::new();
                    comment_list_one.id = comment.id;
                    comment_list_one.theme_id = comment.theme_id;
                    comment_list_one.user_id = comment.user_id;
                    comment_list_one.content = markdown_to_html(&comment.content);
                    comment_list_one.created_at = comment.created_at;
                    let comment_user = users::table.filter(&users::id.eq(comment.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                    match comment_user {
                            Some(someuser) => {  
                            comment_list_one.username = someuser.username; },
                            None => { println!("No comment_user"); },
                    }
                    comment_list_one.rtime = time(Utc::now().naive_utc(), comment.created_at);
                    theme_and_comments.theme_comments.push(comment_list_one);
                }
                let category_result =  categorys::table.filter(&categorys::id.eq(&themeid.category_id)).load::<Category>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match category_result {
                    Some(category_one) => {
                        theme_and_comments.theme_category_name = category_one.category_name;},
                    None => { println!("No theme_category"); },
                }
                theme_and_comments.theme_rtime = time(Utc::now().naive_utc(), themeid.created_at);
                let user_result = users::table.filter(&users::id.eq(&themeid.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match user_result {
                    Some(themeid_user) => { 
                        theme_and_comments.theme_user = themeid_user; },
                    None => { println!("No theme_user"); },
                }
            },
            None => { println!("No theme"); },
        }
        Ok(ThemeAndCommentsMsgs{
            theme: theme_and_comments.theme,
            theme_user: theme_and_comments.theme_user,
            theme_category_name: theme_and_comments.theme_category_name,
            theme_rtime: theme_and_comments.theme_rtime,
            theme_comments: theme_and_comments.theme_comments,
        })
    }
}

impl Handler<ThemeNew> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, theme_new: ThemeNew, _: &mut Self::Context) -> Self::Result {
        use utils::schema::themes::dsl::*;
        use utils::schema::categorys;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let category_one =  categorys::table.filter(categorys::category_name_cn.eq(theme_new.category_name)).load::<Category>(conn).map_err(error::ErrorInternalServerError)?.pop();
        let cid: i32 ;
        match category_one {
            Some(one) => { cid = one.id;},
            None => { cid = 0;},
        }    
        let new_theme = NewTheme {
            user_id: theme_new.user_id,
            category_id: cid,
            title: &theme_new.title,
            content: &theme_new.content,
            created_at: Utc::now().naive_utc(),
        };
        diesel::insert_into(themes).values(&new_theme).execute(conn).map_err(error::ErrorInternalServerError)?;
        Ok(Msgs { 
                status: 200,
                message : "Theme Post Successful.".to_string(),
        })        
    }
}

impl Handler<ThemeComment> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, theme_comment: ThemeComment, _: &mut Self::Context) -> Self::Result {
        use utils::schema::comments::dsl::*;
        use utils::schema::themes;
        let the_theme_id: i32 = theme_comment.the_theme_id.to_owned().parse().map_err(error::ErrorBadRequest)?;
        let new_comment = NewComment {
            theme_id: the_theme_id,
            user_id: theme_comment.user_id,
            content: &theme_comment.comment,
            created_at: Utc::now().naive_utc(),
        };
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        diesel::insert_into(comments).values(&new_comment).execute(conn).map_err(error::ErrorInternalServerError)?;
        diesel::update(themes::table)
            .filter(&themes::id.eq(&the_theme_id))
            .set((themes::comment_count.eq(themes::comment_count + 1),))
            .execute(conn).map_err(error::ErrorInternalServerError)?;
        Ok(Msgs { 
                status: 200,
                message : "Comment Add Successful.".to_string(),
        })        
    }
}

