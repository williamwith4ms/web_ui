use web_ui::{WebUI, WebUIConfig, UIResponse};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = WebUIConfig::default()
        .with_port(3030)
        .with_title("Event Binding Demo".to_string())
        .with_static_dir("./static/event_binding".to_string());

    // Create WebUI instance
    let web_ui = WebUI::new(config);

    // Shared counter for demonstrating state
    let click_counter = Arc::new(AtomicU32::new(0));

    // Bind a simple hello button
    web_ui.bind_click("hello-btn", || {
        println!("Hello button was clicked!");
    }).await;

    // Bind a counter button with state and response data
    let counter_clone = click_counter.clone();
    web_ui.bind_event("count-btn", "click", move |_event| {
        let count = counter_clone.fetch_add(1, Ordering::SeqCst) + 1;
        println!("Count button clicked {} times", count);
        
        Ok(UIResponse {
            success: true,
            message: Some(format!("Button clicked {} times", count)),
            data: Some(serde_json::json!({ "count": count })),
            request_id: None,
        })
    }).await;

    // Bind a greeting button that uses input data
    web_ui.bind_event("greet-btn", "click", |event| {
        println!("Greet button event data: {:?}", event.data);
        
        // Try to get the name from the input field
        let name = if let Some(name_value) = event.data.get("name-input") {
            name_value.as_str().unwrap_or("Anonymous")
        } else {
            // If not provided in event data, we'll need to handle it differently
            "Friend"
        };

        let greeting = format!("Hello, {}! Nice to meet you.", name);
        println!("Greeting: {}", greeting);

        Ok(UIResponse {
            success: true,
            message: Some(greeting),
            data: Some(serde_json::json!({ 
                "greeting_sent": true,
                "name": name 
            })),
            request_id: None,
        })
    }).await;

    // Bind input change events for real-time updates
    web_ui.bind_event("name-input", "change", |event| {
        if let Some(value) = event.data.get("value") {
            if let Some(name) = value.as_str() {
                println!("Name input changed to: {}", name);
                return Ok(UIResponse {
                    success: true,
                    message: Some(format!("Name updated to: {}", name)),
                    data: None,
                    request_id: None,
                });
            }
        }
        
        Ok(UIResponse {
            success: true,
            message: Some("Name input changed".to_string()),
            data: None,
            request_id: None,
        })
    }).await;

    println!("Starting Web UI server...");
    println!("Visit http://localhost:3030 to see the demo");
    println!("Try clicking the buttons to see the event binding in action!");

    // Start the server
    web_ui.run().await
}
