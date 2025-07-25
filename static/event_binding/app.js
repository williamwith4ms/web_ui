/**
 * Event Binding Demo - JavaScript
 * Demonstrates advanced event binding with state management and data exchange
 */

document.addEventListener('DOMContentLoaded', function() {
    const responseDiv = document.getElementById('response');
    const statusDiv = document.getElementById('ws-status');
    
    function updateStatus() {
        if (webui.useWebSocket && webui.websocket && webui.websocket.readyState === WebSocket.OPEN) {
            statusDiv.textContent = 'Connected via WebSocket';
            statusDiv.style.color = '#28a745';
        } else {
            statusDiv.textContent = 'Using HTTP fallback';
            statusDiv.style.color = '#ffc107';
        }
    }
    
    setInterval(updateStatus, 1000);
    updateStatus();
    
    webui.bindClick('hello-btn', function(response) {
        if (response.success) {
            responseDiv.innerHTML = `<strong style="color: #28a745;">✓ Success:</strong> ${response.message || 'Hello button clicked!'}`;
            if (response.data) {
                responseDiv.innerHTML += `<br><strong>Data:</strong> <code>${JSON.stringify(response.data)}</code>`;
            }
        } else {
            responseDiv.innerHTML = `<strong style="color: #dc3545;">✗ Error:</strong> ${response.message}`;
        }
    });
    
    webui.bindClick('count-btn', function(response) {
        if (response.success) {
            const count = response.data ? response.data.count : 'Unknown';
            responseDiv.innerHTML = `<strong style="color: #007acc;">Click count:</strong> ${count}`;
            if (response.message) {
                responseDiv.innerHTML += `<br><em>${response.message}</em>`;
            }
        } else {
            responseDiv.innerHTML = `<strong style="color: #dc3545;">✗ Error:</strong> ${response.message}`;
        }
    });
    
    webui.bindClick('greet-btn', function(response) {
        if (response.success) {
            responseDiv.innerHTML = `<strong style="color: #28a745;">Greeting:</strong> ${response.message}`;
            if (response.data) {
                responseDiv.innerHTML += `<br><strong>Data:</strong> <code>${JSON.stringify(response.data)}</code>`;
            }
        } else {
            responseDiv.innerHTML = `<strong style="color: #dc3545;">✗ Error:</strong> ${response.message}`;
        }
    });
    
    webui.bindChange('name-input', function(response) {
        console.log('Name input changed:', response);
        if (response.success && response.message) {
            const input = document.getElementById('name-input');
            input.style.borderColor = '#28a745';
            setTimeout(() => {
                input.style.borderColor = '#ddd';
            }, 1000);
        }
    });
});
