// yaad - Yet another anime downloader
//
// [WIP]
// When completed (v1.0) will alow you to crawl through web and download anime
//
// Usage: yaad [options] anime_name
//
// This should (as you might expect) find this anime, list its episodes and ask you for
// permission to start downloading selected anime episodes
//
// In future I want this application to look for anime to download on multiple sites and for
// each anime episode select one with best bandwidth
//
// I am also thinking about downloading from torrents but this requires some thinking - this way
// you will be also seeding to others and it might raise some concerns for users (because of the law)

extern crate curl;
use curl::easy::Easy;

use std::env;

extern crate select;
use select::document::Document;
use select::predicate::{Attr};


// struct AnimeSite
// {
//     url: String,
// }

// impl AnimeSite {
//     fn new(site_url: String) -> AnimeSite {
//         return AnimeSite {
//             url: site_url
//         }
//     }

//     fn get_matching_anime_urls(anime_name: String) -> Option<Vec<String>> {
//         let mut easy = Easy::new();
//         easy.url("https://animeheaven.co/watch/steins-gate-0").unwrap();
//         easy.write_function(|data| {
//             stdout().write_all(data).unwrap();
//             Ok(data.len())
//         }).unwrap();
//         easy.perform().unwrap();

//         println!("{}", easy.response_code().unwrap());
//     }
// }

fn main() {
    let supported_sites = vec!["http://ww2.chia-anime.tv/index/"];
 
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expected name of anime to download");
        return;
    }

    let anime_to_download = &args[1].to_lowercase();;
    println!("Aye, aye, Capt'n. Lets look for this anime - {}\n", anime_to_download);

    // Get all available anime series

    let mut easy = Easy::new();
    easy.url(supported_sites[0]).unwrap();
    let mut html: String = String::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            html += &String::from_utf8(Vec::from(data)).unwrap();
            Ok(data.len())
        }).unwrap(); 
        transfer.perform().unwrap();
    }

    struct MatchingAnimeLink {
        name: String,
        url: String,
    }

    let mut matching_series: Vec<MatchingAnimeLink> = Vec::default();

    let document = Document::from(html.as_ref());
    for node in document.find(Attr("itemprop", "url")) {
        let series_name = node.text();
        match series_name.to_lowercase().find(anime_to_download) {
            Some(_) => {
                matching_series.push(MatchingAnimeLink {
                    name: series_name,
                    url: node.attr("href").unwrap().to_string(),
                });
            },
            None => {},
        }
    }

    println!("Matching anime:");
    // Look for all anime series that might be what user wanted
    for matching_anime in matching_series {
        println!("Anime: {} | URL: {:?}", matching_anime.name, matching_anime.url);
    }
}