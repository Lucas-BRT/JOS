use sqlx::{PgPool, Row};
use tracing::info;

use std::env;

#[derive(Debug, thiserror::Error)]
pub enum DiagnosticError {
    #[error("Database connection failed: {0}")]
    DatabaseConnection(String),
    #[error("Environment validation failed: {0}")]
    EnvironmentValidation(String),
    #[error("Migration failed: {0}")]
    Migration(String),
}

/// Diagnostic information about the system
#[derive(Debug)]
pub struct DiagnosticResult {
    pub environment_ok: bool,
    pub database_ok: bool,
    pub migrations_ok: bool,
    pub issues: Vec<String>,
    pub suggestions: Vec<String>,
}

impl DiagnosticResult {
    pub fn is_healthy(&self) -> bool {
        self.environment_ok && self.database_ok && self.migrations_ok
    }

    pub fn print_report(&self) {
        println!("\n🔍 JOS System Diagnosis");
        println!("========================");

        // Environment check
        if self.environment_ok {
            println!("✅ Environment variables are properly configured");
        } else {
            println!("❌ Environment configuration issues found");
        }

        // Database check
        if self.database_ok {
            println!("✅ Database connection is working");
        } else {
            println!("❌ Database connection issues found");
        }

        // Migrations check
        if self.migrations_ok {
            println!("✅ Database migrations are up to date");
        } else {
            println!("❌ Database migration issues found");
        }

        // Print issues
        if !self.issues.is_empty() {
            println!("\n🚨 Issues Found:");
            for issue in &self.issues {
                println!("  • {}", issue);
            }
        }

        // Print suggestions
        if !self.suggestions.is_empty() {
            println!("\n💡 Suggestions:");
            for suggestion in &self.suggestions {
                println!("  • {}", suggestion);
            }
        }

        // Summary
        println!("\n🎯 Summary:");
        if self.is_healthy() {
            println!("✅ System is healthy and ready to run");
            println!("   Run: cargo run");
        } else {
            println!("❌ System has issues that need to be resolved");
            println!("   Please fix the issues above before running the application");
        }
    }
}

/// Validates environment variables and returns detailed feedback
pub async fn validate_environment() -> DiagnosticResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();

    info!("🔍 Validating environment configuration...");

    // Check required environment variables
    let required_vars = vec![
        ("DATABASE_URL", "Database connection string"),
        ("PORT", "Server port number"),
        ("JWT_SECRET", "JWT signing secret"),
    ];

    let mut missing_vars = Vec::new();

    for (var, _description) in required_vars {
        match env::var(var) {
            Ok(value) => {
                if value.is_empty() {
                    missing_vars.push(var);
                    issues.push(format!("{} is empty", var));
                } else {
                    info!("✅ {} is set", var);
                }
            }
            Err(_) => {
                missing_vars.push(var);
                issues.push(format!("{} is not set", var));
            }
        }
    }

    if !missing_vars.is_empty() {
        suggestions
            .push("Create a .env file in the project root with the required variables".to_string());
        suggestions.push("Run 'cargo run -p jos-cli setup' to create a .env template".to_string());
        suggestions.push(format!("Required variables: {}", missing_vars.join(", ")));
    }

    // Validate DATABASE_URL format
    if let Ok(db_url) = env::var("DATABASE_URL") {
        if !db_url.starts_with("postgres://") && !db_url.starts_with("postgresql://") {
            issues
                .push("DATABASE_URL must start with 'postgres://' or 'postgresql://'".to_string());
            suggestions
                .push("Use format: postgres://username:password@host:port/database".to_string());
        }
    }

    // Validate PORT
    if let Ok(port_str) = env::var("PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            if port < 1024 {
                issues.push(format!("PORT {} is outside valid range (1024-65535)", port));
                suggestions.push("Use a port between 1024 and 65535".to_string());
                suggestions.push("Common ports: 3000, 8080, 5000".to_string());
            }
        } else {
            issues.push("PORT must be a valid number".to_string());
            suggestions.push("Set PORT to a number, e.g., PORT=3000".to_string());
        }
    }

    // Validate JWT_SECRET length
    if let Ok(jwt_secret) = env::var("JWT_SECRET") {
        if jwt_secret.len() < 32 {
            issues.push("JWT_SECRET is shorter than recommended (32+ characters)".to_string());
            suggestions.push("Use a longer JWT_SECRET for better security".to_string());
        }
    }

    DiagnosticResult {
        environment_ok: issues.is_empty(),
        database_ok: false,   // Will be checked separately
        migrations_ok: false, // Will be checked separately
        issues,
        suggestions,
    }
}

/// Tests database connectivity and returns detailed feedback
pub async fn test_database_connection() -> DiagnosticResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();

    info!("🔍 Testing database connection...");

    // Get DATABASE_URL
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            issues.push("DATABASE_URL is not set".to_string());
            suggestions.push("Set DATABASE_URL in your .env file".to_string());
            return DiagnosticResult {
                environment_ok: false,
                database_ok: false,
                migrations_ok: false,
                issues,
                suggestions,
            };
        }
    };

    // Test basic connection
    match sqlx::PgPool::connect(&db_url).await {
        Ok(pool) => {
            info!("✅ Database connection established");

            // Test basic query
            match sqlx::query("SELECT 1").execute(&pool).await {
                Ok(_) => {
                    info!("✅ Database is responding to queries");

                    // Check if database exists
                    if let Some(db_name) = extract_database_name(&db_url) {
                        match sqlx::query("SELECT current_database()")
                            .fetch_one(&pool)
                            .await
                        {
                            Ok(row) => {
                                if let Ok(current_db) = row.try_get::<String, _>(0) {
                                    if current_db == db_name {
                                        info!("✅ Connected to correct database: {}", db_name);
                                    } else {
                                        issues.push(format!(
                                            "Connected to database '{}' but expected '{}'",
                                            current_db, db_name
                                        ));
                                        suggestions.push(format!(
                                            "Update DATABASE_URL to connect to '{}'",
                                            db_name
                                        ));
                                    }
                                }
                            }
                            Err(e) => {
                                issues.push(format!("Could not verify database name: {}", e));
                            }
                        }
                    }

                    // Test permissions
                    match sqlx::query(
                        "CREATE TABLE IF NOT EXISTS test_permissions (id SERIAL PRIMARY KEY)",
                    )
                    .execute(&pool)
                    .await
                    {
                        Ok(_) => {
                            match sqlx::query("DROP TABLE test_permissions")
                                .execute(&pool)
                                .await
                            {
                                Ok(_) => {
                                    info!("✅ User has CREATE/DROP permissions");
                                }
                                Err(e) => {
                                    issues.push(format!("User cannot drop tables: {}", e));
                                    suggestions.push(
                                        "Grant DROP permissions to the database user".to_string(),
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            issues.push(format!("User cannot create tables: {}", e));
                            suggestions
                                .push("Grant CREATE permissions to the database user".to_string());
                        }
                    }

                    // Test migrations
                    match test_migrations(&pool).await {
                        Ok(_) => {
                            info!("✅ Database migrations are working");
                            return DiagnosticResult {
                                environment_ok: true,
                                database_ok: true,
                                migrations_ok: true,
                                issues,
                                suggestions,
                            };
                        }
                        Err(e) => {
                            issues.push(format!("Migration test failed: {}", e));
                            suggestions.push(
                                "Run 'sqlx migrate run' manually to see detailed errors"
                                    .to_string(),
                            );
                        }
                    }
                }
                Err(e) => {
                    issues.push(format!("Database query failed: {}", e));
                    suggestions.push("Check if PostgreSQL is running".to_string());
                    suggestions.push("Verify DATABASE_URL format and credentials".to_string());
                }
            }
        }
        Err(e) => {
            let error_msg = e.to_string();

            if error_msg.contains("password authentication failed") {
                issues.push(
                    "Database authentication failed - wrong username or password".to_string(),
                );
                suggestions.push("Check username and password in DATABASE_URL".to_string());
                suggestions.push(
                    "Reset PostgreSQL password: ALTER USER username PASSWORD 'new_password';"
                        .to_string(),
                );
            } else if error_msg.contains("connection refused") {
                issues.push("Cannot connect to PostgreSQL - server not running".to_string());
                suggestions.push("Start PostgreSQL: sudo systemctl start postgresql".to_string());
                suggestions.push("Or use Docker: docker run --name jos-postgres -e POSTGRES_DB=jos_db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:16-alpine".to_string());
            } else if error_msg.contains("does not exist") {
                issues.push("Database does not exist".to_string());
                suggestions.push("Create database: CREATE DATABASE jos_db;".to_string());
                suggestions.push("Or use Docker with correct database name".to_string());
            } else {
                issues.push(format!("Database connection failed: {}", error_msg));
                suggestions.push(
                    "Check DATABASE_URL format: postgres://user:pass@host:port/db".to_string(),
                );
                suggestions.push("Verify PostgreSQL is running and accessible".to_string());
            }
        }
    }

    DiagnosticResult {
        environment_ok: true,
        database_ok: issues.is_empty(),
        migrations_ok: false,
        issues,
        suggestions,
    }
}

/// Tests database migrations
async fn test_migrations(pool: &PgPool) -> Result<(), DiagnosticError> {
    // Check if migrations directory exists
    if !std::path::Path::new("../migrations").exists() {
        return Err(DiagnosticError::Migration(
            "migrations directory not found".to_string(),
        ));
    }

    // Try to run migrations (this will fail if they're already applied, which is fine)
    match sqlx::migrate!("../migrations").run(pool).await {
        Ok(_) => {
            info!("✅ Migrations applied successfully");
            Ok(())
        }
        Err(e) => {
            // If migrations are already applied, this is not an error
            if e.to_string().contains("already applied") {
                info!("✅ Migrations are already up to date");
                Ok(())
            } else {
                Err(DiagnosticError::Migration(e.to_string()))
            }
        }
    }
}

/// Extracts database name from DATABASE_URL
fn extract_database_name(db_url: &str) -> Option<String> {
    db_url
        .split('/')
        .last()
        .map(|s| s.split('?').next().unwrap_or(s).to_string())
}

/// Runs a complete system diagnosis
pub async fn run_full_diagnosis() -> DiagnosticResult {
    println!("🔍 Starting JOS System Diagnosis...\n");

    // Validate environment
    let env_result = validate_environment().await;

    if !env_result.environment_ok {
        env_result.print_report();
        return env_result;
    }

    // Test database connection
    let db_result = test_database_connection().await;

    // Combine results
    let mut combined_issues = env_result.issues;
    combined_issues.extend(db_result.issues);

    let mut combined_suggestions = env_result.suggestions;
    combined_suggestions.extend(db_result.suggestions);

    let result = DiagnosticResult {
        environment_ok: env_result.environment_ok,
        database_ok: db_result.database_ok,
        migrations_ok: db_result.migrations_ok,
        issues: combined_issues,
        suggestions: combined_suggestions,
    };

    result.print_report();
    result
}
