fn main() {
    let file_contents = std::fs::read_to_string("src/data/content.txt").unwrap();
    for line in file_contents
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
    {
        let res = parse_url(line);
        match res {
            Some(url) => println!("Is a URL: {}", url),
            _ => println!("Not a URL"),
        }
    }
}

fn parse_url(input: &str) -> Option<url::Url> {
    match url::Url::parse(input) {
        Ok(url) => Some(url),
        _ => None,
    }
}
