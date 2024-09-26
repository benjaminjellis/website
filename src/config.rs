use std::net::SocketAddr;

pub(crate) struct Config {
    pub(crate) db_url: String,
    pub(crate) socket: SocketAddr,
}

impl Config {
    pub(crate) fn new() -> Self {
        let socket: ([u8; 4], u16) = ([0, 0, 0, 0], 4000);
        let db_url = std::env::var("DB_URL").expect("Missing databse url env var");
        Self {
            db_url,
            socket: SocketAddr::from(socket),
        }
    }
}
