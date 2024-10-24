const SOCKET_URL = process.env.SOCKET_URL || 'ws://localhost:8080';

class WebSocketService {
  private socket: WebSocket | null = null;

  connect() {
    this.socket = new WebSocket(SOCKET_URL);

    this.socket.onopen = () => {
      console.log('WebSocket connection established');
    };

    this.socket.onmessage = (event) => {
      console.log('Message received from server:', event.data);
      // Handle incoming messages
    };

    this.socket.onclose = () => {
      console.log('WebSocket connection closed');
      // Optionally attempt to reconnect
    };

    this.socket.onerror = (error) => {
      console.error('WebSocket error:', error);
    };
  }

  sendMessage(message: string) {
    if (this.socket && this.socket.readyState === WebSocket.OPEN) {
      this.socket.send(message);
    } else {
      console.error('WebSocket is not open. Unable to send message:', message);
    }
  }

  close() {
    if (this.socket) {
      this.socket.close();
    }
  }
}

export default new WebSocketService();