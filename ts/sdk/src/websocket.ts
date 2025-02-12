import { Data, WebSocket } from "ws";
import { EventEmitter } from "events";

export class WebsocketEventsListener extends EventEmitter {
  private websocketUrl: string;
  private token: string;
  private messageHandler: (message: any) => void;
  private websocket: WebSocket | null = null;
  private pingInterval: NodeJS.Timeout | null = null;
  private connected: boolean = false;

  constructor(
    websocketUrl: string,
    token: string,
    messageHandler: (message: any) => void
  ) {
    super();
    this.websocketUrl = websocketUrl;
    this.token = token;
    this.messageHandler = messageHandler;
  }

  /**
   * Connect to the WebSocket and start listening
   */
  async connect(): Promise<void> {
    console.log(`Connecting to ${this.websocketUrl}...`);

    return new Promise((resolve, reject) => {
      this.websocket = new WebSocket(this.websocketUrl, {
        headers: {
          Authorization: `Bearer ${this.token}`,
        },
      });

      this.websocket.on("open", () => {
        console.log("Connected to WebSocket.");
        this.connected = true;
        this.startListening();
        // this.startPinging();
        resolve();
      });

      this.websocket.on("error", (error: Error) => {
        console.error("WebSocket error:", error);
        reject(error);
      });

      this.websocket.on("close", () => {
        console.log("WebSocket connection closed.");
        this.connected = false;
        this.emit("close");
      });

      this.websocket.on("ping", () => {
        if (this.websocket) {
          this.websocket.pong();
        }
      });
    });
  }

  /**
   * Close the WebSocket connection and stop listening
   */
  async close(): Promise<void> {
    console.log("Closing connection...");

    if (this.pingInterval) {
      clearInterval(this.pingInterval);
      this.pingInterval = null;
    }

    if (this.websocket) {
      this.websocket.close();
      this.websocket.removeAllListeners();
      console.log("WebSocket closed.");
    }

    this.connected = false;
    this.emit("close");
  }

  /**
   * Start listening for incoming WebSocket messages
   */
  private startListening(): void {
    if (!this.websocket) return;

    this.websocket.on("message", async (data: Data) => {
      try {
        let message;
        if (data instanceof Buffer) {
          message = data.toString();
        } else if (Array.isArray(data)) {
          message = Buffer.concat(data).toString();
        } else {
          message = data;
        }

        await this.messageHandler(message);
      } catch (error) {
        console.error("Error handling message:", error);
        this.emit("error", error);
      }
    });
  }

  /**
   * Start sending periodic ping messages
   */
  private startPinging(): void {
    this.pingInterval = setInterval(() => {
      if (this.websocket && this.connected) {
        this.websocket.ping();
        console.log("Ping sent.");
      }
    }, 10000); // 10 second interval
  }

  /**
   * Send a string message over the WebSocket connection
   */
  async sendMessage(message: string): Promise<void> {
    if (!this.connected || !this.websocket) {
      throw new Error("Cannot send message: WebSocket is not connected.");
    }

    return new Promise((resolve, reject) => {
      this.websocket?.send(message, (error?: Error) => {
        if (error) {
          console.error("Error sending message:", error);
          reject(error);
        } else {
          console.log(`Sent message: ${message}`);
          resolve();
        }
      });
    });
  }

  /**
   * Static factory method to create and connect a new instance
   */
  static async create(
    websocketUrl: string,
    token: string,
    messageHandler: (message: any) => void
  ): Promise<WebsocketEventsListener> {
    const instance = new WebsocketEventsListener(
      websocketUrl,
      token,
      messageHandler
    );
    await instance.connect();
    return instance;
  }

  /**
   * Check if websocket is connected
   */
  isConnected(): boolean {
    return this.connected && this.websocket?.readyState === WebSocket.OPEN;
  }
}
