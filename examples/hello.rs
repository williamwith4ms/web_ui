use web_ui::{WebUI, WebUIConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = WebUIConfig::default()
        .with_static_dir("./static/hello".to_string());
    let web_ui = WebUI::new(config);
    
    // Simple button click handler
    web_ui.bind_click("hello-btn", || {
        println!("Hello, World!");
    }).await;
    
    println!("Starting simple web UI on http://localhost:3030");
    web_ui.run().await
}
