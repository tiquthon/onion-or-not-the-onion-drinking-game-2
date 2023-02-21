pub const REPLACE_PROTOCOL_WEBSOCKET: ReplaceProtocol = ReplaceProtocol {
    secure: "wss:",
    unsecure: "ws:",
};

pub fn retrieve_browser_location(
    replace_protocol: Option<ReplaceProtocol>,
    append_path: Option<&str>,
) -> String {
    let location: web_sys::Location = web_sys::window().unwrap().location();
    let protocol = match location.protocol() {
        Ok(s) if s == "http:" => replace_protocol.map(|r| r.unsecure).unwrap_or("http:"),
        _ => replace_protocol.map(|r| r.secure).unwrap_or("https:"),
    };
    let host = location.host().unwrap();
    let pathname: String = location.pathname().unwrap();
    let stripped_pathname = pathname.strip_suffix('/').unwrap_or(&pathname);
    format!(
        "{protocol}//{host}{stripped_pathname}{}",
        append_path.unwrap_or("")
    )
}

pub struct ReplaceProtocol<'a> {
    pub secure: &'a str,
    pub unsecure: &'a str,
}
