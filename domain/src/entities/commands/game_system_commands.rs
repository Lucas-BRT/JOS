use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct CreateGameSystemCommand<'a> {
    pub id: Uuid,
    pub name: &'a str,
}

#[derive(Debug, Clone, Default)]
pub struct GetGameSystemCommand<'a> {
    pub id: Option<Uuid>,
    pub name: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub struct UpdateGameSystemCommand<'a> {
    pub id: Uuid,
    pub name: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub struct DeleteGameSystemCommand {
    pub id: Uuid,
}
