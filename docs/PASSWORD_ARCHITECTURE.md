# Password Architecture - Clean Architecture & DDD

## Overview

O sistema de password foi reorganizado seguindo os princípios de **Arquitetura Limpa** e **Domain-Driven Design (DDD)**, com separação clara de responsabilidades e inversão de dependência, seguindo o mesmo padrão implementado para JWT.

## Estrutura da Arquitetura

```
┌─────────────────────────────────────────────────────────────┐
│                    APPLICATION                              │
├─────────────────────────────────────────────────────────────┤
│  src/application/services/password_service.rs               │
│  - PasswordService (orquestra operações de password)       │
│  - Delega para PasswordRepository (inversão de dependência)│
└─────────────────────────────────────────────────────────────┘
                                ↑
┌─────────────────────────────────────────────────────────────┐
│                      DOMAIN                                 │
├─────────────────────────────────────────────────────────────┤
│  src/domain/password.rs                                     │
│  - PasswordRepository trait (interface)                    │
│  - Regras de negócio para password                         │
└─────────────────────────────────────────────────────────────┘
                                ↑
┌─────────────────────────────────────────────────────────────┐
│                  INFRASTRUCTURE                             │
├─────────────────────────────────────────────────────────────┤
│  src/infrastructure/repositories/password.rs                │
│  - PasswordRepositoryImpl (implementação concreta)         │
│  - Usa Argon2 para hash e verificação                      │
│  - Operações não-bloqueantes com tokio::task::spawn        │
│  - Mutex compartilhado para instância Argon2               │
└─────────────────────────────────────────────────────────────┘
```

## Componentes

### 1. **Domain Layer** (`src/domain/password.rs`)
```rust
// Interface do repositório de password
#[async_trait]
pub trait PasswordRepository: Send + Sync {
    async fn generate_hash(&self, password: String) -> crate::Result<String>;
    async fn verify_hash(&self, password: String, hash: String) -> crate::Result<bool>;
}
```

### 2. **Infrastructure Layer** (`src/infrastructure/repositories/password.rs`)
```rust
// Implementação concreta usando Argon2 com operações não-bloqueantes
#[derive(Clone)]
pub struct PasswordRepositoryImpl {
    argon2: Arc<Mutex<Argon2<'static>>>,
}

#[async_trait]
impl PasswordRepository for PasswordRepositoryImpl {
    async fn generate_hash(&self, password: String) -> Result<String> {
        // Implementação não-bloqueante usando tokio::task::spawn
    }
    
    async fn verify_hash(&self, password: String, hash: String) -> Result<bool> {
        // Implementação não-bloqueante usando tokio::task::spawn
    }
}
```

### 3. **Application Layer** (`src/application/services/password_service.rs`)
```rust
// Serviço de aplicação
pub struct PasswordService {
    password_repository: Arc<dyn PasswordRepository>,
}

impl PasswordService {
    pub async fn generate_hash(&self, password: String) -> Result<String> {
        self.password_repository.generate_hash(password).await
    }
    
    pub async fn verify_hash(&self, password: String, hash: String) -> Result<bool> {
        self.password_repository.verify_hash(password, hash).await
    }
}
```

## Fluxo de Uso

### 1. **Geração de Hash (Signup)**
```
UserService.signup() 
    → PasswordService.generate_hash() 
    → PasswordRepositoryImpl.generate_hash()
    → tokio::task::spawn() → Argon2::hash_password()
```

### 2. **Verificação de Hash (Login)**
```
UserService.login() 
    → PasswordService.verify_hash() 
    → PasswordRepositoryImpl.verify_hash()
    → tokio::task::spawn() → Argon2::verify_password()
```

## Características Técnicas

### ✅ **Performance Não-Bloqueante**
- **tokio::task::spawn**: Operações em tasks separadas
- **Servidor responsivo**: Continua atendendo requisições durante hash
- **Concorrência**: Múltiplas operações de hash simultâneas
- **Mutex compartilhado**: Instância Argon2 reutilizada entre operações

### ✅ **Segurança**
- **Argon2**: Algoritmo de hash recomendado para passwords
- **Salt único**: Cada password tem um salt gerado aleatoriamente
- **Thread-safe**: Mutex protege acesso à instância Argon2

### ✅ **Testabilidade**
- **Mockável**: Pode mockar `PasswordRepository` para testes
- **Testes isolados**: Testes unitários para cada camada
- **Testes de concorrência**: Valida operações simultâneas

## Como Usar

### Em Services
```rust
// PasswordService já injetado via AppState
let hash = self.password_service.generate_hash(password).await?;
let is_valid = self.password_service.verify_hash(password, hash).await?;
```

### Setup e Injeção
```rust
let password_repo = PasswordRepositoryImpl::new();
let password_service = PasswordService::new(Arc::new(password_repo));
let user_service = UserService::new(
    Arc::new(user_repo), 
    jwt_service, 
    password_service
);
```

## Testes

Os testes incluem validação de operações concorrentes:

```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_generate_hash() {
        let password_repo = PasswordRepositoryImpl::new();
        let hash = password_repo.generate_hash("password123".to_string()).await.unwrap();
        assert!(hash.starts_with("$argon2id$"));
    }
    
    #[tokio::test]
    async fn test_concurrent_hash_operations() {
        let password_repo = PasswordRepositoryImpl::new();
        
        // Spawn multiple concurrent hash operations
        let handles: Vec<_> = (0..5)
            .map(|_| {
                let repo = password_repo.clone();
                tokio::spawn(async move {
                    repo.generate_hash("password123".to_string()).await
                })
            })
            .collect();

        // All operations complete successfully
        for handle in handles {
            let result = handle.await.unwrap();
            let hash = result.unwrap();
            assert!(hash.starts_with("$argon2id$"));
        }
    }
}
```

## Benefícios da Nova Arquitetura

### ✅ **Performance**
- **Não-bloqueante**: Servidor continua atendendo durante hash
- **Concorrência**: Múltiplas operações simultâneas
- **Eficiência**: Reutilização de instância Argon2
- **Responsividade**: Melhor experiência do usuário

### ✅ **Separação de Responsabilidades**
- **Domain**: Interface e regras de negócio
- **Infrastructure**: Implementação técnica (Argon2)
- **Application**: Orquestração de operações

### ✅ **Inversão de Dependência**
- `PasswordService` depende da trait `PasswordRepository` (abstração)
- `PasswordRepositoryImpl` implementa a trait (concretização)
- Domain não conhece Infrastructure

### ✅ **Testabilidade**
- Pode mockar `PasswordRepository` para testes unitários
- Testes isolados por camada
- Testes de concorrência incluídos

### ✅ **Flexibilidade**
- Pode trocar implementação de hash sem afetar outras camadas
- Pode adicionar novas implementações (bcrypt, scrypt, etc.)
- Configuração centralizada

### ✅ **Manutenibilidade**
- Código organizado por responsabilidade
- Fácil localização de funcionalidades
- Baixo acoplamento entre camadas

## Migração de Utils

### ❌ **Antes (Utils - Bloqueante)**
```rust
// src/utils/password.rs
pub async fn generate_hash(password: String) -> Result<String> {
    tokio::task::spawn_blocking(move || {
        // Operação bloqueante
    }).await??
}
```

### ✅ **Depois (Clean Architecture - Não-bloqueante)**
```rust
// Domain
trait PasswordRepository { ... }

// Infrastructure  
impl PasswordRepository for PasswordRepositoryImpl {
    async fn generate_hash(&self, password: String) -> Result<String> {
        tokio::task::spawn(async move {
            // Operação não-bloqueante
        }).await?
    }
}

// Application
struct PasswordService { ... }
```

## Próximos Passos

1. **Testes Unitários**: Mockar `PasswordRepository` para testar `PasswordService`
2. **Testes de Integração**: Testar fluxo completo de signup/login
3. **Configuração**: Adicionar parâmetros configuráveis do Argon2
4. **Rate Limiting**: Proteger endpoints de autenticação
5. **Auditoria**: Logs de tentativas de login
6. **Métricas**: Monitorar performance das operações de hash
