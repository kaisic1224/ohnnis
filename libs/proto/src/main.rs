mod request;

use request::schema::{HTTPHeader, UserAgent, Locale};
use request::utility::get_base_url;

use std::net::{SocketAddr, ToSocketAddrs};
use tokio::{io::{AsyncWriteExt, AsyncBufReadExt, BufReader}, net::TcpStream};
use tokio_native_tls::native_tls::TlsConnector;
use tokio_native_tls::TlsConnector as TokioTlsConnector;

#[tokio::main]
async fn main() {

    let header = fingerprint("fuckvoidopps.com/spinning-cock-cap/", UserAgent::FfLinux, Locale::EnUs).await;
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



// ==== fingerprinting the server ====
// fingerprint(url: &str) takes in a url and performs a request
// to the url, then returns a HTTPHeader denoting the fingerprint
// of the server.
// assumes: url is in a format with only the TLD and SLD
async fn fingerprint(url: &str, user_agent: UserAgent, locale: Locale) -> HTTPHeader {
    // first step, change the url from human-readable into its ip using dns-lookup
    // no syscall, we use our custom function
    let (ssl, base, path) = get_base_url(url);

    let mut config: HTTPHeader = HTTPHeader::new("yikesdawgie", Some(ssl));

    let mut ip: String = base.to_owned();
    // ip.push_str(":443");
    ip.push_str(":80");
    // other common ports include 21 (FTP), 587 (SMTP), 80 (http)
    let addr = ip.to_socket_addrs().unwrap().collect::<Vec<SocketAddr>>()[0];
    // attempt to use port 80 first

    let connector = TlsConnector::new().unwrap();
    let stream = TokioTlsConnector::from(connector);
    let conn = TcpStream::connect(addr).await.unwrap();
    let mut conn = stream.connect(base, conn).await.unwrap();

    let path = format!("GET /{} HTTP/{}", path, config.version.value());
    let host = format!("Host: {base}");
    let ua = format!("User-Agent: {}", user_agent.value());
    let lang = format!("Accept-Language: {},{};q=0.5", locale.value(), locale.value().split("-").collect::<Vec<&str>>()[0]);
    let req = [path.as_str(),
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
                // "Cache-Control: max-age=0",
                "\r\n"
    ].join("\r\n");

    conn.write_all(req.as_bytes()).await.unwrap();
    conn.flush().await.unwrap();

    let buf_reader = BufReader::new(&mut conn);
    let mut lines = buf_reader.lines();
    let mut headers: Vec<String> = vec![];
    while let Some(line) = lines.next_line().await.unwrap() {
        println!("Request: {line:#?}");

        // if line.contains("WordPress.com") {
        //     config.is_wordpress = true;
        // }

        if line.contains("Server:") {
            config.server = String::from(&line[8..]);
        }
        headers.push(line);
    }

    conn.shutdown().await.unwrap();
    config
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
