use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct ImageFile {
    pub filename: String,
    pub content_type: String,
    pub data: Bytes,
}
