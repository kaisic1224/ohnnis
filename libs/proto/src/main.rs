use core::fmt;
use std::net::{SocketAddr, ToSocketAddrs};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {

    fingerprint("fuckvoidopps.com/spoiler-alert/", UserAgent::FfLinux, Locale::enUS).await;
    // let listener = TcpListener::bind("127.0.0.1:1224").await.unwrap();
    //
    // loop {
    //     let (mut socket, _) = listener.accept().await.unwrap();
    //     tokio::spawn(async move {
    //        println!("connection opened");
    //         socket.write(b"something");
    //     });
    // }
}




enum UserAgent {
    FfMacOs,
    ChromeMacOs,
    ChromeWin10,
    FfLinux
}

impl UserAgent {
    fn value(&self) -> &str {
        match *self {
            UserAgent::FfMacOs => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
            UserAgent::ChromeMacOs => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
            UserAgent::ChromeWin10 => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
            UserAgent::FfLinux => "Mozilla/5.0 (X11; Linux x86_64; rv:131.0) Gecko/20100101 Firefox/131.0"
        }
    }
}

impl fmt::Display for UserAgent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

enum Locale {
    enUS,
    koKR
}

impl Locale {
    fn value(&self) -> &str {
        match *self {
            Locale::enUS => "en-US",
            Locale::koKR => "ko-KR"
        }
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

fn get_base_url(url: &str) -> (&str, String) {
    let paths = url.split("/").collect::<Vec<&str>>();

    // remove any "https:// or http:// from beginning"

    (paths[0], paths[1..].join("/").to_string())
}

// ==== fingerprinting the server ====
// fingerprint(url: &str) takes in a url and performs a request
// to the url, then returns a HTTPHeader denoting the fingerprint
// of the server.
// assumes: url is in a format with only the TLD and SLD
async fn fingerprint(url: &str, user_agent: UserAgent, locale: Locale) {
    // first step, change the url from human-readable into its ip using dns-lookup
    // no syscall, we use our custom function
    let (base, path) = get_base_url(url);
    let mut ip: String = base.to_owned();
    ip.push_str(":80");
    // other common ports include 21 (FTP), 587 (SMTP)
    let addr = ip.to_socket_addrs().unwrap().collect::<Vec<SocketAddr>>()[0];
    let mut conn = TcpStream::connect(addr).await.unwrap();

    let path = format!("GET /{} HTTP/1.1", path);
    let host = format!("Host: {base}");
    let ua = format!("User-Agent: {}", user_agent.value());
    let lang = format!("Accept-Language: {},{};q=0.5", locale.value(), locale.value().split("-").collect::<Vec<&str>>()[0]);
    let resp = [path.as_str(),
                host.as_str(),
                ua.as_str(),
                "Accept: text/html, application/xhtml_xml, application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
                lang.as_str(),
                "Accept-Encoding: gzip, deflate, br",
                // "DNT: 1",
                // "Connection: keep-alive",
                // "Upgrade-Insecure-Requests: 1",
                // "Sec-Fetch-Dest: document",
                // "Sec-Fetch-Mode: navigate",
                // "Sec-Fetch-Site: none",
                // "Sec-Fetch-User: ?1",
                "Cache-Control: max-age=0",
                "\r\n"
    ].join("\r\n");

    println!("{resp}");
    conn.write_all(resp.as_bytes()).await.unwrap();
    conn.flush().await.unwrap();

    let buf_reader = BufReader::new(&mut conn);
    let mut lines = buf_reader.lines();
    let mut headers: Vec<String> = vec![];
    while let Some(line) = lines.next_line().await.unwrap() {
        // println!("length = {}", line.len());
        println!("Request: {line:#?}");
        headers.push(line);
    }
    // loop {
    //     let mut line = String::new();
    //     buf_reader.read_line(&mut line).await.unwrap();
    //     buf_reader.flush().await.unwrap();
    //     if line.is_empty() {
    //         break;
    //     }
    //
    //     println!("{line:#?}");
    // }


    // let res = String::from_utf8(buf.to_vec()).unwrap();
    // println!("{res}");

    conn.shutdown().await.unwrap();
    // let (stream, _) = listener.accept().await.unwrap();
    //
    // let buf_reader = BufReader::new(stream.into_std().unwrap());
    // let req: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    //
    // println!("{req:#?}")
}


// HOW TO USE SHARED STATE CONCURRENCY
//
// CLONE everything for synchronous operations
// When you are working in Async, use Arc<Mutex<T>>
// - then clone the thing that Arc points to whenever you need to access the thing inside Arc
// - Arcs = shared pointers, a way to keep track of how many objects are cloned and preserve its
//   lifetime across multiple instances
// - Mutexes = a state storage mechanism - store a piece of mutable data within a mutex, it will be
//   accessible iff it is not currently under read/write operations (from other threads) aka locked
//
//   CLOSURES AND move keyword - '||' defines a closure, any variables passed into the || will be
//   passed into the closure function. If 'move' preceeds a closure declaration, all variables used
//   in the closure will have their ownership transferred into the closure and be freed after
//   closure's lifetime is finsihed (wallahi).
//
//   CONSIDER ASKING THESE QUESTIONS:   
//   Do I need to pass around a read only copy? Use a ref. Example: Sharing a config struct.
//
//   Do I need to do the above but across multiple threads? Use an Arc.
//
//   Do I have single threaded shared data that needs to be edited? Use mut refs.
//
//   Do I need to keep something unedited in one path, and edited in another? Use a clone. Example: You have a vec of planet objects created at runtime by sourcing a DB, and a function that takes a vec of planets and uses them as state and then prints a cute interactive galaxy. You want the user to be able to reset to the default state while they play with their galaxy, so you hold the original. It's cheaper than requerying the DB.
//
//   Do I have to do the above across threads? Use an Arc<Mutex>. Example: You have the application running on Tokio. Or maybe more simply, you need to share a logger instance.
