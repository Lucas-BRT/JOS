use std::io::{self, Write};
use std::process::{Command, Stdio};

pub async fn start_services() {
    println!("🚀 Starting development services...");

    let status = Command::new("docker-compose").args(["up", "-d"]).status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("✅ Services started successfully");
            println!("📊 Services available at:");
            println!("  • API: http://localhost:3000");
            println!("  • pgAdmin: http://localhost:8080");
            println!("  • Database: localhost:5432");
            println!("  • Redis: localhost:6379");
        }
        _ => {
            eprintln!("❌ Failed to start services");
            eprintln!("   Please check if Docker is running");
        }
    }
}

pub fn stop_services() {
    println!("🛑 Stopping development services...");

    let status = Command::new("docker-compose").arg("down").status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("✅ Services stopped successfully");
        }
        _ => {
            eprintln!("❌ Failed to stop services");
        }
    }
}

pub async fn restart_services() {
    println!("🔄 Restarting development services...");

    // Stop services
    stop_services();

    // Wait a bit
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Start services
    start_services().await;
}

pub fn show_logs(service: &str) {
    println!("📋 Showing logs for: {service}");

    let args = match service {
        "api" => vec!["logs", "-f", "api"],
        "db" => vec!["logs", "-f", "db"],
        "redis" => vec!["logs", "-f", "redis"],
        "all" => vec!["logs", "-f"],
        _ => {
            eprintln!("❌ Invalid service: {service}");
            eprintln!("   Valid services: api, db, redis, all");
            return;
        }
    };

    let child = Command::new("docker-compose")
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match child {
        Ok(mut child) => {
            // Wait for user to press Ctrl+C
            ctrlc::set_handler(move || {
                println!("\n🛑 Stopping logs...");
                std::process::exit(0);
            })
            .expect("Error setting Ctrl-C handler");

            let _ = child.wait();
        }
        Err(e) => {
            eprintln!("❌ Failed to show logs: {e}");
        }
    }
}

pub fn show_status() {
    println!("📊 Service Status:");
    println!();

    // Show Docker Compose status
    let output = Command::new("docker-compose").arg("ps").output();

    match output {
        Ok(output) => {
            let status = String::from_utf8_lossy(&output.stdout);
            println!("{status}");
        }
        Err(e) => {
            eprintln!("❌ Failed to get service status: {e}");
        }
    }

    println!();

    // Check health of services
    check_service_health();
}

fn check_service_health() {
    // Check database
    let db_healthy = Command::new("docker-compose")
        .args([
            "exec",
            "-T",
            "db",
            "pg_isready",
            "-U",
            "postgres",
            "-d",
            "jos_db",
        ])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if db_healthy {
        println!("✅ Database: Healthy");
    } else {
        println!("❌ Database: Unhealthy");
    }

    // Check Redis
    let redis_healthy = Command::new("docker-compose")
        .args(["exec", "-T", "redis", "redis-cli", "ping"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if redis_healthy {
        println!("✅ Redis: Healthy");
    } else {
        println!("❌ Redis: Unhealthy");
    }

    // Check API (if running)
    let api_healthy = Command::new("curl")
        .args(["-f", "http://localhost:3000/health"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if api_healthy {
        println!("✅ API: Healthy");
    } else {
        println!("⚠️  API: Not responding (might not be running)");
    }
}

pub async fn reset_database() {
    println!("🔄 Resetting database...");
    println!("⚠️  This will completely reset the database. All data will be lost!");
    print!("Are you sure? (y/N): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "y" {
        println!("🛑 Stopping services...");
        stop_services();

        println!("🗑️ Removing database volumes...");
        let _ = Command::new("docker")
            .args(["volume", "rm", "jos_postgres_data"])
            .output();

        println!("🚀 Starting services...");
        start_services().await;

        // Wait for database to be ready
        wait_for_database().await;

        println!("🗄️ Running migrations...");
        run_migrations().await;

        println!("✅ Database reset complete");
    } else {
        println!("❌ Database reset cancelled");
    }
}

async fn wait_for_database() {
    println!("⏳ Waiting for database to be ready...");

    let mut attempts = 0;
    let max_attempts = 60;

    while attempts < max_attempts {
        let output = Command::new("docker-compose")
            .args([
                "exec",
                "-T",
                "db",
                "pg_isready",
                "-U",
                "postgres",
                "-d",
                "jos_db",
            ])
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

pub async fn run_migrations() {
    println!("🗄️ Running database migrations...");

    let status = Command::new("sqlx").args(["migrate", "run"]).status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("✅ Database migrations completed");
        }
        _ => {
            eprintln!("❌ Failed to run migrations");
            eprintln!("   Please check if the database is running");
        }
    }
}

pub fn open_db_shell() {
    println!("🐘 Opening PostgreSQL shell...");

    let status = Command::new("docker-compose")
        .args(["exec", "db", "psql", "-U", "postgres", "-d", "jos_db"])
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            // Shell was closed normally
        }
        _ => {
            eprintln!("❌ Failed to open database shell");
        }
    }
}

pub fn open_redis_shell() {
    println!("🔴 Opening Redis shell...");

    let status = Command::new("docker-compose")
        .args(["exec", "redis", "redis-cli"])
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            // Shell was closed normally
        }
        _ => {
            eprintln!("❌ Failed to open Redis shell");
        }
    }
}

pub fn run_tests() {
    println!("🧪 Running tests...");

    let status = Command::new("cargo").arg("test").status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("✅ All tests passed");
        }
        _ => {
            eprintln!("❌ Some tests failed");
            std::process::exit(1);
        }
    }
}

pub fn build_project() {
    println!("🔨 Building project...");

    let status = Command::new("cargo").arg("build").status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("✅ Build completed");
        }
        _ => {
            eprintln!("❌ Build failed");
            std::process::exit(1);
        }
    }
}

pub fn clean_project() {
    println!("🧹 Cleaning build artifacts...");

    let status = Command::new("cargo").arg("clean").status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("✅ Build artifacts cleaned");
        }
        _ => {
            eprintln!("❌ Failed to clean build artifacts");
        }
    }
}
