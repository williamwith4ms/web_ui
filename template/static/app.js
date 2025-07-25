/**
 * Web UI Component Template - JavaScript
 * Template for creating WebUI JavaScript components with Rust backend integration
 * 
 * Usage:
 * 1. Copy this template to your component directory
 * 2. Customize the event handlers and UI interactions
 * 3. Ensure corresponding Rust functions are implemented
 */

document.addEventListener('DOMContentLoaded', function() {
    // DOM element references - customize based on your needs
    const responseDiv = document.getElementById('response');
    const statusDiv = document.getElementById('status');
    
    /**
     * Optional: WebSocket connection status monitoring
     * Useful for real-time applications or debugging connectivity
     */
    function updateConnectionStatus() {
        if (!statusDiv) return; // Skip if status div doesn't exist
        
        if (webui.useWebSocket && webui.websocket && webui.websocket.readyState === WebSocket.OPEN) {
            statusDiv.textContent = 'Connected via WebSocket';
            statusDiv.style.color = '#28a745';
            statusDiv.style.fontWeight = 'bold';
        } else {
            statusDiv.textContent = 'Using HTTP fallback';
            statusDiv.style.color = '#ffc107';
            statusDiv.style.fontWeight = 'normal';
        }
    }
    
    // Update status every second (remove if not needed)
    setInterval(updateConnectionStatus, 1000);
    updateConnectionStatus();
    
    /**
     * Button Click Handler Template
     * Replace 'your-button-id' with actual button ID
     * Replace 'your_rust_function' with actual Rust function name
     */
    webui.bindClick('your-button-id', function(response) {
        console.log('Button clicked, response:', response);
        
        if (response.success) {
            // Success handling
            if (responseDiv) {
                responseDiv.innerHTML = `<strong style="color: #28a745;">✓ Success:</strong> ${response.message || 'Operation completed successfully!'}`;
                
                // Optional: Display additional data if present
                if (response.data) {
                    responseDiv.innerHTML += `<br><strong>Data:</strong> <code>${JSON.stringify(response.data, null, 2)}</code>`;
                }
            }
            
            // Optional: Visual feedback on button
            const button = document.getElementById('your-button-id');
            if (button) {
                const originalText = button.textContent;
                const originalColor = button.style.backgroundColor;
                
                button.textContent = 'Success!';
                button.style.backgroundColor = '#28a745';
                
                setTimeout(() => {
                    button.textContent = originalText;
                    button.style.backgroundColor = originalColor || '#007acc';
                }, 1500);
            }
            
        } else {
            // Error handling
            if (responseDiv) {
                responseDiv.innerHTML = `<strong style="color: #dc3545;">✗ Error:</strong> ${response.message || 'Operation failed'}`;
                
                // Display error details if present
                if (response.error) {
                    responseDiv.innerHTML += `<br><small>${response.error}</small>`;
                }
            }
            
            console.error('Operation failed:', response);
        }
    });
    
    /**
     * Input Change Handler Template
     * Replace 'your-input-id' with actual input ID
     * Replace 'your_rust_input_handler' with actual Rust function name
     */
    webui.bindChange('your-input-id', function(response) {
        console.log('Input changed, response:', response);
        
        const input = document.getElementById('your-input-id');
        if (!input) return;
        
        if (response.success) {
            // Success visual feedback
            input.style.borderColor = '#28a745';
            input.style.boxShadow = '0 0 5px rgba(40, 167, 69, 0.3)';
            
            // Optional: Display validation message
            if (response.message && responseDiv) {
                responseDiv.innerHTML = `<small style="color: #28a745;">${response.message}</small>`;
            }
            
        } else {
            // Error visual feedback
            input.style.borderColor = '#dc3545';
            input.style.boxShadow = '0 0 5px rgba(220, 53, 69, 0.3)';
            
            // Display error message
            if (response.message && responseDiv) {
                responseDiv.innerHTML = `<small style="color: #dc3545;">${response.message}</small>`;
            }
        }
        
        // Reset visual feedback after delay
        setTimeout(() => {
            input.style.borderColor = '#ddd';
            input.style.boxShadow = 'none';
        }, 2000);
    });
    
    /**
     * Form Submit Handler Template
     * Replace 'your-form-id' with actual form ID
     * This uses webui.bindSubmit which matches the Rust backend setup
     */
    webui.bindSubmit('your-form-id', function(response) {
        console.log('Form submitted, response:', response);
        
        if (response.success) {
            // Success handling
            if (responseDiv) {
                responseDiv.innerHTML = `<strong style="color: #28a745;">✓ Form submitted successfully!</strong>`;
                if (response.message) {
                    responseDiv.innerHTML += `<br>${response.message}`;
                }
                
                // Optional: Display form response data if present
                if (response.data) {
                    responseDiv.innerHTML += `<br><strong>Response Data:</strong> <code>${JSON.stringify(response.data, null, 2)}</code>`;
                }
            }
            
            // Optional: Reset form on success
            const form = document.getElementById('your-form-id');
            if (form) {
                form.reset();
                
                // Optional: Visual feedback
                const submitBtn = form.querySelector('button[type="submit"]');
                if (submitBtn) {
                    const originalText = submitBtn.textContent;
                    const originalColor = submitBtn.style.backgroundColor;
                    
                    submitBtn.textContent = 'Success!';
                    submitBtn.style.backgroundColor = '#28a745';
                    
                    setTimeout(() => {
                        submitBtn.textContent = originalText;
                        submitBtn.style.backgroundColor = originalColor || '#28a745';
                    }, 2000);
                }
            }
            
        } else {
            // Error handling
            if (responseDiv) {
                responseDiv.innerHTML = `<strong style="color: #dc3545;">✗ Form submission failed:</strong> ${response.message || 'Unknown error'}`;
                
                // Display error details if present
                if (response.error) {
                    responseDiv.innerHTML += `<br><small>${response.error}</small>`;
                }
            }
            
            console.error('Form submission failed:', response);
        }
    });
    
    /**
     * Utility Functions
     */
    
    // Display a temporary message
    function showTemporaryMessage(message, type = 'info', duration = 3000) {
        if (!responseDiv) return;
        
        const colors = {
            success: '#28a745',
            error: '#dc3545',
            warning: '#ffc107',
            info: '#007acc'
        };
        
        responseDiv.innerHTML = `<strong style="color: ${colors[type] || colors.info};">${message}</strong>`;
        
        setTimeout(() => {
            responseDiv.innerHTML = '';
        }, duration);
    }
    
    // Format JSON data for display
    function formatJsonData(data) {
        return `<pre style="background: #f8f9fa; padding: 10px; border-radius: 5px; font-size: 12px; overflow-x: auto;">${JSON.stringify(data, null, 2)}</pre>`;
    }
    
    // Show loading state
    function setLoadingState(elementId, isLoading = true) {
        const element = document.getElementById(elementId);
        if (!element) return;
        
        if (isLoading) {
            element.disabled = true;
            element.style.opacity = '0.6';
            element.style.cursor = 'not-allowed';
        } else {
            element.disabled = false;
            element.style.opacity = '1';
            element.style.cursor = 'pointer';
        }
    }
    
    /**
     * Example: Advanced event binding with loading states
     */
    /*
    webui.bindClick('advanced-btn', function(response) {
        const btnId = 'advanced-btn';
        setLoadingState(btnId, true);
        
        if (response.success) {
            showTemporaryMessage('Operation completed successfully!', 'success');
            if (response.data) {
                responseDiv.innerHTML += formatJsonData(response.data);
            }
        } else {
            showTemporaryMessage(`Error: ${response.message}`, 'error');
        }
        
        setLoadingState(btnId, false);
    });
    */
    
    /**
     * Example: Real-time data updates
     */
    /*
    function startDataPolling() {
        setInterval(() => {
            webui.call('get_status_update', {}, function(response) {
                if (response.success && response.data) {
                    // Update UI with real-time data
                    const statusElement = document.getElementById('live-status');
                    if (statusElement) {
                        statusElement.textContent = response.data.status;
                    }
                }
            });
        }, 5000); // Poll every 5 seconds
    }
    
    // Uncomment to enable real-time updates
    // startDataPolling();
    */
});

/**
 * Custom Event Listeners (outside DOMContentLoaded)
 * Add any global event listeners or window-level handlers here
 */

// Example: Handle window resize
/*
window.addEventListener('resize', function() {
    console.log('Window resized:', window.innerWidth, 'x', window.innerHeight);
});
*/

// Example: Handle before page unload
/*
window.addEventListener('beforeunload', function(e) {
    // Cleanup or save state before leaving
    webui.call('cleanup_resources', {}, function(response) {
        console.log('Cleanup completed:', response);
    });
});
*/
