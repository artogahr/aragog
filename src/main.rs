use clap::{arg, Command};
use std::io::Write;
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

    work_website(url, depth);
}

fn work_website(url: &String, depth: u32) {
    dbg!(url);
    let info = Webpage::from_url(url, WebpageOptions::default());
    let mut path = std::env::current_dir().unwrap();
    path.push("output");
    match Result::from(info) {
        Ok(info) => {
            path.push(info.html.title.unwrap());
            std::fs::create_dir_all(&path).unwrap();
            path.push("index.html");
            let mut file = std::fs::File::create(path).unwrap();
            file.write_all(info.html.text_content.as_bytes()).unwrap();
            if depth > 0 {
                for link in info.html.links {
                    //check if the link is an url
                    if link.url.starts_with("http") {
                        work_website(&link.url, depth - 1);
                    }
                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
