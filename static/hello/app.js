/**
 * Simple Web UI Example - JavaScript
 * Demonstrates basic button binding to a Rust function
 */

document.addEventListener('DOMContentLoaded', function() {
    webui.bindClick('hello-btn', function(response) {
        console.log('Hello button response:', response);
        if (response.success) {
            const button = document.getElementById('hello-btn');
            const originalText = button.textContent;
            button.textContent = 'Hello sent!';
            button.style.backgroundColor = '#28a745';
            
            setTimeout(() => {
                button.textContent = originalText;
                button.style.backgroundColor = '#007acc';
            }, 1000);
        }
    });
});
