# JOS - Junta os Dados

Sistema de gerenciamento de mesas para RPG com arquitetura limpa e DDD.

## 🚀 Fluxo de Desenvolvimento

### 1. Configuração Inicial

```bash
# Clone o repositório
git clone "seu-repositorio"
cd JOS

# Copie o arquivo de ambiente (opcional)
cp env.example .env
# Edite as variáveis se necessário
```

### 2. Subir Infraestrutura

```bash
# Suba apenas os serviços de infraestrutura
docker compose up -d
```

Isso irá subir:
- **PostgreSQL** na porta 5432
- **Redis** na porta 6379  
- **pgAdmin** na porta 8080 (opcional)

### 3. Desenvolvimento Local

```bash
# Instale cargo-watch se não tiver
cargo install cargo-watch

# Execute com hot reload
cargo watch -x check -x test -x run
```

A aplicação estará disponível em: http://localhost:3000

## 🏗️ Arquitetura

### Estrutura de Arquivos

```
JOS/
├── docker-compose.yml          # Infraestrutura local
├── docker-compose.prod.yml     # Produção/CI-CD
├── Dockerfile                  # Build da aplicação
├── env.example                 # Variáveis de ambiente
├── src/                        # Código fonte
├── migrations/                 # Migrações do banco
└── scripts/                    # Scripts de inicialização
```

### Variáveis de Ambiente

#### Desenvolvimento Local
```bash
# Infraestrutura
POSTGRES_DB=jos_dev
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_PORT=5432
REDIS_PORT=6379

# Aplicação
DATABASE_URL=postgres://postgres:postgres@localhost:5432/jos_dev
REDIS_URL=redis://localhost:6379
PORT=3000
JWT_SECRET=dev-jwt-secret-key-32-chars-long
RUST_LOG=debug
```

#### Produção
```bash
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
JWT_SECRET=your-production-secret
RUST_LOG=info
```

## 🔄 Fluxo de Deploy

### 1. Desenvolvimento Local
```bash
# Infraestrutura
docker compose up -d

# Aplicação
cargo watch -x check -x test -x run
```

### 2. CI/CD - Teste Online
```bash
# Build e deploy com docker-compose.prod.yml
docker compose -f docker-compose.prod.yml up -d
```

### 3. Produção
```bash
# Deploy com variáveis de produção
docker compose -f docker-compose.prod.yml up -d
```

## 🛠️ Comandos Úteis

### Infraestrutura
```bash
# Subir infraestrutura
docker compose up -d

# Parar infraestrutura
docker compose down

# Ver logs
docker compose logs -f

# Acessar banco
docker compose exec db psql -U postgres -d jos_dev
```

### Desenvolvimento
```bash
# Hot reload
cargo watch -x check -x test -x run

# Testes
cargo test

# Migrações
cargo run --bin jos-cli migrate
```

### Produção
```bash
# Build e deploy
docker compose -f docker-compose.prod.yml up -d

# Logs de produção
docker compose -f docker-compose.prod.yml logs -f
```

## 📊 Serviços

### Desenvolvimento Local
- **API**: http://localhost:3000
- **PostgreSQL**: localhost:5432
- **Redis**: localhost:6379
- **pgAdmin**: http://localhost:8080

### Documentação
- **API Docs**: http://localhost:3000/docs
- **Health Check**: http://localhost:3000/health

## 🔧 Configuração

### Variáveis de Ambiente

Copie `env.example` para `.env` e configure:

```bash
# Infraestrutura
POSTGRES_DB=jos_dev
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_PORT=5432
REDIS_PORT=6379

# pgAdmin (opcional)
PGADMIN_EMAIL=admin@jos.local
PGADMIN_PASSWORD=admin
PGADMIN_PORT=8080

# Aplicação
DATABASE_URL=postgres://postgres:postgres@localhost:5432/jos_dev
REDIS_URL=redis://localhost:6379
PORT=3000
JWT_SECRET=dev-jwt-secret-key-32-chars-long
RUST_LOG=debug
RUST_BACKTRACE=1
```

## 🚨 Troubleshooting

### Problemas Comuns

#### 1. Porta já em uso
```bash
# Verificar o que está usando a porta
lsof -i :3000
lsof -i :5432

# Parar serviços conflitantes
sudo systemctl stop postgresql
```

#### 2. Banco não conecta
```bash
# Verificar se o container está rodando
docker compose ps

# Ver logs do banco
docker compose logs db
```

#### 3. Aplicação não inicia
```bash
# Verificar variáveis de ambiente
echo $DATABASE_URL

# Ver logs da aplicação
cargo run
```

## 📚 Documentação

- [Arquitetura JWT](docs/JWT_ARCHITECTURE.md)
- [Arquitetura Password](docs/PASSWORD_ARCHITECTURE.md)
- [Regras de Negócio Password](docs/PASSWORD_BUSINESS_RULES.md)

## 🎯 Próximos Passos

1. **Desenvolvimento**: `docker compose up -d && cargo watch -x run`
2. **Testes**: `cargo test`
3. **CI/CD**: Deploy automático com `docker-compose.prod.yml`
4. **Produção**: Deploy com variáveis de produção

Happy coding! 🎉
