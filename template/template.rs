use web_ui::{WebUI, WebUIConfig, UIResponse};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration for your web UI
    let config = WebUIConfig::default()
        .with_port(3030)  // Change this port if needed
        .with_title("Your Web UI Component".to_string())  // Customize your title
        .with_static_dir("./template/static".to_string());  // Point to your static files directory

    // Create WebUI instance
    let web_ui = WebUI::new(config);

    // Example: Shared state (optional)
    let counter = Arc::new(AtomicU32::new(0));

    // Example 1: Simple button click handler
    web_ui.bind_click("your-button-id", || {
        println!("Button was clicked!");
        // Add your custom logic here
    }).await;

    // Example 2: Event handler with response data
    let counter_clone = counter.clone();
    web_ui.bind_event("your-button-id", "click", move |event| {
        let count = counter_clone.fetch_add(1, Ordering::SeqCst) + 1;
        println!("Event received: {:?}", event);
        
        // Return a response to the client
        Ok(UIResponse {
            success: true,
            message: Some(format!("Action completed successfully! Count: {}", count)),
            data: Some(serde_json::json!({ 
                "count": count,
            })),
            request_id: event.request_id,
        }) 
    }).await;

    // Example 3: Input field handler
    web_ui.bind_event("your-input-id", "change", |event| {
        println!("Input changed: {:?}", event.data);
        
        // Process input data
        let input_value = event.data.get("value")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        Ok(UIResponse {
            success: true,
            message: Some(format!("Input received: {}", input_value)),
            data: Some(serde_json::json!({ "processed_input": input_value.to_uppercase() })),
            request_id: event.request_id,
        })
    }).await;

    // Example 4: Form submission handler
    web_ui.bind_event("your-form-id", "submit", |event| {
        println!("Form submitted: {:?}", event.data);
        
        // Extract form data from the formData object
        let form_data = event.data.get("formData")
            .and_then(|v| v.as_object());
            
        let name = form_data
            .and_then(|fd| fd.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");
        
        let email = form_data
            .and_then(|fd| fd.get("email"))
            .and_then(|v| v.as_str())
            .unwrap_or("No email provided");

        // Process form data here
        // You can validate, save to database, send emails, etc.

        Ok(UIResponse {
            success: true,
            message: Some(format!("Thank you, {}! Form submitted successfully.", name)),
            data: Some(serde_json::json!({ 
                "user_name": name,
                "user_email": email,
            })),
            request_id: event.request_id,
        })
    }).await;

    // Example 5: Custom event with error handling
    web_ui.bind_event("custom-action", "custom", |event| {
        println!("Custom event received: {:?}", event);
        
        // Simulate some processing that might fail
        let data = event.data.get("required_field");
        
        if data.is_none() {
            return Ok(UIResponse {
                success: false,
                message: Some("Required field is missing".to_string()),
                data: None,
                request_id: event.request_id,
            });
        }

        // Process the data...
        
        Ok(UIResponse {
            success: true,
            message: Some("Custom action completed".to_string()),
            data: Some(serde_json::json!({ "result": "success" })),
            request_id: event.request_id,
        })
    }).await;

    // Start the web server
    println!("Starting web UI server on http://localhost:3030");
    web_ui.run().await?;
    
    Ok(())
}

// Helper functions (optional)

/// Example helper function for data validation
fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

/// Example helper function for data processing
fn process_user_data(name: &str, email: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    
    if !validate_email(email) {
        return Err("Invalid email format".to_string());
    }
    
    Ok(format!("User {} with email {} processed successfully", name, email))
}

/// Example helper function for generating responses
fn create_success_response(message: &str, data: serde_json::Value, request_id: Option<u32>) -> UIResponse {
    UIResponse {
        success: true,
        message: Some(message.to_string()),
        data: Some(data),
        request_id,
    }
}

fn create_error_response(error: &str, request_id: Option<u32>) -> UIResponse {
    UIResponse {
        success: false,
        message: Some(error.to_string()),
        data: None,
        request_id,
    }
}