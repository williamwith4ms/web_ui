# WebUI

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple Rust library for creating web interfaces for local programs.

## Quick Start

Add crate to your `Cargo.toml`:

### Basic Example

```rust
use web_ui::{WebUI, WebUIConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = WebUIConfig::default();
    let web_ui = WebUI::new(config);
    
    // Bind a simple click handler
    web_ui.bind_click("hello-btn", || {
        println!("Hello, World!");
    }).await;
    
    println!("Starting web UI on http://localhost:3030");
    web_ui.run().await
}
```

### HTML Setup

Create an `index.html` file in your static directory:

```html
<!DOCTYPE html>
<html>
<head>
    <title>My Web UI</title>
    <script src="webui.js"></script>
</head>
<body>
    <button id="hello-btn">Click Me!</button>
</body>
</html>
```

## Configuration

```rust
let config = WebUIConfig::default()
    .with_port(8080)                           // Custom port
    .with_title("My App".to_string())          // Window title
    .with_static_dir("./assets".to_string());  // Static files directory
```

## Event Handling

### Simple Click Handler

```rust
web_ui.bind_click("button-id", || {
    println!("Button clicked!");
}).await;
```

### Event Handler with Response

```rust
web_ui.bind_event("input-btn", "click", |event| {
    println!("Event data: {:?}", event.data);
    
    Ok(UIResponse {
        success: true,
        message: Some("Operation completed".to_string()),
        data: Some(serde_json::json!({ "result": "success" })),
        request_id: event.request_id,
    })
}).await;
```

## Examples

This repository includes several examples:

- **`hello`** - Basic button click example
- **`event_binding`** - Event handling with state management
- **`welcome`** - Welcome page example
- **`template`** - Template for creating new projects

Run examples with:

```bash
cargo run --example hello
cargo run --example event_binding
...
```

## Project Structure

```
static/
├── index.html      # Your main HTML file
├── style.css       # Optional CSS styles
├── app.js          # Your custom JavaScript
└── webui.js        # WebUI client library (needed to bind events)
```