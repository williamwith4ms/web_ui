# WebUI Template

This template provides a starting point for creating new web UI components using the WebUI library.

## Getting Started

This template includes:

- **`template.rs`** - A complete example with various event handlers
- **`static/`** - Static assets directory with example HTML, CSS, and JavaScript
- Comprehensive examples of different event handling patterns

## Usage

1. Copy this template directory to start a new project
2. Modify `template.rs` to implement your specific functionality
3. Update the HTML in `static/index.html` to match your UI needs
4. Customize the CSS in `static/style.css` for your design
5. Add any custom JavaScript to `static/app.js`

## Template Features

The template includes examples of:

- Basic button click handlers
- Event handlers with response data
- Input field handling
- Custom JavaScript integration

## Running the Template

```bash
# From the root directory
cargo run --example template

# Or if you've copied this as a standalone project
cargo run
```

Then open your browser to `http://localhost:3030`

## Customization

### 1. Update Configuration

```rust
let config = WebUIConfig::default()
    .with_port(3030)  // Change this port if needed
    .with_title("Your Web UI Component".to_string())  // Customize your title
    .with_static_dir("./static".to_string());  // Point to your static files directory
```

### 2. Add Your Event Handlers

Replace the example event handlers with your own:

```rust
// Replace this example
web_ui.bind_click("your-button-id", || {
    println!("Button was clicked!");
    // Add your custom logic here
}).await;

// With your actual handler
web_ui.bind_click("save-btn", || {
    // Your save logic here
    save_data_to_file();
}).await;
```

### 3. Update the HTML

Modify `static/index.html` to include your UI elements:

```html
<!-- Replace the template buttons with your UI -->
<button id="save-btn">Save Data</button>
<input id="user-input" type="text" placeholder="Enter data...">
```

### 4. Add Custom Styling

Update `static/style.css` with your design:

```css
/* Add your custom styles */
.my-custom-class {
    background-color: #your-color;
    /* ... */
}
```

## File Structure

```
template/
├── README.md           # This file
├── template.rs         # Main Rust code with examples
└── static/
    ├── index.html      # HTML template with example UI
    ├── style.css       # CSS styling
    ├── app.js          # Custom JavaScript
    └── webui.js        # WebUI javascript library
```