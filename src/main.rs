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
use middleware::JwtAuth;
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

    let auth_service = AuthService::new(pool.clone(), config.jwt_secret.clone());

    let server = HttpServer::new(move || {
        // Create the JWT middleware instance
        let jwt_middleware = JwtAuth::new(config.jwt_secret.clone());

        App::new()
            .app_data(web::Data::new(auth_service.clone()))
            .wrap(tracing_actix_web::TracingLogger::default())
            .route("/health", web::get().to(|| async { "✅ OK" }))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/login")
                            .route(web::post().to(handlers::login)))
                    // Apply JWT middleware only to protected routes
                    .service(
                        web::scope("/protected")
                            .wrap(jwt_middleware)
                            .route("", web::get().to(|| async { "🔐 Protected route" })),
                    ),
            )
    })
    .workers(4)
    .bind(("0.0.0.0", config.server_port))
    .map_err(|e| {
        error!("❌ Could not bind to port {}: {}", config.server_port, e);
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
    if let Err(e) = signal::ctrl_c().await {
        error!("Failed to listen for shutdown signal: {}", e);
    }
}