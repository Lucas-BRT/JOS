# JOS - Sistema de Gerenciamento de Mesas de RPG

## Arquitetura Hexagonal (Ports and Adapters)

Este projeto implementa a arquitetura hexagonal, também conhecida como Ports and Adapters, que separa a lógica de negócio da infraestrutura técnica.

## Estrutura do Projeto

```
src/
├── domain/                    # Camada de Domínio (Core Business Logic)
│   ├── entities/             # Entidades de negócio
│   │   ├── user.rs          # Usuário do sistema
│   │   ├── table.rs         # Mesa de RPG
│   │   ├── session.rs       # Sessão de jogo
│   │   ├── game_system.rs   # Sistema de jogo (D&D, Pathfinder, etc.)
│   │   ├── table_member.rs  # Membro de uma mesa
│   │   ├── session_intent.rs # Intenção de participar de uma sessão
│   │   ├── session_checkin.rs # Check-in de presença
│   │   └── table_request.rs # Solicitação para entrar em uma mesa
│   ├── repositories/         # Interfaces dos repositórios (Ports de Saída)
│   └── services/            # Serviços de domínio (Ports de Saída)
├── application/              # Camada de Aplicação
│   └── use_cases/           # Casos de uso da aplicação
│       ├── auth_use_cases.rs    # Autenticação e registro
│       ├── user_use_cases.rs    # Gerenciamento de usuários
│       ├── table_use_cases.rs   # Gerenciamento de mesas
│       └── session_use_cases.rs # Gerenciamento de sessões
├── infrastructure/           # Camada de Infraestrutura (Adapters)
│   ├── persistence/          # Implementações de persistência
│   │   ├── postgres.rs       # Configuração do PostgreSQL
│   │   └── repositories/     # Implementações PostgreSQL dos repositórios
│   ├── http/                 # Camada HTTP
│   │   ├── handlers/         # Handlers das rotas
│   │   ├── routes.rs         # Definição das rotas
│   │   └── server.rs         # Configuração do servidor
│   ├── config/               # Configuração da aplicação
│   └── services/             # Implementações dos serviços de domínio
└── ports/                    # Portas de entrada (Interfaces HTTP)
```

## Princípios da Arquitetura Hexagonal

### 1. **Domínio (Core)**
- **Entidades**: Representam os conceitos principais do negócio
- **Regras de Negócio**: Lógica que não deve mudar independente da tecnologia
- **Portas de Saída**: Interfaces que definem como o domínio se comunica com o mundo exterior

### 2. **Aplicação**
- **Casos de Uso**: Orquestram as operações de negócio
- **Serviços de Aplicação**: Coordenam entre diferentes repositórios e serviços
- **Não contém lógica de negócio**, apenas coordenação

### 3. **Infraestrutura**
- **Implementações Concretas**: PostgreSQL, HTTP, JWT, etc.
- **Adapters**: Conectam a aplicação com tecnologias externas
- **Configuração**: Variáveis de ambiente, conexões de banco, etc.

### 4. **Portas**
- **Portas de Entrada**: APIs HTTP, CLI, etc.
- **Portas de Saída**: Repositórios, serviços externos, etc.

## Benefícios da Arquitetura

1. **Testabilidade**: Fácil mockar dependências externas
2. **Independência de Framework**: Lógica de negócio não depende de tecnologias específicas
3. **Manutenibilidade**: Mudanças na infraestrutura não afetam o domínio
4. **Flexibilidade**: Fácil trocar implementações (ex: PostgreSQL por MongoDB)
5. **Separação de Responsabilidades**: Cada camada tem uma responsabilidade clara

## Como Executar

### Pré-requisitos
- Rust 1.70+
- PostgreSQL 13+
- Docker (opcional)

### Configuração
1. Copie `.env.example` para `.env` e configure as variáveis
2. Configure o banco de dados PostgreSQL
3. Execute as migrações: `cargo run --bin migrate`

### Execução
```bash
cargo run
```

### Desenvolvimento com Docker
```bash
docker-compose up -d
cargo run
```

## Exemplo de Uso

### Criando uma Mesa
```rust
// 1. Aplicação recebe requisição HTTP
// 2. Handler converte para DTO
// 3. Use Case valida e executa regras de negócio
// 4. Repository persiste no banco
// 5. Resposta é retornada via HTTP

let table = table_use_cases.create_table(CreateTableRequest {
    gm_id: user.id,
    title: "Aventura Épica".to_string(),
    description: "Uma jornada fantástica...".to_string(),
    visibility: TableVisibility::Public,
    player_slots: 6,
    game_system_id: dnd_system.id,
}).await?;
```

## Testes

```bash
# Testes unitários
cargo test

# Testes de integração
cargo test --test integration_tests

# Testes com cobertura
cargo tarpaulin
```

## Contribuição

1. Fork o projeto
2. Crie uma branch para sua feature
3. Implemente seguindo a arquitetura hexagonal
4. Adicione testes
5. Abra um Pull Request

## Licença

MIT License - veja [LICENSE](LICENSE) para detalhes.
