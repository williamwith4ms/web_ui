use axum::{
    routing::get_service,
    Router,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub struct WebUIConfig {
    pub port: u16,
    pub host: [u8; 4],
    pub title: String,
    pub static_dir: String,
}

impl Default for WebUIConfig {
    fn default() -> Self {
        Self {
            port: 3030,
            host: [127, 0, 0, 1],
            title: "Web UI".to_string(),
            static_dir: "./static".to_string(),
        }
    }
}
    
impl WebUIConfig {
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    pub fn with_host(mut self, host: [u8; 4]) -> Self {
        self.host = host;
        self
    }
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
    pub fn with_static_dir(mut self, static_dir: String) -> Self {
        self.static_dir = static_dir;
        self
    }
}

pub struct WebUI {
    config: WebUIConfig,
}

impl WebUI {
    pub fn new(config: WebUIConfig) -> Self {
        Self { config }
    }

    fn create_router(&self) -> Router {
        Router::new()
            .fallback_service(get_service(ServeDir::new(&self.config.static_dir)))
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = SocketAddr::from((self.config.host, self.config.port));
        println!("Listening on http://{}", addr);
        
        let app = self.create_router();
        let listener = TcpListener::bind(addr).await?;
        
        axum::serve(listener, app).await?;
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = WebUIConfig::default()
            .with_port(8080)
            .with_title("Test UI".to_string());
        
        assert_eq!(config.port, 8080);
        assert_eq!(config.title, "Test UI");
    }
}
