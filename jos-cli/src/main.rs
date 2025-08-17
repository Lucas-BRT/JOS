use clap::{Parser, Subcommand};
use jos_cli::{run_full_diagnosis, run_setup};

#[derive(Parser)]
#[command(name = "jos-cli")]
#[command(about = "JOS CLI - Tools for managing JOS application")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run full system diagnosis
    Diagnose,
    /// Setup the JOS environment
    Setup,
    /// Start development services (Docker)
    Start,
    /// Stop development services (Docker)
    Stop,
    /// Restart development services (Docker)
    Restart,
    /// Show service logs
    Logs {
        /// Service name (api, db, redis, all)
        #[arg(default_value = "all")]
        service: String,
    },
    /// Show service status
    Status,
    /// Reset database (drop and recreate)
    ResetDb,
    /// Run database migrations
    Migrate,
    /// Open database shell
    ShellDb,
    /// Open Redis shell
    ShellRedis,
    /// Run tests
    Test,
    /// Build project
    Build,
    /// Clean build artifacts
    Clean,
}

#[tokio::main]
async fn main() {
    // Initialize basic logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Diagnose => {
            let result = run_full_diagnosis().await;
            if !result.is_healthy() {
                std::process::exit(1);
            }
        }
        Commands::Setup => {
            run_setup().await;
        }
        Commands::Start => {
            jos_cli::services::start_services().await;
        }
        Commands::Stop => {
            jos_cli::services::stop_services();
        }
        Commands::Restart => {
            jos_cli::services::restart_services().await;
        }
        Commands::Logs { service } => {
            jos_cli::services::show_logs(&service);
        }
        Commands::Status => {
            jos_cli::services::show_status();
        }
        Commands::ResetDb => {
            jos_cli::services::reset_database().await;
        }
        Commands::Migrate => {
            jos_cli::services::run_migrations().await;
        }
        Commands::ShellDb => {
            jos_cli::services::open_db_shell();
        }
        Commands::ShellRedis => {
            jos_cli::services::open_redis_shell();
        }
        Commands::Test => {
            jos_cli::services::run_tests();
        }
        Commands::Build => {
            jos_cli::services::build_project();
        }
        Commands::Clean => {
            jos_cli::services::clean_project();
        }
    }
}
