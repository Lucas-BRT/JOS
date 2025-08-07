# Exemplos de Mensagens de Erro Melhoradas

O sistema agora fornece mensagens de erro detalhadas e úteis que ajudam a resolver problemas rapidamente.

## 🔐 Erro de Autenticação do Banco de Dados

**Erro Original:**
```
Error: Setup(FailedToEstablishDatabaseConnection("error returned from database: password authentication failed for user \"postgres\""))
```

**Nova Mensagem Melhorada:**
```
❌ Database connection failed: error returned from database: password authentication failed for user "postgres"

🔐 Authentication Error:
• Check your DATABASE_URL in .env file
• Verify username and password are correct
• Example: postgres://username:password@localhost:5432/db_name
• Make sure PostgreSQL is running
• Try: docker run --name jos-postgres -e POSTGRES_DB=jos_db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:16-alpine

💡 Troubleshooting:
• Run './scripts/setup.sh' to check your environment
• Verify DATABASE_URL format: postgres://user:pass@host:port/db
• Check PostgreSQL logs for more details
• Ensure firewall allows connections to PostgreSQL port
```

## 🔌 Erro de Conexão Recusada

**Erro Original:**
```
Error: Setup(FailedToEstablishDatabaseConnection("connection to server at \"localhost\" (127.0.0.1), port 5432 failed: Connection refused"))
```

**Nova Mensagem Melhorada:**
```
❌ Database connection failed: connection to server at "localhost" (127.0.0.1), port 5432 failed: Connection refused

🔌 Connection Error:
• PostgreSQL is not running
• Check if PostgreSQL is started
• Verify the port in DATABASE_URL (default: 5432)
• Try: docker run --name jos-postgres -e POSTGRES_DB=jos_db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:16-alpine

💡 Troubleshooting:
• Run './scripts/setup.sh' to check your environment
• Verify DATABASE_URL format: postgres://user:pass@host:port/db
• Check PostgreSQL logs for more details
• Ensure firewall allows connections to PostgreSQL port
```

## 🗄️ Erro de Banco Inexistente

**Erro Original:**
```
Error: Setup(FailedToEstablishDatabaseConnection("database \"jos_db\" does not exist"))
```

**Nova Mensagem Melhorada:**
```
❌ Database connection failed: database "jos_db" does not exist

🗄️ Database Error:
• Database does not exist
• Create the database: CREATE DATABASE jos_db;
• Or use Docker: docker run --name jos-postgres -e POSTGRES_DB=jos_db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:16-alpine

💡 Troubleshooting:
• Run './scripts/setup.sh' to check your environment
• Verify DATABASE_URL format: postgres://user:pass@host:port/db
• Check PostgreSQL logs for more details
• Ensure firewall allows connections to PostgreSQL port
```

## 📝 Erro de Variável de Ambiente Ausente

**Erro Original:**
```
Error: Setup(EnvironmentValidationFailed("Missing required environment variables: DATABASE_URL, PORT"))
```

**Nova Mensagem Melhorada:**
```
❌ Environment validation failed: Missing required environment variables: DATABASE_URL, PORT

💡 Solution:
• Check if .env file exists in project root
• Verify all required variables are set
• Run './scripts/setup.sh' to create .env template
• Required variables: DATABASE_URL, PORT, JWT_SECRET
• See docs/SETUP.md for configuration guide
```

## 🔌 Erro de Porta em Uso

**Erro Original:**
```
Error: Setup(FailedToBindAddress("Address already in use (os error 98)"))
```

**Nova Mensagem Melhorada:**
```
❌ Failed to bind server to address: Address already in use (os error 98)

💡 Solution:
• Port might be in use by another application
• Check if port 3000 is available
• Try a different port in your .env file
• Example: PORT=3001
• Use 'lsof -i :3000' to see what's using the port
```

## 🔧 Erro de Configuração Inválida

**Erro Original:**
```
Error: Setup(InvalidConfiguration("DATABASE_URL must start with 'postgres://' or 'postgresql://'"))
```

**Nova Mensagem Melhorada:**
```
❌ Invalid configuration: DATABASE_URL must start with 'postgres://' or 'postgresql://'

💡 Solution:
• Check your .env file for correct values
• Verify all required variables are set
• Run './scripts/setup.sh' to validate your setup
• See docs/SETUP.md for configuration examples
```

## 🛠️ Como Usar os Scripts de Diagnóstico

### Diagnóstico Automático
```bash
./scripts/diagnose.sh
```

**Exemplo de Saída:**
```
🔍 JOS Database Diagnosis
=========================
[INFO] Checking environment variables...
[SUCCESS] DATABASE_URL is set
  URL: postgres://postgres:postgres@localhost:5432/jos_db
[SUCCESS] PORT is set to 3000
[SUCCESS] JWT_SECRET is set
[INFO] Checking PostgreSQL connection...
[SUCCESS] PostgreSQL is running on localhost:5432
[INFO] Testing database connection...
[SUCCESS] Database connection successful!
[SUCCESS] Database 'jos_db' exists
[INFO] Checking user permissions...
[SUCCESS] User has CREATE/DROP permissions
[INFO] Testing SQLx CLI...
[SUCCESS] sqlx-cli is installed
[INFO] Testing SQLx connection...
[SUCCESS] SQLx can connect to database
[INFO] Checking for existing migrations...
[SUCCESS] Migrations directory exists with files
  Found 3 migration files
[INFO] Testing migrations...
[SUCCESS] Migrations applied successfully

🎯 Diagnosis Summary:
=====================
✅ Database connection is working
You should be able to run: cargo run

💡 Next steps:
1. Fix any issues identified above
2. Run: cargo run
3. If successful, access: http://localhost:3000
4. Check health: http://localhost:3000/health
```

### Setup Automatizado
```bash
./scripts/setup.sh
```

**Exemplo de Saída:**
```
🚀 JOS (Join Our Session) - Setup Script
==========================================
[WARNING] .env file not found. Creating from template...
[SUCCESS] .env file created successfully!
[WARNING] Please review and update the .env file with your actual configuration.
[SUCCESS] Rust is installed
[SUCCESS] sqlx-cli is already installed
[SUCCESS] PostgreSQL is running
[INFO] Testing database connection...
[SUCCESS] Database connection successful
[INFO] Building the project...
[SUCCESS] Project built successfully
[INFO] Running database migrations...
[SUCCESS] Database migrations completed successfully

🎉 Setup completed!

Next steps:
1. Review and update your .env file if needed
2. Start the application: cargo run
3. Access the API at: http://localhost:3000
4. View API docs at: http://localhost:3000/docs
5. Check health at: http://localhost:3000/health

For more information, see docs/SETUP.md
```

## 🎯 Benefícios das Melhorias

1. **Mensagens Claras**: Erros são explicados em linguagem simples
2. **Soluções Específicas**: Cada erro inclui passos para resolver
3. **Diagnóstico Automático**: Scripts identificam problemas rapidamente
4. **Exemplos Práticos**: Comandos específicos para resolver problemas
5. **Documentação Integrada**: Links para guias detalhados
6. **Validação Precoce**: Problemas são detectados antes da aplicação subir

## 🔧 Scripts Disponíveis

- **`./scripts/setup.sh`**: Setup automatizado do ambiente
- **`./scripts/diagnose.sh`**: Diagnóstico completo de problemas
- **`cargo run`**: Execução com mensagens de erro melhoradas
