use clap::{arg, Command};
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

    println!(
        "URL: {:?}",
        matches
            .get_one::<String>("url")
            .expect("Initial URL is required")
    );
    println!("Depth: {:?}", matches.get_one::<String>("url").unwrap());
}
