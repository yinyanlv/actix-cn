use model::user::User;
use model::theme::{ThemeListResult, Theme, CommentReturn, Comment};
use model::category::{Category, CategoryThemeListResult};

pub enum MyError {
    NotFound,
    DatabaseError,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct Msgs {
    pub status: i32,
    pub message : String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct SigninMsgs {
    pub status: i32,
    pub token: String,
    pub signin_user: User,
    pub message : String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct ThemeListMsgs {
    pub status: i32,
    pub message : String,
    pub theme_list: Vec<ThemeListResult>,
    pub theme_page_count: i32,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct ThemePageListMsgs {
    pub status: i32,
    pub message : String,
    pub theme_list: Vec<ThemeListResult>,
    pub theme_page_count: i32,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct UserInfoMsgs {
    pub status: i32,
    pub message : String,
    pub current_user: User,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct ThemeAndCommentsMsgs {
    pub theme: Theme,
    pub theme_user: User,
    pub theme_category_name: String,
    pub theme_rtime: String,
    pub theme_comments: Vec<CommentReturn>,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct CategorysMsgs {
    pub status: i32,
    pub message : String,
    pub categorys : Vec<Category>,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct CategoryThemePageListMsgs {
    pub status: i32,
    pub message : String,
    pub category_theme_list : Vec<CategoryThemeListResult>,
    pub theme_category_page_count: i32,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct UserThemesMsgs {
    pub status: i32,
    pub message : String,
    pub themes : Vec<Theme>,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct UserCommentsMsgs {
    pub status: i32,
    pub message : String,
    pub comments : Vec<Comment>,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct UserSavesMsgs {
    pub status: i32,
    pub message : String,
    pub saves : Vec<Theme>,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct BlogLikeMsgs {
    pub status: i32,
    pub message : String,
    pub number : i32,
    pub saveorno : bool,
}
impl ThemeAndCommentsMsgs {
    pub fn new() -> ThemeAndCommentsMsgs {
            ThemeAndCommentsMsgs{
                theme: Theme::new(),
                theme_user: User::new(),
                theme_category_name: "".to_string(),
                theme_rtime: "".to_string(),
                theme_comments: vec![],
            }
    }
}