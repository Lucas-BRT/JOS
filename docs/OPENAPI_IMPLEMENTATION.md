# Implementação OpenAPI com utoipa

Esta documentação descreve a implementação da documentação OpenAPI usando `utoipa` e `utoipa-swagger-ui` na aplicação JOS.

## Visão Geral

A implementação OpenAPI foi migrada de uma abordagem manual para usar `utoipa`, que permite gerar automaticamente a documentação OpenAPI a partir das anotações no código Rust.

## Estrutura

### Arquivos Principais

- `src/interfaces/http/openapi/mod.rs` - Módulo principal do OpenAPI
- `src/interfaces/http/openapi/routes.rs` - Configuração do OpenAPI e Swagger UI
- `src/interfaces/http/openapi/schemas.rs` - Schemas OpenAPI definidos usando `ToSchema`

### Schemas Definidos

Os seguintes schemas foram definidos em `schemas.rs`:

#### Autenticação
- `SignupDto` - Dados para criação de conta
- `LoginDto` - Dados para login
- `UserSignupResponse` - Resposta de criação de usuário
- `LoginResponse` - Resposta de login
- `PasswordRequirementsResponse` - Requisitos de senha

#### Usuários
- `UserResponse` - Informações do usuário
- `MeResponse` - Informações do usuário atual

#### Tabelas
- `CreateTableDto` - Dados para criação de tabela
- `AvailableTableResponse` - Informações de tabela disponível

#### Solicitações de Tabela
- `CreateTableRequestDto` - Dados para criar solicitação
- `TableRequestResponse` - Informações de solicitação
- `UpdateTableRequestDto` - Dados para atualizar solicitação

#### Utilitários
- `HealthResponse` - Resposta de health check
- `ErrorResponse` - Resposta de erro
- `ValidationErrorResponse` - Resposta de erro de validação
- `FieldError` - Erro de campo específico

## Anotações utoipa

### Endpoints

Cada endpoint é documentado usando a anotação `#[utoipa::path]`:

```rust
#[utoipa::path(
    post,
    path = "/v1/auth/signup",
    tag = "auth",
    request_body = SignupDto,
    responses(
        (status = 201, description = "User created successfully", body = UserSignupResponse),
        (status = 400, description = "Bad request", body = ValidationErrorResponse)
    )
)]
```

### Schemas

Os schemas são definidos usando `#[derive(ToSchema)]`:

```rust
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SignupDto {
    #[schema(min_length = 4, max_length = 100)]
    pub name: String,
    #[schema(format = "email")]
    pub email: String,
    // ...
}
```

### Segurança

A autenticação JWT é documentada usando:

```rust
security(
    ("bearer_auth" = [])
)
```

## Configuração do Swagger UI

O Swagger UI está configurado em `/docs` e acessa a especificação OpenAPI em `/api-docs/openapi.json`.

### Acesso

- **URL da documentação**: `http://localhost:3000/docs`
- **URL da especificação**: `http://localhost:3000/api-docs/openapi.json`

## Tags Organizadas

A documentação está organizada em tags:

- **auth** - Endpoints de autenticação
- **users** - Gerenciamento de usuários
- **tables** - Gerenciamento de tabelas RPG
- **table-requests** - Gerenciamento de solicitações de tabela
- **health** - Health checks

## Vantagens da Implementação utoipa

1. **Geração Automática**: A documentação é gerada automaticamente a partir do código
2. **Sincronização**: A documentação sempre está sincronizada com o código
3. **Type Safety**: Erros de documentação são capturados em tempo de compilação
4. **Manutenibilidade**: Mudanças no código são refletidas automaticamente na documentação
5. **Validação**: Schemas são validados em tempo de compilação

## Exemplo de Uso

### Adicionando um Novo Endpoint

1. Defina o schema de request/response em `schemas.rs`:

```rust
#[derive(Debug, Deserialize, ToSchema)]
pub struct NewEndpointDto {
    pub field: String,
}
```

2. Adicione a anotação utoipa ao endpoint:

```rust
#[utoipa::path(
    post,
    path = "/v1/new-endpoint",
    tag = "new-tag",
    request_body = NewEndpointDto,
    responses(
        (status = 200, description = "Success", body = String)
    )
)]
async fn new_endpoint(/* ... */) -> Result<Json<String>> {
    // implementation
}
```

3. Adicione o endpoint ao `ApiDoc` em `routes.rs`:

```rust
#[derive(OpenApi)]
#[openapi(
    paths(
        // ... outros endpoints
        crate::path::to::new_endpoint
    ),
    components(
        schemas(
            // ... outros schemas
            NewEndpointDto
        )
    ),
    tags(
        // ... outras tags
        (name = "new-tag", description = "New tag description")
    )
)]
pub struct ApiDoc;
```

## Dependências

As seguintes dependências foram adicionadas ao `Cargo.toml`:

```toml
utoipa = { version = "5.0.0", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "7.0.0", features = ["axum"] }
```

## Migração da Implementação Anterior

A implementação anterior usava:
- `openapiv3 = "1.0.0"`
- `swagger-ui = "0.1.5"`

E gerava manualmente o JSON OpenAPI. A nova implementação usa `utoipa` para gerar automaticamente a documentação a partir das anotações no código.
