# JOS - Setup Guide

## Pré-requisitos

- Rust 1.85+
- PostgreSQL 16+
- Docker (opcional)

## Configuração do Ambiente

### 1. Variáveis de Ambiente

Crie um arquivo `.env` na raiz do projeto com as seguintes variáveis:

```env
# Database Configuration
DATABASE_URL=postgres://username:password@localhost:5432/jos_db

# Server Configuration
PORT=3000

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-should-be-at-least-32-characters-long

# Optional: Logging Level (DEBUG, INFO, WARN, ERROR)
# RUST_LOG=INFO
```

### 2. Configuração do Banco de Dados

#### Opção A: PostgreSQL Local

1. Instale PostgreSQL 16+
2. Crie um banco de dados:
   ```sql
   CREATE DATABASE jos_db;
   ```
3. Configure a variável `DATABASE_URL` no `.env`

#### Opção B: Docker

```bash
# Inicie o PostgreSQL com Docker
docker run --name jos-postgres \
  -e POSTGRES_DB=jos_db \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -p 5432:5432 \
  -d postgres:16-alpine
```

### 3. Instalação de Dependências

```bash
# Instale o sqlx-cli para migrações
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

## Executando o Projeto

### 1. Desenvolvimento

```bash
# Execute as migrações
sqlx migrate run

# Execute o projeto
cargo run
```

### 2. Produção

```bash
# Build de release
cargo build --release

# Execute
./target/release/jos
```

### 3. Docker

```bash
# Build da imagem
docker build -t jos-api .

# Execute com docker-compose
docker-compose up
```

## Verificações de Setup

O sistema agora inclui verificações automáticas durante o startup:

- ✅ **Validação de Variáveis de Ambiente**: Verifica se todas as variáveis obrigatórias estão presentes
- ✅ **Validação de Configuração**: Valida formato da URL do banco, porta, etc.
- ✅ **Health Check do Banco**: Testa a conectividade com o banco de dados
- ✅ **Execução de Migrações**: Executa automaticamente as migrações pendentes
- ✅ **Inicialização de Serviços**: Inicializa todos os serviços e repositórios

## Logs e Debugging

O sistema agora fornece logs detalhados durante o startup:

```
🔧 Initializing application setup...
📝 Logging system initialized
✅ Environment variables loaded from .env file
✅ Environment validation passed
✅ Configuration validation passed
🚀 Starting JOS (Join Our Session) API
📊 Environment: Development
🌐 Server will bind to: 127.0.0.1:3000
🗄️  Database: localhost:5432/jos_db
🔐 JWT expiration: 1 days
🔌 Establishing database connection...
✅ Database connection established
✅ Database health check passed
🔄 Running database migrations...
✅ Database migrations completed
🏗️  Initializing services...
✅ User service initialized
✅ Table service initialized
✅ Table request service initialized
✅ Application state initialized
🎉 Application setup completed successfully!
🚀 Launching HTTP server...
✅ Server bound to: 127.0.0.1:3000
🌐 API documentation available at: http://127.0.0.1:3000/docs
🔍 Health check available at: http://127.0.0.1:3000/health
```

## Endpoints Úteis

- **API Documentation**: `http://localhost:3000/docs`
- **Health Check**: `http://localhost:3000/health`
- **API Base**: `http://localhost:3000/v1`

## Troubleshooting

### Diagnóstico Automático

Para diagnosticar problemas automaticamente, execute:

```bash
cargo run --bin diagnose
```

Este comando irá:
- Verificar se todas as variáveis de ambiente estão configuradas
- Testar a conectividade com PostgreSQL
- Verificar permissões do usuário do banco
- Testar a conexão com SQLx
- Executar migrações de teste
- Fornecer soluções específicas para problemas encontrados

### Erros Comuns e Soluções

#### ❌ Erro: "password authentication failed for user"

**Causa**: Senha incorreta ou usuário inexistente no PostgreSQL.

**Soluções**:
```bash
# Opção 1: Usar Docker (recomendado)
docker run --name jos-postgres \
  -e POSTGRES_DB=jos_db \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -p 5432:5432 \
  -d postgres:16-alpine

# Opção 2: Corrigir usuário local
sudo -u postgres psql
CREATE USER postgres WITH PASSWORD 'postgres';
GRANT ALL PRIVILEGES ON DATABASE jos_db TO postgres;
```

#### ❌ Erro: "Missing required environment variables"

**Causa**: Arquivo `.env` não existe ou variáveis não estão definidas.

**Solução**:
```bash
# Criar .env automaticamente
cargo run -p jos-cli setup

# Ou criar manualmente
cat > .env << EOF
DATABASE_URL=postgres://postgres:postgres@localhost:5432/jos_db
PORT=3000
JWT_SECRET=your-super-secret-jwt-key-should-be-at-least-32-characters-long
EOF
```

#### ❌ Erro: "Database health check failed"

**Causa**: Problemas de conectividade com o banco de dados.

**Soluções**:
```bash
# Verificar se PostgreSQL está rodando
pg_isready -h localhost -p 5432

# Testar conexão manual
psql postgres://postgres:postgres@localhost:5432/jos_db -c "SELECT 1;"

# Executar diagnóstico completo
cargo run --bin diagnose

#### ❌ Erro: "Failed to bind address"

**Causa**: Porta já está em uso ou inválida.

**Soluções**:
```bash
# Verificar o que está usando a porta
lsof -i :3000

# Usar porta diferente
echo "PORT=3001" >> .env

# Parar processo que está usando a porta
sudo kill -9 $(lsof -t -i:3000)
```

#### ❌ Erro: "Failed to run database migrations"

**Causa**: Problemas de permissões ou banco inexistente.

**Soluções**:
```bash
# Criar banco de dados
sudo -u postgres createdb jos_db

# Conceder permissões
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE jos_db TO postgres;"

# Executar migrações manualmente
sqlx migrate run
```

### Mensagens de Erro Melhoradas

O sistema agora fornece mensagens de erro detalhadas com soluções específicas:

```
❌ Database connection failed: error returned from database: password authentication failed for user "postgres"

🔐 Authentication Error:
• Check your DATABASE_URL in .env file
• Verify username and password are correct
• Example: postgres://username:password@localhost:5432/db_name
• Make sure PostgreSQL is running
• Try: docker run --name jos-postgres -e POSTGRES_DB=jos_db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:16-alpine

💡 Troubleshooting:
• Run 'cargo run -p jos-cli setup' to check your environment
• Verify DATABASE_URL format: postgres://user:pass@host:port/db
• Check PostgreSQL logs for more details
• Ensure firewall allows connections to PostgreSQL port
```
