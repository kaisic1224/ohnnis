// ==== HTTP Header/Version ====

enum HTTPVersion {
    ZeroNine,
    One,
    OneOne,
    Two,
    Three,
}

impl HTTPVersion {
    pub fn value(&self) -> &str {
        match self {
            HTTPVersion::ZeroNine => "0.9",
            HTTPVersion::One => "1.0",
            HTTPVersion::OneOne => "1.1",
            HTTPVersion::Two => "2",
            HTTPVersion::Three => "3"
        }
    }
}


enum ServerType {
    Nginx,
    Apache,
    Lightppd
}

impl ServerType {
    fn value(&self) -> &str {
        match self {
            ServerType::Nginx => "nginx",
            ServerType::Apache => "apache",
            ServerType::Lightppd => "lightppd"
        }
    }
}

pub struct HTTPHeader {
    ssl: Option<bool>,
    server: Option<ServerType>,
    version: HTTPVersion,
}

impl HTTPHeader {
    pub fn new(server_type: &str, ssl_option: Option<bool>) -> HTTPHeader {
        let typ = match server_type.to_ascii_lowercase().as_str() {
            "nginx" => Some(ServerType::Nginx),
            "apache" => Some(ServerType::Apache),
            "lightppd" => Some(ServerType::Lightppd),
            _ => None
        };
        let xpp = HTTPHeader {server: typ, ssl: ssl_option, version: HTTPVersion::Two};

        xpp
    }

    // do i have to clone? think of some other way to return a rvalue
    pub fn get_version(&self) -> HTTPVersion {
        // (self.version)
    }

    // pub fn update<'a>(&mut self, server_type: &'a str) {
    //     self.server = server_type;
    // }
}

pub enum UserAgent {
    FfMacOs,
    ChromeMacOs,
    ChromeWin10,
    FfLinux
}


impl UserAgent {
    pub fn value(&self) -> &str {
        match *self {
            UserAgent::FfMacOs => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
            UserAgent::ChromeMacOs => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
            UserAgent::ChromeWin10 => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
            UserAgent::FfLinux => "Mozilla/5.0 (X11; Linux x86_64; rv:131.0) Gecko/20100101 Firefox/131.0"
        }
    }
}

impl std::fmt::Display for UserAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}


// ==== Misc Items ====

pub enum Locale {
    EnUs,
    KoKr
}

impl Locale {
    pub fn value(&self) -> &str {
        match *self {
            Locale::EnUs => "en-US",
            Locale::KoKr => "ko-KR"
        }
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
