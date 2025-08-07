# JOS (Join Our Session)

Uma API para gerenciar mesas de RPG e sessões.

## 🚀 Início Rápido

### 🎯 Setup Automatizado (Recomendado)

Para uma experiência de desenvolvimento completa e automatizada:

```bash
# Setup completo com Docker
cargo run -p jos-cli setup
```

### 🛠️ Comandos do CLI

```bash
# Setup completo
cargo run -p jos-cli setup

# Gerenciar serviços
cargo run -p jos-cli start      # Iniciar serviços
cargo run -p jos-cli stop       # Parar serviços
cargo run -p jos-cli restart    # Reiniciar serviços

# Monitoramento
cargo run -p jos-cli status     # Status dos serviços
cargo run -p jos-cli logs       # Ver logs de todos os serviços
cargo run -p jos-cli logs api   # Ver logs apenas da API
cargo run -p jos-cli logs db    # Ver logs apenas do banco

# Banco de dados
cargo run -p jos-cli migrate    # Executar migrações
cargo run -p jos-cli reset-db   # Reset completo do banco
cargo run -p jos-cli shell-db   # Shell do PostgreSQL
cargo run -p jos-cli shell-redis # Shell do Redis

# Desenvolvimento
cargo run -p jos-cli build      # Build do projeto
cargo run -p jos-cli test       # Executar testes
cargo run -p jos-cli clean      # Limpar build artifacts

# Diagnóstico
cargo run -p jos-cli diagnose   # Diagnóstico completo do sistema
```

### 📋 Setup Manual

1. **Clone o repositório**
   ```bash
   git clone <repository-url>
   cd JOS
   ```

2. **Configure as variáveis de ambiente**
   ```bash
   cp env.example .env
   # Edite o arquivo .env com suas configurações
   ```

3. **Inicie os serviços com Docker**
   ```bash
   docker-compose up -d db redis
   ```

4. **Instale dependências**
   ```bash
   cargo install sqlx-cli --no-default-features --features rustls,postgres
   cargo install cargo-watch  # Para hot reload
   ```

5. **Execute as migrações**
   ```bash
   sqlx migrate run
   ```

6. **Execute o projeto**
   ```bash
   cargo run
   ```

## 🚀 Ambiente de Desenvolvimento Melhorado

### 🐳 Docker Compose Completo

O projeto agora inclui um ambiente Docker completo com:

- **PostgreSQL 16**: Banco de dados principal
- **Redis 7**: Cache e sessões
- **pgAdmin**: Interface web para gerenciar o banco
- **Volumes persistentes**: Dados preservados entre reinicializações
- **Health checks**: Verificação automática de saúde dos serviços

### 🛠️ CLI Rust Melhorado

O CLI Rust agora inclui funcionalidades completas para gerenciar o ambiente de desenvolvimento:

#### Setup Automatizado
```bash
cargo run -p jos-cli setup
```

**Funcionalidades:**
- ✅ Verificação de Docker e Docker Compose
- ✅ Criação automática do arquivo `.env`
- ✅ Inicialização dos serviços com health checks
- ✅ Instalação de dependências Rust (sqlx-cli, cargo-watch)
- ✅ Execução automática de migrações
- ✅ Build e testes do projeto
- ✅ Logs coloridos e informativos

#### Comandos de Serviços
```bash
# Gerenciar serviços Docker
cargo run -p jos-cli start      # Iniciar todos os serviços
cargo run -p jos-cli stop       # Parar todos os serviços
cargo run -p jos-cli restart    # Reiniciar todos os serviços

# Monitoramento
cargo run -p jos-cli status     # Status e saúde dos serviços
cargo run -p jos-cli logs       # Logs de todos os serviços
cargo run -p jos-cli logs api   # Logs apenas da API
cargo run -p jos-cli logs db    # Logs apenas do banco
cargo run -p jos-cli logs redis # Logs apenas do Redis
```

#### Comandos de Banco de Dados
```bash
# Gerenciar banco de dados
cargo run -p jos-cli migrate    # Executar migrações
cargo run -p jos-cli reset-db   # Reset completo do banco
cargo run -p jos-cli shell-db   # Abrir shell do PostgreSQL
cargo run -p jos-cli shell-redis # Abrir shell do Redis
```

#### Comandos de Desenvolvimento
```bash
# Desenvolvimento
cargo run -p jos-cli build      # Build do projeto
cargo run -p jos-cli test       # Executar testes
cargo run -p jos-cli clean      # Limpar build artifacts
cargo run -p jos-cli diagnose   # Diagnóstico completo do sistema
```

### 🔍 Sistema de Diagnóstico

O sistema fornece logs detalhados durante o startup:

```
🚀 JOS Setup Tool
==================
🐳 Checking Docker installation...
✅ Docker is installed
✅ Docker is running
🐳 Checking Docker Compose...
✅ Docker Compose is available
📝 Setting up environment variables...
✅ .env file created successfully!
🚀 Starting development services...
✅ Services started successfully
⏳ Waiting for database to be ready...
✅ Database is ready
⏳ Waiting for Redis to be ready...
✅ Redis is ready
🔧 Installing Rust dependencies...
✅ sqlx-cli is already installed
✅ cargo-watch is already installed
🔨 Building project...
✅ Project built successfully
🗄️ Running database migrations...
✅ Database migrations completed
🔍 Running system diagnosis...
✅ System diagnosis passed

🎉 Setup completed successfully!
==================================

📊 Services Status:
  • Database: http://localhost:5432 (postgres/postgres)
  • Redis: http://localhost:6379
  • pgAdmin: http://localhost:8080 (admin@jos.local/admin)

🚀 Available Commands:
  • Start API: cargo run
  • Hot reload: cargo watch -x run
  • Run tests: cargo test
  • Database migrations: sqlx migrate run
  • Stop services: docker-compose down
  • View logs: docker-compose logs -f

📚 Useful URLs:
  • API: http://localhost:3000
  • API Docs: http://localhost:3000/docs
  • Health Check: http://localhost:3000/health

🔧 Development Tips:
  • Use 'cargo watch -x run' for automatic reloading
  • Check logs: docker-compose logs -f
  • Reset database: docker-compose down -v && docker-compose up -d
```

### 🔍 Endpoints de Monitoramento
- **Health Check**: `GET /health` - Status da aplicação
- **API Documentation**: `GET /docs` - Documentação interativa
- **OpenAPI Spec**: `GET /api-docs/openapi.json` - Especificação OpenAPI

### 🛠️ Ferramentas de Diagnóstico
- **Diagnóstico Completo**: `cargo run -p jos-cli diagnose` - Verifica todo o sistema
- **Setup Automatizado**: `cargo run -p jos-cli setup` - Configura o ambiente

## 📚 Documentação

- [Guia de Setup Detalhado](docs/SETUP.md)
- [Documentação da API](docs/API_DOCUMENTATION.md)

## 🛠️ Tecnologias

- **Rust** - Linguagem principal
- **Axum** - Framework web
- **SQLx** - ORM para PostgreSQL
- **JWT** - Autenticação
- **OpenAPI** - Documentação da API

## 📝 Variáveis de Ambiente

| Variável | Descrição | Exemplo |
|----------|-----------|---------|
| `DATABASE_URL` | URL de conexão com PostgreSQL | `postgres://user:pass@localhost:5432/jos_db` |
| `PORT` | Porta do servidor | `3000` |
| `JWT_SECRET` | Chave secreta para JWT | `your-super-secret-key-32-chars` |

## 🐛 Troubleshooting

### Erro: "Missing required environment variables"
- Verifique se o arquivo `.env` existe e contém todas as variáveis obrigatórias

### Erro: "Database health check failed"
- Verifique se o PostgreSQL está rodando
- Verifique se a `DATABASE_URL` está correta

### Erro: "Failed to bind address"
- Verifique se a porta especificada não está em uso
- Verifique se a porta está no range válido (1024-65535)

Para mais informações, consulte [docs/SETUP.md](docs/SETUP.md).

## 🤝 Contribuindo

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.
