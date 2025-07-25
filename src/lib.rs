//! # Web UI
//!
//! A simple Rust library for creating local web interfaces with real-time communication.
//!
//! This library provides a framework for building local web applications with event-driven
//! architecture, supporting both WebSocket and HTTP-based communication between
//! the frontend and backend.
//!
//! ## Features
//!
//! - Event-driven architecture with customizable handlers
//! - WebSocket support for real-time communication
//! - HTTP fallback for event handling
//! - Static file serving
//! - Simple configuration API
//!
//! ## Quick Start
//!
//! ```rust
//! use web_ui::{WebUI, WebUIConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = WebUIConfig::default()
//!     .with_port(3030)
//!     .with_title("My Web App".to_string());
//!
//! let webui = WebUI::new(config);
//!
//! // Bind a click event handler
//! webui.bind_click("my-button", || {
//!     println!("Button clicked!");
//! }).await;
//!
//! // webui.run().await // This would start the server
//! # Ok(())
//! # }
//! ```
//! 
//! Ensure that the `webui.js` file is included in your static files directory 
//! - (https://raw.githubusercontent.com/williamwith4ms/web_ui/refs/heads/main/static/webui.js)
//!
//! ## Event System
//!
//! The library uses an event-driven architecture where UI events from the frontend
//! are dispatched to registered handlers on the backend. Events are identified by
//! a combination of element ID and event type (e.g., "button1:click").

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

/// Represents a UI event sent from the frontend to the backend.
///
/// This structure contains all the information needed to identify and handle
/// an event triggered by user interaction in the web interface.
///
/// # Examples
///
/// ```rust
/// use web_ui::UIEvent;
/// use serde_json::json;
///
/// let event = UIEvent {
///     element_id: "submit-button".to_string(),
///     event_type: "click".to_string(),
///     data: json!({"value": "Submit"}),
///     request_id: Some(123),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UIEvent {
    /// The unique identifier of the UI element that triggered the event
    pub element_id: String,
    /// The type of event (e.g., "click", "change", "submit")
    pub event_type: String,
    /// Additional data associated with the event (e.g., form values, coordinates)
    pub data: serde_json::Value,
    /// Optional request ID for matching responses with requests in async scenarios
    pub request_id: Option<u32>,
}

/// Represents a response sent from the backend to the frontend after processing an event.
///
/// This structure is used to communicate the result of event handling back to the
/// frontend, including success status, optional messages, and data payload.
///
/// # Examples
///
/// ```rust
/// use web_ui::UIResponse;
/// use serde_json::json;
///
/// let response = UIResponse {
///     success: true,
///     message: Some("Data saved successfully".to_string()),
///     data: Some(json!({"id": 42})),
///     request_id: Some(123),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct UIResponse {
    /// Whether the event was processed successfully
    pub success: bool,
    /// Optional human-readable message describing the result
    pub message: Option<String>,
    /// Optional data payload to send back to the frontend
    pub data: Option<serde_json::Value>,
    /// Request ID matching the original event request
    pub request_id: Option<u32>,
}

// Handler function type

/// Type alias for event handler functions.
///
/// Event handlers are functions that take a `UIEvent` and return a `Result<UIResponse, String>`.
/// They must be thread-safe (`Send + Sync`) to work with the async runtime.
pub type EventHandler = Box<dyn Fn(UIEvent) -> Result<UIResponse, String> + Send + Sync>;

// Event registry

/// Type alias for the event registry that maps event keys to handlers.
///
/// The registry uses a combination of element ID and event type as the key
/// (format: "element_id:event_type") to uniquely identify event handlers.
pub type EventRegistry = Arc<RwLock<HashMap<String, EventHandler>>>;

/// Configuration for the WebUI server.
///
/// This struct contains all the settings needed to configure and run the web server,
/// including network settings, UI customization, and file serving options.
///
/// # Examples
///
/// ```rust
/// use web_ui::WebUIConfig;
///
/// let config = WebUIConfig::default()
///     .with_port(8080)
///     .with_host([0, 0, 0, 0]) // Listen on all interfaces
///     .with_title("My Application".to_string())
///     .with_static_dir("./public".to_string());
/// ```
pub struct WebUIConfig {
    /// Port number to bind the server to
    pub port: u16,
    /// Host IP address as a 4-byte array [a, b, c, d]
    pub host: [u8; 4],
    /// Title of the web application (used in HTML title tag)
    pub title: String,
    /// Directory path containing static files to serve
    pub static_dir: String,
}

impl Default for WebUIConfig {
    /// Creates a default configuration with sensible defaults.
    ///
    /// # Default Values
    ///
    /// - Port: 3030
    /// - Host: [127, 0, 0, 1] (localhost)
    /// - Title: "Web UI"
    /// - Static directory: "./static"
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
    /// Sets the port number for the server.
    ///
    /// # Arguments
    ///
    /// * `port` - The port number to bind to (1-65535)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use web_ui::WebUIConfig;
    ///
    /// let config = WebUIConfig::default().with_port(8080);
    /// ```
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets the host IP address for the server.
    ///
    /// # Arguments
    ///
    /// * `host` - IP address as a 4-byte array [a, b, c, d]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use web_ui::WebUIConfig;
    ///
    /// // Listen on all interfaces
    /// let config = WebUIConfig::default().with_host([0, 0, 0, 0]);
    /// ```
    pub fn with_host(mut self, host: [u8; 4]) -> Self {
        self.host = host;
        self
    }

    /// Sets the title of the web application.
    ///
    /// # Arguments
    ///
    /// * `title` - The title string to use in the HTML title tag
    ///
    /// # Examples
    ///
    /// ```rust
    /// use web_ui::WebUIConfig;
    ///
    /// let config = WebUIConfig::default().with_title("My App".to_string());
    /// ```
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Sets the directory path for serving static files.
    ///
    /// # Arguments
    ///
    /// * `static_dir` - Path to the directory containing static files
    ///
    /// # Examples
    ///
    /// ```rust
    /// use web_ui::WebUIConfig;
    ///
    /// let config = WebUIConfig::default().with_static_dir("./public".to_string());
    /// ```
    pub fn with_static_dir(mut self, static_dir: String) -> Self {
        self.static_dir = static_dir;
        self
    }
}

/// The main WebUI server instance.
///
/// This struct represents a configured web server that can handle UI events
/// and serve static files. It maintains an event registry for mapping UI
/// events to handler functions.
///
/// # Examples
///
/// ```rust
/// use web_ui::{WebUI, WebUIConfig};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = WebUIConfig::default().with_port(3030);
/// let webui = WebUI::new(config);
///
/// // Register event handlers
/// webui.bind_click("button1", || {
///     println!("Button 1 clicked!");
/// }).await;
///
/// // Start the server (commented out for doctest)
/// // webui.run().await
/// # Ok(())
/// # }
/// ```
pub struct WebUI {
    config: WebUIConfig,
    event_registry: EventRegistry,
}

impl WebUI {
    /// Creates a new WebUI instance with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration settings for the web server
    ///
    /// # Examples
    ///
    /// ```rust
    /// use web_ui::{WebUI, WebUIConfig};
    ///
    /// # async fn example() {
    /// let config = WebUIConfig::default().with_port(8080);
    /// let webui = WebUI::new(config);
    /// # }
    /// ```
    pub fn new(config: WebUIConfig) -> Self {
        Self { 
            config,
            event_registry: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register an event handler for a specific element and event type.
    ///
    /// This method allows you to bind custom handler functions to UI events.
    /// The handler function receives a `UIEvent` and should return a `UIResponse`
    /// or an error message.
    ///
    /// # Arguments
    ///
    /// * `element_id` - The ID of the HTML element to bind to
    /// * `event_type` - The type of event to handle (e.g., "click", "change")
    /// * `handler` - The function to call when the event occurs
    ///
    /// # Examples
    ///
    /// ```rust
    /// use web_ui::{WebUI, WebUIConfig, UIEvent, UIResponse};
    /// use serde_json::json;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let webui = WebUI::new(WebUIConfig::default());
    ///
    /// webui.bind_event("form1", "submit", |event| {
    ///     println!("Form submitted with data: {:?}", event.data);
    ///     Ok(UIResponse {
    ///         success: true,
    ///         message: Some("Form processed successfully".to_string()),
    ///         data: Some(json!({"result": "ok"})),
    ///         request_id: event.request_id,
    ///     })
    /// }).await;
    /// # }
    /// ```
    pub async fn bind_event<F>(&self, element_id: &str, event_type: &str, handler: F)
    where
        F: Fn(UIEvent) -> Result<UIResponse, String> + Send + Sync + 'static,
    {
        let key = format!("{}:{}", element_id, event_type);
        let mut registry = self.event_registry.write().await;
        registry.insert(key, Box::new(handler));
    }

    /// Register a simple click handler that doesn't return data.
    ///
    /// This is a convenience method for registering click event handlers that
    /// don't need to process event data or return responses. The handler function
    /// is called when the specified element is clicked.
    ///
    /// # Arguments
    ///
    /// * `element_id` - The ID of the HTML element to bind to
    /// * `handler` - The function to call when the element is clicked
    ///
    /// # Examples
    ///
    /// ```rust
    /// use web_ui::{WebUI, WebUIConfig};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let webui = WebUI::new(WebUIConfig::default());
    ///
    /// webui.bind_click("logout-button", || {
    ///     println!("User logged out");
    ///     // Perform logout logic here
    /// }).await;
    /// # }
    /// ```
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

    /// WebSocket upgrade handler for real-time communication.
    ///
    /// This method handles the WebSocket upgrade request and delegates
    /// to the socket handler for processing WebSocket messages.
    ///
    /// # Arguments
    ///
    /// * `ws` - WebSocket upgrade request
    /// * `event_registry` - Shared event registry for handling events
    ///
    /// # Returns
    ///
    /// HTTP response that upgrades the connection to WebSocket
    async fn websocket_handler(
        ws: WebSocketUpgrade,
        State(event_registry): State<EventRegistry>,
    ) -> Response {
        ws.on_upgrade(move |socket| Self::handle_socket(socket, event_registry))
    }

    /// Handles WebSocket connections and processes incoming events.
    ///
    /// This method maintains a WebSocket connection, listens for incoming
    /// UI events, processes them through registered handlers, and sends
    /// responses back to the client.
    ///
    /// # Arguments
    ///
    /// * `socket` - The WebSocket connection
    /// * `event_registry` - Shared registry of event handlers
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

    /// HTTP event handler for processing events via REST API.
    ///
    /// This method provides an HTTP endpoint for sending UI events when
    /// WebSocket communication is not available or preferred. Events are
    /// processed synchronously and responses are returned immediately.
    ///
    /// # Arguments
    ///
    /// * `event_registry` - Shared registry of event handlers
    /// * `event` - The UI event to process
    ///
    /// # Returns
    ///
    /// JSON response containing the processing result
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

    /// Creates the Axum router with all routes and middleware configured.
    ///
    /// This method sets up the web server routes including:
    /// - `/ws` - WebSocket endpoint for real-time communication
    /// - `/api/event` - HTTP endpoint for event handling
    /// - Static file serving for all other requests
    ///
    /// # Returns
    ///
    /// Configured Axum router ready to serve requests
    fn create_router(&self) -> Router {
        Router::new()
            .route("/ws", get(Self::websocket_handler))
            .route("/api/event", post(Self::http_event_handler))
            .with_state(self.event_registry.clone())
            .fallback_service(get_service(ServeDir::new(&self.config.static_dir)))
    }

    /// Starts the web server and begins listening for connections.
    ///
    /// This method consumes the WebUI instance and starts the web server
    /// on the configured host and port. The server will continue running
    /// until the process is terminated or an error occurs.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the server shuts down gracefully, or an error if
    /// the server fails to start or encounters a fatal error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use web_ui::{WebUI, WebUIConfig};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = WebUIConfig::default().with_port(3030);
    /// let webui = WebUI::new(config);
    /// 
    /// println!("Starting server...");
    /// // webui.run().await // This would start the server
    /// # Ok(())
    /// # }
    /// ```
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

    /// Test that WebUIConfig can be created and configured properly.
    #[test]
    fn test_config_creation() {
        let config = WebUIConfig::default()
            .with_port(8080)
            .with_title("Test UI".to_string());
        
        assert_eq!(config.port, 8080);
        assert_eq!(config.title, "Test UI");
    }

    /// Test that default configuration has expected values.
    #[test]
    fn test_default_config() {
        let config = WebUIConfig::default();
        
        assert_eq!(config.port, 3030);
        assert_eq!(config.host, [127, 0, 0, 1]);
        assert_eq!(config.title, "Web UI");
        assert_eq!(config.static_dir, "./static");
    }

    /// Test UIEvent serialization and deserialization.
    #[test]
    fn test_ui_event_serde() {
        use serde_json::json;
        
        let event = UIEvent {
            element_id: "test-button".to_string(),
            event_type: "click".to_string(),
            data: json!({"value": "test"}),
            request_id: Some(123),
        };
        
        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: UIEvent = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(event.element_id, deserialized.element_id);
        assert_eq!(event.event_type, deserialized.event_type);
        assert_eq!(event.request_id, deserialized.request_id);
    }

    /// Test UIResponse serialization and deserialization.
    #[test]
    fn test_ui_response_serde() {
        use serde_json::json;
        
        let response = UIResponse {
            success: true,
            message: Some("Test message".to_string()),
            data: Some(json!({"result": "ok"})),
            request_id: Some(456),
        };
        
        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: UIResponse = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(response.success, deserialized.success);
        assert_eq!(response.message, deserialized.message);
        assert_eq!(response.request_id, deserialized.request_id);
    }
}
