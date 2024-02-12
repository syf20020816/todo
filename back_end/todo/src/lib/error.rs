use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    /// 身份认证失败
    /// 说明登录时的账号或密码错误
    IdentityAuthentication,
    ExistAccount,
    ChangeUserSetting,
    ChangeUserAvatar,
}

impl Error {
    pub fn get(self) -> (u16, String) {
        match self {
            Error::IdentityAuthentication => (1001, self.to_string()),
            Error::ExistAccount => (1002, self.to_string()),
            Error::ChangeUserSetting => (1003, self.to_string()),
            Error::ChangeUserAvatar => (1004, self.to_string()),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alert_msg = match self {
            Error::IdentityAuthentication => {
                "Identity Authentication Failed: Incorrect username or password"
            }
            Error::ExistAccount => "The current account already exists. Please change the username",
            Error::ChangeUserSetting => "Failed to modify user configuration",
            Error::ChangeUserAvatar => "Failed to modify user avatar",
        };
        f.write_str(alert_msg)
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(self)
    }
}
