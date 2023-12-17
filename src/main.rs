use clap::{arg, Command};
use std::io::Write;
use url::Url;
use webpage::{Webpage, WebpageOptions};
fn main() {
    let matches = Command::new("aragog")
        .version("0.1")
        .author("Arto")
        .about("Crawls a link up to the depth of N recursions and saves them to local files")
        .arg(arg!(--url <VALUE>).short('u').required(true))
        .arg(
            arg!(--depth <VALUE>)
                .short('n')
                .default_value("2")
                .required(false),
        )
        .get_matches();

    let url = matches.get_one::<String>("url").unwrap();

    dbg!(url);

    let depth = matches
        .get_one::<String>("depth")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let mut all_links: Vec<String> = Vec::new();

    work_website(url, 0, depth, &mut all_links);
}

fn work_website(url: &String, depth: u32, depth_remaining: u32, all_links: &mut Vec<String>) {
    all_links.push(url.to_string());
    println!("{}  {}", " ".repeat(depth as usize), url);
    let info = Webpage::from_url(url, WebpageOptions::default());
    let mut path = std::env::current_dir().unwrap();
    path.push("output");
    match Result::from(info) {
        Ok(info) => {
            //recurse through the links
            if depth_remaining > 0 {
                let current_domain = Url::parse(url).unwrap().domain().unwrap().to_string();
                for link in info.html.links {
                    //check if the link is an URL and has a different domain
                    if link.url.starts_with("http") {
                        let link_domain =
                            Url::parse(&link.url).unwrap().domain().unwrap().to_string();
                        if link_domain != current_domain && !all_links.contains(&link.url) {
                            work_website(&link.url, depth + 1, depth_remaining - 1, all_links);
                        }
                    }
                }
            }
            //create a directory for the page
            match info.html.title {
                Some(title) => {
                    path.push(title);
                }
                None => {
                    path.push(info.html.url.unwrap());
                }
            }
            std::fs::create_dir_all(&path).unwrap();
            path.push("index.html");
            let mut file = std::fs::File::create(path).unwrap();
            file.write_all(info.html.text_content.as_bytes()).unwrap();
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
