use std::collections::HashMap;

pub const PAGE_SIZE: i32 = 4;
//pub const PAGE_SIZE: i32 = 33;

pub fn get_category_id(category_name: &str) -> i32 {
    let mut category_dir = HashMap::new();
    category_dir.insert("blog", 2);
    category_dir.insert("faq", 3);
    category_dir.insert("share", 4);
    category_dir.insert("job", 5);
    let category_id  = category_dir.get(category_name).unwrap();
    *category_id
}