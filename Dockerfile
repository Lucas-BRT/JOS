# --- Builder Stage ---
# Usamos uma imagem completa do Rust para compilar a aplicação
FROM rust:1.90-slim as builder

# Instala dependências de compilação, como o linker para build estático
RUN apt-get update && apt-get install -y musl-tools musl-dev

# Cria um diretório de trabalho
WORKDIR /app

# Copia os arquivos de dependência e compila apenas as dependências (cache layer)
COPY Cargo.toml Cargo.lock ./
# Cria um projeto dummy para compilar as dependências separadamente
RUN mkdir src && echo "fn main(){}" > src/main.rs && \
    cargo build --release --target=x86_64-unknown-linux-musl && \
    rm -rf src

# Copia o código-fonte da aplicação
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx

# Compila a aplicação final
# O --touch força o cargo a verificar se algo mudou
RUN touch src/main.rs && cargo build --release --target=x86_64-unknown-linux-musl

# --- Final Stage ---
# Usamos uma imagem mínima para a versão final. `scratch` é a menor possível.
FROM scratch

# Copia apenas o binário compilado do estágio de build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/jos /jos

# Expõe a porta que sua aplicação usa (ajuste se necessário)
EXPOSE 8000

# Comando para executar a aplicação
CMD ["/jos"]