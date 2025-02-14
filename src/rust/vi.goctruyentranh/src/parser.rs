use aidoku_std::prelude::*;
use aidoku_std::net::Request;
use aidoku_std::html::HtmlNode;
use aidoku_std::chapter::{Chapter, ChapterList};
use aidoku_std::manga::{Manga, MangaList};
use aidoku_std::page::{Page, PageList};

const BASE_URL: &str = "https://goctruyentranhvuiaa.com";

pub fn get_manga_list() -> MangaList {
    let url = format!("{}/danh-sach-truyen", BASE_URL);
    let html = Request::new(&url).html().unwrap();
    let mut manga_list = Vec::new();

    for item in html.select(".manga-list .item") {
        let title = item.select(".manga-title").text().unwrap();
        let cover_url = item.select(".manga-cover img").attr("src").unwrap();
        let link = item.select("a").attr("href").unwrap();
        manga_list.push(Manga {
            title: title.to_string(),
            cover_url: cover_url.to_string(),
            link: link.to_string(),
            ..Default::default()
        });
    }

    MangaList {
        manga: manga_list,
        has_more: false,
    }
}
