/**
 * WebUI JavaScript Client Library
 * Provides easy binding of HTML elements to Rust backend functions
 */

class WebUIClient {
    constructor() {
        this.useWebSocket = true;
        this.websocket = null;
        this.eventQueue = [];
        this.pendingRequests = new Map();
        this.requestId = 0;
        this.reconnectDelay = 1000;
        this.maxReconnectDelay = 30000;
        this.currentReconnectDelay = this.reconnectDelay;
        
        this.init();
    }

    init() {
        this.setupWebSocket();
        
        // Fallback to HTTP if WebSocket fails
        setTimeout(() => {
            if (!this.websocket || this.websocket.readyState !== WebSocket.OPEN) {
                console.log('WebSocket connection failed, falling back to HTTP');
                this.useWebSocket = false;
            }
        }, 2000);
    }

    setupWebSocket() {
        try {
            const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
            const wsUrl = `${protocol}//${window.location.host}/ws`;
            
            this.websocket = new WebSocket(wsUrl);
            
            this.websocket.onopen = () => {
                console.log('WebSocket connected');
                this.useWebSocket = true;
                this.currentReconnectDelay = this.reconnectDelay;
                this.processEventQueue();
            };
            
            this.websocket.onmessage = (event) => {
                try {
                    const response = JSON.parse(event.data);
                    this.handleResponse(response);
                } catch (error) {
                    console.error('Error parsing WebSocket message:', error);
                }
            };
            
            this.websocket.onclose = () => {
                console.log('WebSocket disconnected');
                this.useWebSocket = false;
                this.scheduleReconnect();
            };
            
            this.websocket.onerror = (error) => {
                console.error('WebSocket error:', error);
                this.useWebSocket = false;
            };
        } catch (error) {
            console.error('Failed to create WebSocket:', error);
            this.useWebSocket = false;
        }
    }

    scheduleReconnect() {
        setTimeout(() => {
            console.log('Attempting to reconnect WebSocket...');
            this.setupWebSocket();
            this.currentReconnectDelay = Math.min(this.currentReconnectDelay * 1.5, this.maxReconnectDelay);
        }, this.currentReconnectDelay);
    }

    async sendEvent(elementId, eventType, data = {}) {
        const event = {
            element_id: elementId,
            event_type: eventType,
            data: data
        };

        if (this.useWebSocket && this.websocket && this.websocket.readyState === WebSocket.OPEN) {
            return this.sendEventViaWebSocket(event);
        } else {
            return this.sendEventViaHTTP(event);
        }
    }

    sendEventViaWebSocket(event) {
        return new Promise((resolve, reject) => {
            try {
                const requestId = ++this.requestId;
                event.request_id = requestId;
                
                this.pendingRequests.set(requestId, { resolve, reject });
                this.websocket.send(JSON.stringify(event));
                
                // Timeout after 10 seconds
                setTimeout(() => {
                    if (this.pendingRequests.has(requestId)) {
                        this.pendingRequests.delete(requestId);
                        reject(new Error('Request timeout'));
                    }
                }, 10000);
            } catch (error) {
                reject(error);
            }
        });
    }

    async sendEventViaHTTP(event) {
        try {
            const response = await fetch('/api/event', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(event)
            });
            
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            
            return await response.json();
        } catch (error) {
            console.error('HTTP request failed:', error);
            throw error;
        }
    }

    handleResponse(response) {
        if (response.request_id && this.pendingRequests.has(response.request_id)) {
            const { resolve } = this.pendingRequests.get(response.request_id);
            this.pendingRequests.delete(response.request_id);
            resolve(response);
        }
    }

    processEventQueue() {
        while (this.eventQueue.length > 0) {
            const event = this.eventQueue.shift();
            this.sendEventViaWebSocket(event.event)
                .then(event.resolve)
                .catch(event.reject);
        }
    }

    // Convenience methods for common events
    bindClick(elementId, callback) {
        return this.bindEvent(elementId, 'click', callback);
    }

    bindChange(elementId, callback) {
        return this.bindEvent(elementId, 'change', callback);
    }

    bindSubmit(elementId, callback) {
        return this.bindEvent(elementId, 'submit', callback);
    }

    bindEvent(elementId, eventType, callback) {
        const element = document.getElementById(elementId);
        if (!element) {
            console.error(`Element with ID '${elementId}' not found`);
            return;
        }

        element.addEventListener(eventType, async (domEvent) => {
            try {
                if (eventType === 'submit') {
                    domEvent.preventDefault();
                }

                const eventData = this.extractEventData(domEvent, element);
                
                const response = await this.sendEvent(elementId, eventType, eventData);
                
                if (callback) {
                    callback(response, domEvent);
                }
                
                if (!response.success && response.message) {
                    console.error('Backend error:', response.message);
                }
            } catch (error) {
                console.error('Error handling event:', error);
                if (callback) {
                    callback({ success: false, message: error.message }, domEvent);
                }
            }
        });
    }

    extractEventData(domEvent, element) {
        const data = {};
        
        if (element.value !== undefined) {
            data.value = element.value;
        }
        
        if (element.checked !== undefined) {
            data.checked = element.checked;
        }
        
        if (element.tagName === 'FORM') {
            const formData = new FormData(element);
            data.formData = {};
            for (let [key, value] of formData.entries()) {
                data.formData[key] = value;
            }
        }
        
        if (element.tagName === 'BUTTON' && (element.hasAttribute('data-collect') || element.id === 'greet-btn')) {
            const container = element.closest('div') || document;
            const inputs = container.querySelectorAll('input, select, textarea');
            inputs.forEach(input => {
                if (input.id) {
                    data[input.id] = input.value;
                }
            });
        }
        
        if (domEvent.clientX !== undefined) {
            data.mouse = {
                x: domEvent.clientX,
                y: domEvent.clientY
            };
        }
        
        if (domEvent.key !== undefined) {
            data.key = domEvent.key;
        }
        
        return data;
    }

    bindElements(bindings) {
        for (const binding of bindings) {
            this.bindEvent(binding.elementId, binding.eventType, binding.callback);
        }
    }
}

window.webui = new WebUIClient();

if (typeof module !== 'undefined' && module.exports) {
    module.exports = WebUIClient;
}
