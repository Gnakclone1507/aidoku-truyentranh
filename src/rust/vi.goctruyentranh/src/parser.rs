use aidoku::prelude::*;
use aidoku::std::net::Request;
use aidoku::std::String;
use aidoku::std::Vec;
use aidoku::Manga;
use aidoku::Chapter;
use aidoku::Page;

pub fn get_manga_list() -> Vec<Manga> {
    let html = Request::new("https://goctruyentranhvuiaa.com/danh-sach-truyen", Method::Get)
        .html();
    // Phân tích HTML để lấy danh sách truyện
    Vec::new()
}

pub fn get_chapter_list(manga_id: String) -> Vec<Chapter> {
    let url = format!("https://goctruyentranhvuiaa.com/{}", manga_id);
    let html = Request::new(&url, Method::Get).html();
    // Phân tích HTML để lấy danh sách chương
    Vec::new()
}

pub fn get_page_list(chapter_id: String) -> Vec<Page> {
    let url = format!("https://goctruyentranhvuiaa.com/{}", chapter_id);
    let html = Request::new(&url, Method::Get).html();
    // Phân tích HTML để lấy danh sách trang ảnh
    Vec::new()
}
