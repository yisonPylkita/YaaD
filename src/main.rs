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
use curl::easy::{Easy, List};

use std::env;

extern crate select;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

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
    {
        easy.url(supported_sites[0]).unwrap();
        let mut list = List::new();
        list.append("User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:65.0) Gecko/20100101 Firefox/65.0").unwrap();
        easy.http_headers(list).unwrap();
        easy.perform().unwrap();
    }
    let mut anime_list_html: String = String::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                anime_list_html += &String::from_utf8(Vec::from(data)).unwrap();
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    struct MatchingAnimeLink {
        name: String,
        url: String,
    }

    let mut matching_series: Vec<MatchingAnimeLink> = Vec::default();

    let document = Document::from(anime_list_html.as_ref());
    for node in document.find(Attr("itemprop", "url")) {
        let series_name = node.text();
        match series_name.to_lowercase().find(anime_to_download) {
            Some(_) => {
                matching_series.push(MatchingAnimeLink {
                    name: series_name,
                    url: node.attr("href").unwrap().to_string(),
                });
            }
            None => {}
        }
    }

    println!("Matching anime:");
    // Look for all anime series that might be what user wanted
    for (i, matching_anime) in matching_series.iter().enumerate() {
        println!("{}: {} | URL: {:?}", i + 1, matching_anime.name, matching_anime.url);
    }

    // Let user choose anime series to download
    let mut anime_index_str = String::new();
    std::io::stdin()
        .read_line(&mut anime_index_str)
        .expect("Failed to read anime series number from stdin");
    let anime_index = anime_index_str
        .trim()
        .parse::<usize>()
        .expect("Could not parse anime index as number");
    if anime_index < 1 || anime_index > matching_series.len() {
        panic!("Invalid anime number");
    }
    let selected_anime = &matching_series[anime_index - 1];

    // Get list of episodes
    println!("Lets look for episodes of {} | {}", selected_anime.name, selected_anime.url);
    easy.url(&*selected_anime.url).unwrap();
    let mut html_2: String = String::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                html_2 += &String::from_utf8(Vec::from(data)).unwrap();
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    // TODO: get list of episodes

    let mut episode_list: Vec<String> = Vec::new();
    let document2 = Document::from(html_2.as_ref());
    for node in document2.find(Attr("itemprop", "episodeNumber").descendant(Name("a"))) {
        episode_list.push(node.attr("href").unwrap().to_string());
    }
    episode_list.reverse();

    // TODO: download all episodes
    // TODO: add 
    for (i, episode) in episode_list.iter().enumerate() {
        println!("Episode: {}: {}", i + 1, episode);
    }

    for episode in episode_list.iter() {
        easy.url(&*episode).unwrap();
        let mut episode_html: String = String::new();
        {
            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                    episode_html += &String::from_utf8(Vec::from(data)).unwrap();
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }

        let document = Document::from(episode_html.as_ref());
        for node in document.find(Attr("id", "download")) {
            let episode_download_url = node.attr("href").unwrap();
            println!("Episode {}, URL: {}", episode, episode_download_url);
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
