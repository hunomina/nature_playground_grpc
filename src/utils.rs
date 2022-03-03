pub fn get_server_http_endpoint() -> String {
    format!("http://{}", get_server_address())
}

pub fn get_server_address() -> String {
    std::env::var("SERVER_ADDRESS").unwrap_or(format!("0.0.0.0:{}", get_server_port()))
}

pub fn get_server_port() -> String {
    std::env::var("SERVER_PORT").unwrap_or("10000".into())
}