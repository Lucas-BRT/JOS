use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateSessionCheckinCommand<'a> {
    pub session_intent_id: Uuid,
    pub attendance: bool,
    pub notes: Option<&'a str>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateSessionCheckinCommand<'a> {
    pub id: Uuid,
    pub session_intent_id: Option<Uuid>,
    pub attendance: Option<bool>,
    pub notes: Option<Option<&'a str>>,
}

#[derive(Debug, Clone)]
pub struct DeleteSessionCheckinCommand {
    pub id: Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct GetSessionCheckinCommand {
    pub id: Option<Uuid>,
    pub session_intent_id: Option<Uuid>,
    pub attendance: Option<bool>,
}
