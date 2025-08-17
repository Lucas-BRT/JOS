# Testando a Implementação OpenAPI

Este documento descreve como testar a implementação OpenAPI da aplicação JOS.

## Pré-requisitos

1. Aplicação JOS rodando localmente
2. Navegador web
3. Ferramentas de teste de API (opcional): curl, Postman, Insomnia

## Iniciando a Aplicação

```bash
# Compilar e executar a aplicação
cargo run

# Ou usando docker-compose
docker-compose up
```

A aplicação estará disponível em `http://localhost:3000`

## Acessando a Documentação

### Swagger UI

Acesse a interface Swagger UI em:
```
http://localhost:3000/docs
```

### Especificação OpenAPI

A especificação OpenAPI em formato JSON está disponível em:
```
http://localhost:3000/api-docs/openapi.json
```

## Testando Endpoints

### 1. Health Check

```bash
curl -X GET http://localhost:3000/health
```

**Resposta esperada:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T12:00:00Z",
  "service": "JOS API",
  "version": "0.1.0"
}
```

### 2. Autenticação

#### Criar conta

```bash
curl -X POST http://localhost:3000/v1/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test User",
    "email": "test@example.com",
    "password": "password123",
    "confirm_password": "password123"
  }'
```

#### Login

```bash
curl -X POST http://localhost:3000/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }'
```

#### Obter requisitos de senha

```bash
curl -X GET http://localhost:3000/v1/auth/password-requirements
```

### 3. Usuários

#### Obter usuário atual (requer autenticação)

```bash
curl -X GET http://localhost:3000/v1/users/me \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### 4. Tabelas

#### Listar tabelas disponíveis

```bash
curl -X GET http://localhost:3000/v1/tables
```

#### Criar tabela (requer autenticação)

```bash
curl -X POST http://localhost:3000/v1/tables \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "title": "D&D Campaign",
    "description": "A epic fantasy campaign in the Forgotten Realms",
    "game_system_id": "550e8400-e29b-41d4-a716-446655440000",
    "is_public": true,
    "max_players": 6
  }'
```

### 5. Solicitações de Tabela

#### Listar solicitações (requer autenticação)

```bash
curl -X GET http://localhost:3000/v1/table-requests \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

#### Criar solicitação (requer autenticação)

```bash
curl -X POST http://localhost:3000/v1/table-requests \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "table_id": "550e8400-e29b-41d4-a716-446655440000",
    "message": "I would like to join this table"
  }'
```

#### Obter solicitação específica

```bash
curl -X GET http://localhost:3000/v1/table-requests/REQUEST_ID \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

#### Atualizar solicitação

```bash
curl -X PUT http://localhost:3000/v1/table-requests/REQUEST_ID \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "status": "approved"
  }'
```

#### Deletar solicitação

```bash
curl -X DELETE http://localhost:3000/v1/table-requests/REQUEST_ID \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Validação da Documentação

### 1. Verificar Schemas

No Swagger UI, verifique se todos os schemas estão corretamente definidos:

- [ ] SignupDto
- [ ] LoginDto
- [ ] UserSignupResponse
- [ ] CreateTableDto
- [ ] AvailableTableResponse
- [ ] CreateTableRequestDto
- [ ] TableRequestResponse
- [ ] ErrorResponse

### 2. Verificar Endpoints

Confirme que todos os endpoints estão documentados:

#### Auth
- [ ] POST /v1/auth/signup
- [ ] POST /v1/auth/login
- [ ] GET /v1/auth/password-requirements

#### Users
- [ ] GET /v1/users/me

#### Tables
- [ ] GET /v1/tables
- [ ] POST /v1/tables

#### Table Requests
- [ ] GET /v1/table-requests
- [ ] POST /v1/table-requests
- [ ] GET /v1/table-requests/{id}
- [ ] PUT /v1/table-requests/{id}
- [ ] DELETE /v1/table-requests/{id}

#### Health
- [ ] GET /health

### 3. Verificar Segurança

Confirme que a autenticação JWT está configurada:

- [ ] Bearer token authentication scheme
- [ ] Endpoints protegidos marcados corretamente
- [ ] Exemplos de token fornecidos

### 4. Verificar Tags

Confirme que os endpoints estão organizados em tags:

- [ ] auth
- [ ] users
- [ ] tables
- [ ] table-requests
- [ ] health

## Testes Automatizados

### Usando curl em script

```bash
#!/bin/bash

BASE_URL="http://localhost:3000"

echo "Testing Health Check..."
curl -s -X GET "$BASE_URL/health" | jq '.'

echo "Testing Password Requirements..."
curl -s -X GET "$BASE_URL/v1/auth/password-requirements" | jq '.'

echo "Testing Signup..."
SIGNUP_RESPONSE=$(curl -s -X POST "$BASE_URL/v1/auth/signup" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test User",
    "email": "test@example.com",
    "password": "password123",
    "confirm_password": "password123"
  }')

echo "$SIGNUP_RESPONSE" | jq '.'

echo "Testing Login..."
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/v1/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }')

TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.token')
echo "Token: $TOKEN"

echo "Testing Get Current User..."
curl -s -X GET "$BASE_URL/v1/users/me" \
  -H "Authorization: Bearer $TOKEN" | jq '.'
```

### Usando Postman

1. Importe a especificação OpenAPI em `http://localhost:3000/api-docs/openapi.json`
2. Configure a variável de ambiente `baseUrl` como `http://localhost:3000`
3. Execute a coleção de testes

## Troubleshooting

### Problemas Comuns

1. **Erro de CORS**: Verifique se o servidor está configurado corretamente
2. **Token inválido**: Certifique-se de usar o token correto no header Authorization
3. **Validação falhou**: Verifique se os dados enviados seguem o schema definido
4. **Endpoint não encontrado**: Confirme se a URL está correta

### Logs

Monitore os logs da aplicação para identificar problemas:

```bash
# Se usando cargo
RUST_LOG=debug cargo run

# Se usando docker
docker-compose logs -f
```

## Próximos Passos

1. Implementar testes de integração automatizados
2. Adicionar mais exemplos na documentação
3. Configurar CI/CD para validar a documentação
4. Implementar rate limiting e outras proteções
