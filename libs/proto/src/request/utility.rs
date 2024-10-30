use crate::request::schema::HTTPHeader;

async fn retry(config: HTTPHeader) {
}

pub fn get_base_url(url: &str) -> (bool, &str, String) {
    // remove any "https:// or http:// from beginning"
    let s = &url[..8];

    let ssl = match s {
        "https://" => true,
        // "http://" => false,
        _ => false,
    };

    let mut paths: Vec<&str> = vec![];
    if ssl {
        url[8..].split("/").for_each(|item| paths.push(item));
    } else {
        url[7..].split("/").for_each(|item| paths.push(item));
    }

    (ssl, paths[0], paths[1..].join("/").to_string())
}
