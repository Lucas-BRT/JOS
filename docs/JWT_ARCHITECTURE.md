# JWT Architecture - Clean Architecture & DDD

## Overview

O JWT foi reorganizado seguindo os princípios de **Arquitetura Limpa** e **Domain-Driven Design (DDD)**, com separação clara de responsabilidades e inversão de dependência.

## Estrutura da Arquitetura

```
┌─────────────────────────────────────────────────────────────┐
│                    INTERFACES (HTTP)                        │
├─────────────────────────────────────────────────────────────┤
│  src/interfaces/http/auth/extractor.rs                      │
│  - FromRequestParts<Arc<AppState>> for Claims               │
│  - Extração de token do header Authorization                │
│                                                             │
│  src/interfaces/http/error.rs                               │
│  - AuthError (InvalidToken)                                 │
│  - IntoResponse para erros de autenticação                 │
└─────────────────────────────────────────────────────────────┘
                                ↑
┌─────────────────────────────────────────────────────────────┐
│                    APPLICATION                              │
├─────────────────────────────────────────────────────────────┤
│  src/application/services/jwt_service.rs                    │
│  - JwtService (orquestra operações JWT)                    │
│  - Delega para JwtRepository (inversão de dependência)     │
└─────────────────────────────────────────────────────────────┘
                                ↑
┌─────────────────────────────────────────────────────────────┐
│                      DOMAIN                                 │
├─────────────────────────────────────────────────────────────┤
│  src/domain/jwt.rs                                          │
│  - JwtRepository trait (interface)                         │
│  - Claims struct (entidade de domínio)                     │
│  - Regras de negócio JWT                                   │
└─────────────────────────────────────────────────────────────┘
                                ↑
┌─────────────────────────────────────────────────────────────┐
│                  INFRASTRUCTURE                             │
├─────────────────────────────────────────────────────────────┤
│  src/infrastructure/repositories/jwt.rs                     │
│  - JwtRepositoryImpl (implementação concreta)              │
│  - Usa jsonwebtoken crate                                  │
│  - Configuração de secret e expiration                     │
└─────────────────────────────────────────────────────────────┘
```

## Componentes

### 1. **Domain Layer** (`src/domain/jwt.rs`)
```rust
// Interface do repositório JWT
#[async_trait]
pub trait JwtRepository: Send + Sync {
    async fn generate_token(&self, user_id: Uuid, user_role: Role) -> Result<String>;
    async fn decode_token(&self, token: &str) -> Result<Claims>;
}

// Entidade de domínio
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub role: Role,
}
```

### 2. **Infrastructure Layer** (`src/infrastructure/repositories/jwt.rs`)
```rust
// Implementação concreta
pub struct JwtRepositoryImpl {
    secret: String,
    expiration_duration: Duration,
}

#[async_trait]
impl JwtRepository for JwtRepositoryImpl {
    // Implementação usando jsonwebtoken crate
}
```

### 3. **Application Layer** (`src/application/services/jwt_service.rs`)
```rust
// Serviço de aplicação
pub struct JwtService {
    jwt_repository: Arc<dyn JwtRepository>,
}

impl JwtService {
    pub async fn generate_token(&self, user_id: Uuid, user_role: Role) -> Result<String> {
        self.jwt_repository.generate_token(user_id, user_role).await
    }
}
```

### 4. **Interface Layer** (`src/interfaces/http/`)
```rust
// Extractor para handlers
impl FromRequestParts<Arc<AppState>> for Claims {
    // Extrai token do header e decodifica via JwtService
}

// Erro de autenticação
pub enum AuthError {
    InvalidToken,
}
```

## Fluxo de Uso

### 1. **Login (Geração de Token)**
```
UserService.login() 
    → JwtService.generate_token() 
    → JwtRepositoryImpl.generate_token()
    → jsonwebtoken::encode()
```

### 2. **Autenticação (Decodificação de Token)**
```
Handler recebe Claims 
    → FromRequestParts<Arc<AppState>> 
    → JwtService.decode_token() 
    → JwtRepositoryImpl.decode_token()
    → jsonwebtoken::decode()
```

## Benefícios da Nova Arquitetura

### ✅ **Separação de Responsabilidades**
- **Domain**: Regras de negócio e interfaces
- **Infrastructure**: Implementação técnica (jsonwebtoken)
- **Application**: Orquestração de operações
- **Interface**: Adaptação para HTTP

### ✅ **Inversão de Dependência**
- `JwtService` depende da trait `JwtRepository` (abstração)
- `JwtRepositoryImpl` implementa a trait (concretização)
- Domain não conhece Infrastructure

### ✅ **Testabilidade**
- Pode mockar `JwtRepository` para testes unitários
- Testes isolados por camada
- Fácil substituição de implementações

### ✅ **Flexibilidade**
- Pode trocar implementação JWT sem afetar outras camadas
- Pode adicionar novas implementações (Redis, etc.)
- Configuração centralizada

### ✅ **Manutenibilidade**
- Código organizado por responsabilidade
- Fácil localização de funcionalidades
- Baixo acoplamento entre camadas

## Como Usar

### Em Handlers
```rust
use crate::domain::jwt::Claims;

async fn protected_route(user: Claims) -> Result<Json<UserResponse>> {
    // user.sub contém o ID do usuário autenticado
    // user.role contém o papel do usuário
}
```

### Em Services
```rust
// JwtService já injetado via AppState
let token = self.jwt_service.generate_token(user_id, user_role).await?;
```

## Configuração

O JWT é configurado no setup da aplicação:
```rust
let jwt_repo = JwtRepositoryImpl::new(
    config.jwt_secret.clone(),
    config.jwt_expiration_duration,
);
let jwt_service = JwtService::new(Arc::new(jwt_repo));
```

## Próximos Passos

1. **Testes Unitários**: Mockar `JwtRepository` para testar `JwtService`
2. **Testes de Integração**: Testar fluxo completo de autenticação
3. **Refresh Tokens**: Implementar renovação de tokens
4. **Blacklisting**: Implementar invalidação de tokens
5. **Rate Limiting**: Proteger endpoints de autenticação
