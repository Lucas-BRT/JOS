mod display_name;
mod email;
mod password;
mod phone_number;
mod user_role;
mod username;

pub use display_name::DisplayNameVo;
pub use email::EmailVo;
pub use password::Hashed;
pub use password::PasswordVo;
pub use password::Raw;
pub use phone_number::PhoneNumberVo;
pub use user_role::UserRoleVo;
pub use username::UsernameVo;

pub const MAX_DISPLAY_NAME_LENGTH: usize = 30;
pub const MIN_DISPLAY_NAME_LENGTH: usize = 5;
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 100;
pub const MAX_USERNAME_LENGTH: usize = 30;
pub const MIN_USERNAME_LENGTH: usize = 5;
