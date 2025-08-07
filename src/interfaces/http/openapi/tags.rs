use utoipa::openapi::Tag;

pub const AUTH_TAG: &str = "Authentication";
pub const USER_TAG: &str = "Users";
pub const TABLE_TAG: &str = "Tables";
pub const TABLE_REQUEST_TAG: &str = "Table Requests";

pub fn auth_tag() -> Tag {
    let mut tag = Tag::new(AUTH_TAG);
    tag.description = Some("Authentication and authorization endpoints".to_string());
    tag
}

pub fn user_tag() -> Tag {
    let mut tag = Tag::new(USER_TAG);
    tag.description = Some("User management endpoints".to_string());
    tag
}

pub fn table_tag() -> Tag {
    let mut tag = Tag::new(TABLE_TAG);
    tag.description = Some("RPG table management endpoints".to_string());
    tag
}

pub fn table_request_tag() -> Tag {
    let mut tag = Tag::new(TABLE_REQUEST_TAG);
    tag.description = Some("Table request management endpoints".to_string());
    tag
}
