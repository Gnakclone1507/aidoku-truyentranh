use aidoku_macro::get_manga_list;
use crate::parser::get_manga_list;

mod parser;

#[get_manga_list]
fn manga_list() {
    get_manga_list();
}
