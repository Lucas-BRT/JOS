use crate::utils::jwt::Claims;
use serde::Deserialize;

// Esta struct pode ser usada como um extractor em seus handlers protegidos
// para obter acesso às claims do token validado.
#[derive(Debug, Deserialize)]
pub struct AuthenticatedUser {
    pub claims: Claims,
}
