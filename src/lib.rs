use axum::{
    routing::{get_service, get, post},
    Router,
    extract::{ws::{WebSocket, WebSocketUpgrade}, State},
    response::Response,
    Json,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use futures::{sink::SinkExt, stream::StreamExt};

// Event system types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UIEvent {
    pub element_id: String,
    pub event_type: String,
    pub data: serde_json::Value,
    pub request_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UIResponse {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<serde_json::Value>,
    pub request_id: Option<u32>,
}

// Handler function type
pub type EventHandler = Box<dyn Fn(UIEvent) -> Result<UIResponse, String> + Send + Sync>;

// Event registry
pub type EventRegistry = Arc<RwLock<HashMap<String, EventHandler>>>;

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
    event_registry: EventRegistry,
}

impl WebUI {
    pub fn new(config: WebUIConfig) -> Self {
        Self { 
            config,
            event_registry: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register an event handler for a specific element and event type
    pub async fn bind_event<F>(&self, element_id: &str, event_type: &str, handler: F)
    where
        F: Fn(UIEvent) -> Result<UIResponse, String> + Send + Sync + 'static,
    {
        let key = format!("{}:{}", element_id, event_type);
        let mut registry = self.event_registry.write().await;
        registry.insert(key, Box::new(handler));
    }

    /// Register a simple click handler that doesn't return data
    pub async fn bind_click<F>(&self, element_id: &str, handler: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        let key = format!("{}:click", element_id);
        let mut registry = self.event_registry.write().await;
        registry.insert(key, Box::new(move |_event| {
            handler();
            Ok(UIResponse {
                success: true,
                message: None,
                data: None,
                request_id: None,
            })
        }));
    }

    async fn websocket_handler(
        ws: WebSocketUpgrade,
        State(event_registry): State<EventRegistry>,
    ) -> Response {
        ws.on_upgrade(move |socket| Self::handle_socket(socket, event_registry))
    }

    async fn handle_socket(socket: WebSocket, event_registry: EventRegistry) {
        let (mut sender, mut receiver) = socket.split();
        
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if let Ok(text) = msg.to_text() {
                    if let Ok(event) = serde_json::from_str::<UIEvent>(text) {
                        let key = format!("{}:{}", event.element_id, event.event_type);
                        let request_id = event.request_id;
                        let registry = event_registry.read().await;
                        
                        let response = if let Some(handler) = registry.get(&key) {
                            match handler(event) {
                                Ok(mut response) => {
                                    response.request_id = request_id;
                                    response
                                },
                                Err(error) => UIResponse {
                                    success: false,
                                    message: Some(error),
                                    data: None,
                                    request_id,
                                },
                            }
                        } else {
                            UIResponse {
                                success: false,
                                message: Some(format!("No handler found for {}", key)),
                                data: None,
                                request_id,
                            }
                        };
                        
                        if let Ok(response_json) = serde_json::to_string(&response) {
                            let _ = sender.send(axum::extract::ws::Message::Text(response_json.into())).await;
                        }
                    }
                }
            }
        }
    }

    async fn http_event_handler(
        State(event_registry): State<EventRegistry>,
        Json(event): Json<UIEvent>,
    ) -> Json<UIResponse> {
        let key = format!("{}:{}", event.element_id, event.event_type);
        let registry = event_registry.read().await;
        
        let response = if let Some(handler) = registry.get(&key) {
            match handler(event) {
                Ok(mut response) => {
                    response.request_id = None; // HTTP doesn't need request IDs
                    response
                },
                Err(error) => UIResponse {
                    success: false,
                    message: Some(error),
                    data: None,
                    request_id: None,
                },
            }
        } else {
            UIResponse {
                success: false,
                message: Some(format!("No handler found for {}", key)),
                data: None,
                request_id: None,
            }
        };
        
        Json(response)
    }

    fn create_router(&self) -> Router {
        Router::new()
            .route("/ws", get(Self::websocket_handler))
            .route("/api/event", post(Self::http_event_handler))
            .with_state(self.event_registry.clone())
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
