/// Shitty URL parser, customized only to do what I need it to do

pub fn parse_url(url: &str) -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();

    vector.push(String::from("/"));

    for page in url.split("/") {
        vector.push(String::from(page));
    }

    vector
}