use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use tokio::signal;

mod config;
mod db;
mod errors;
mod handlers;
mod middleware;
mod services;

use config::AppConfig;
use db::create_db_pool;
use services::auth::AuthService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Setup structured logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .with_target(false)
        .init();

    // Load configuration
    let config = match AppConfig::from_env() {
        Ok(cfg) => cfg,
        Err(err) => {
            error!("❌ Failed to load configuration: {}", err);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Configuration error"));
        }
    };

    // Create DB pool
    let pool = match create_db_pool(&config.database_url) {
        Ok(p) => p,
        Err(err) => {
            error!("❌ Failed to create DB pool: {}", err);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database connection error"));
        }
    };

    let auth_service = AuthService::new(pool, config.jwt_secret.clone());

    // Start HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(auth_service.clone()))
            .wrap(actix_web::middleware::Logger::default()) // Can be replaced with a tracing middleware
            .route("/login", web::post().to(handlers::login))
    })
    .bind(("0.0.0.0", config.server_port))
    .map_err(|e| {
        error!("❌ Failed to bind to port {}: {}", config.server_port, e);
        std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, e)
    })?;

    info!("🚀 Starting Tesla-Authenticator app on port {}", config.server_port);

    // Graceful shutdown hook
    let graceful = server.run();
    tokio::select! {
        _ = graceful => {
            info!("🛑 Server exited gracefully.");
        }
        _ = shutdown_signal() => {
            info!("📴 Received shutdown signal. Shutting down server...");
        }
    }

    Ok(())
}

// SIGINT / SIGTERM listener
async fn shutdown_signal() {
    // Wait for Ctrl+C
    if let Err(e) = signal::ctrl_c().await {
        error!("Failed to listen for shutdown signal: {}", e);
    }
}