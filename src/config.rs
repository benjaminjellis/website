use std::net::SocketAddr;

#[allow(dead_code)]
pub(crate) struct Config {
    pub(crate) db_url: String,
    pub(crate) socket: SocketAddr,
}

impl Config {
    pub(crate) fn new() -> Self {
        let socket: ([u8; 4], u16) = ([0, 0, 0, 0], 4000);
        Self {
            db_url: "postgres://postgres@localhost:5432/blog-db".into(),
            socket: SocketAddr::from(socket),
        }
    }
}
