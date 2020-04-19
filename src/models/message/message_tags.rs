#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum MessageTags {
    None,
    Join,
    Exit,
    Config,
    Login,
    Register,
}
