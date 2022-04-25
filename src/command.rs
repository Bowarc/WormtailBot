#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    TBan(String, Option<String>), // ban user name, reason
    TUnban(String),               // ban user name
    TSay(String),
    // None,
}
