use std::fs;
use std::path::Path;
use std::process::Command;

pub async fn run_setup() {
    println!("🚀 JOS Setup Tool");
    println!("==================");

    // Check Docker installation
    check_docker();
    
    // Check Docker Compose
    check_docker_compose();

    // Create .env file
    create_env_file();

    // Start services
    start_services().await;

    // Install Rust dependencies
    install_rust_dependencies();

    // Build the project
    build_project();

    // Run migrations
    run_migrations().await;

    // Run diagnosis
    run_diagnosis();

    show_completion_message();
}

fn check_docker() {
    println!("🐳 Checking Docker installation...");
    
    if Command::new("docker")
        .arg("--version")
        .output()
        .is_ok()
    {
        println!("✅ Docker is installed");
        
        // Check if Docker is running
        if Command::new("docker")
            .arg("info")
            .output()
            .is_ok()
        {
            println!("✅ Docker is running");
        } else {
            eprintln!("❌ Docker is not running");
            eprintln!("   Please start Docker and try again");
            std::process::exit(1);
        }
    } else {
        eprintln!("❌ Docker is not installed");
        eprintln!("   Please install Docker from https://docs.docker.com/get-docker/");
        std::process::exit(1);
    }
}

fn check_docker_compose() {
    println!("🐳 Checking Docker Compose...");
    
    // Try both docker-compose and docker compose
    let compose_available = Command::new("docker-compose")
        .arg("--version")
        .output()
        .is_ok() || Command::new("docker")
        .args(&["compose", "version"])
        .output()
        .is_ok();
    
    if compose_available {
        println!("✅ Docker Compose is available");
    } else {
        eprintln!("❌ Docker Compose is not available");
        eprintln!("   Please install Docker Compose");
        std::process::exit(1);
    }
}

fn create_env_file() {
    println!("📝 Setting up environment variables...");
    
    if !Path::new(".env").exists() {
        println!("📝 Creating .env file...");
        
        let env_content = r#"# JOS Development Environment
# Database Configuration
DATABASE_URL=postgres://postgres:postgres@localhost:5432/jos_db
DATABASE_URL_TEST=postgres://postgres:postgres@localhost:5432/jos_test

# Server Configuration
PORT=3000
HOST=0.0.0.0

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-should-be-at-least-32-characters-long
JWT_EXPIRATION_DAYS=1

# Redis Configuration (optional)
REDIS_URL=redis://localhost:6379

# Logging
RUST_LOG=debug
RUST_BACKTRACE=1

# Development Settings
ENVIRONMENT=development
ENABLE_SWAGGER=true
ENABLE_HEALTH_CHECK=true
ENABLE_DETAILED_ERRORS=true
ENABLE_CORS=true
CORS_ALLOWED_ORIGINS=http://localhost:3000,http://localhost:8080

# Security Settings
RATE_LIMIT_REQUESTS_PER_MINUTE=100
SESSION_TIMEOUT_MINUTES=60
"#;

        match fs::write(".env", env_content) {
            Ok(_) => println!("✅ .env file created successfully!"),
            Err(e) => {
                eprintln!("❌ Failed to create .env file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        println!("✅ .env file already exists");
    }
}

async fn start_services() {
    println!("🚀 Starting development services...");
    
    // Stop any existing containers
    let _ = Command::new("docker-compose")
        .arg("down")
        .output();
    
    // Start services
    let status = Command::new("docker-compose")
        .args(&["up", "-d", "db", "redis"])
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("✅ Services started successfully");
            
            // Wait for database to be ready
            wait_for_database().await;
            
            // Wait for Redis to be ready
            wait_for_redis().await;
        }
        _ => {
            eprintln!("❌ Failed to start services");
            eprintln!("   Please check if Docker is running and try again");
            std::process::exit(1);
        }
    }
}

async fn wait_for_database() {
    println!("⏳ Waiting for database to be ready...");
    
    let mut attempts = 0;
    let max_attempts = 60;
    
    while attempts < max_attempts {
        let output = Command::new("docker-compose")
            .args(&["exec", "-T", "db", "pg_isready", "-U", "postgres", "-d", "jos_db"])
            .output();
        
        if output.is_ok() && output.unwrap().status.success() {
            println!("✅ Database is ready");
            return;
        }
        
        std::thread::sleep(std::time::Duration::from_secs(1));
        attempts += 1;
        print!(".");
    }
    
    eprintln!("\n❌ Database failed to start within 60 seconds");
    std::process::exit(1);
}

async fn wait_for_redis() {
    println!("⏳ Waiting for Redis to be ready...");
    
    let mut attempts = 0;
    let max_attempts = 30;
    
    while attempts < max_attempts {
        let output = Command::new("docker-compose")
            .args(&["exec", "-T", "redis", "redis-cli", "ping"])
            .output();
        
        if output.is_ok() && output.unwrap().status.success() {
            println!("✅ Redis is ready");
            return;
        }
        
        std::thread::sleep(std::time::Duration::from_secs(1));
        attempts += 1;
        print!(".");
    }
    
    eprintln!("\n❌ Redis failed to start within 30 seconds");
    std::process::exit(1);
}

fn install_rust_dependencies() {
    println!("🔧 Installing Rust dependencies...");
    
    // Install sqlx-cli if not already installed
    if Command::new("sqlx")
        .arg("--version")
        .output()
        .is_ok()
    {
        println!("✅ sqlx-cli is already installed");
    } else {
        println!("📦 Installing sqlx-cli...");
        let status = Command::new("cargo")
            .args(&["install", "sqlx-cli", "--no-default-features", "--features", "rustls,postgres"])
            .status();

        match status {
            Ok(exit_status) if exit_status.success() => {
                println!("✅ sqlx-cli installed successfully");
            }
            _ => {
                eprintln!("❌ Failed to install sqlx-cli");
                eprintln!("   Please install manually: cargo install sqlx-cli --no-default-features --features rustls,postgres");
            }
        }
    }
    
    // Install cargo-watch for development
    if Command::new("cargo-watch")
        .arg("--version")
        .output()
        .is_ok()
    {
        println!("✅ cargo-watch is already installed");
    } else {
        println!("📦 Installing cargo-watch...");
        let status = Command::new("cargo")
            .args(&["install", "cargo-watch"])
            .status();

        match status {
            Ok(exit_status) if exit_status.success() => {
                println!("✅ cargo-watch installed successfully");
            }
            _ => {
                eprintln!("❌ Failed to install cargo-watch");
                eprintln!("   Please install manually: cargo install cargo-watch");
            }
        }
    }
}

fn build_project() {
    println!("🔨 Building project...");
    let build_status = Command::new("cargo")
        .arg("build")
        .status();

    match build_status {
        Ok(exit_status) if exit_status.success() => {
            println!("✅ Project built successfully");
        }
        _ => {
            eprintln!("❌ Build failed");
            std::process::exit(1);
        }
    }
}

async fn run_migrations() {
    println!("🗄️ Running database migrations...");
    
    // Wait a bit more for database to be fully ready
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    let status = Command::new("sqlx")
        .args(&["migrate", "run"])
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("✅ Database migrations completed");
        }
        _ => {
            eprintln!("❌ Failed to run migrations");
            eprintln!("   You can try running migrations manually: sqlx migrate run");
        }
    }
}

fn run_diagnosis() {
    println!("🔍 Running system diagnosis...");
    let diagnosis_status = Command::new("cargo")
        .args(&["run", "-p", "jos-cli", "diagnose"])
        .status();

    match diagnosis_status {
        Ok(exit_status) if exit_status.success() => {
            println!("✅ System diagnosis passed");
        }
        _ => {
            println!("⚠️  System diagnosis found issues");
            println!("   Run 'cargo run -p jos-cli diagnose' for detailed information");
        }
    }
}

fn show_completion_message() {
    println!("\n🎉 Setup completed successfully!");
    println!("==================================");
    println!();
    println!("📊 Services Status:");
    println!("  • Database: http://localhost:5432 (postgres/postgres)");
    println!("  • Redis: http://localhost:6379");
    println!("  • pgAdmin: http://localhost:8080 (admin@jos.local/admin)");
    println!();
    println!("🚀 Available Commands:");
    println!("  • Start API: cargo run");
    println!("  • Hot reload: cargo watch -x run");
    println!("  • Run tests: cargo test");
    println!("  • Database migrations: sqlx migrate run");
    println!("  • Stop services: docker-compose down");
    println!("  • View logs: docker-compose logs -f");
    println!();
    println!("📚 Useful URLs:");
    println!("  • API: http://localhost:3000");
    println!("  • API Docs: http://localhost:3000/docs");
    println!("  • Health Check: http://localhost:3000/health");
    println!();
    println!("🔧 Development Tips:");
    println!("  • Use 'cargo watch -x run' for automatic reloading");
    println!("  • Check logs: docker-compose logs -f");
    println!("  • Reset database: docker-compose down -v && docker-compose up -d");
    println!();
}
