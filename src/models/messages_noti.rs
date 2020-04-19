pub enum MessageSuccess {
    Login,
    Register,
    Join,
}
impl MessageSuccess {
    pub fn to_string(&self) -> String {
        match self {
            MessageSuccess::Login => format!("Login success"),
            MessageSuccess::Register => format!("Register success"),
            MessageSuccess::Join => format!("Join success"),
            _ => String::new(),
        }
    }
}
pub enum MessageErrors {
    DDos,
    Logged,
    UsernameExist,
    UserNotFound,
    UsnOrPwdInvalid,
    RoomInvalid,
    RoomFull,
}
impl MessageErrors {
    pub fn to_string(&self) -> String {
        match self {
            MessageErrors::DDos => format!("Fuck You!"),
            MessageErrors::UserNotFound => format!("User not found!"),
            MessageErrors::UsernameExist => format!("Username Exist!"),
            MessageErrors::UsnOrPwdInvalid => format!("Username or password invalid!"),
            MessageErrors::Logged => format!("Account is online. Please try again!"),
            MessageErrors::RoomInvalid => format!("Room join not found or invalid!"),
            MessageErrors::RoomFull => format!("Current Room is full. Please try again!"),
            _ => String::new(),
        }
    }
}
