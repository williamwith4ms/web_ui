use web_ui::{WebUI, WebUIConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = WebUIConfig::default()
        .with_port(3030)
        .with_title("My Web App".to_string())
        .with_static_dir("./static".to_string());

    let web_ui = WebUI::new(config);
    web_ui.run().await?;
    
    Ok(())
}
