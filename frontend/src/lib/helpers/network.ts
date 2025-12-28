export type WebSocketManagerOptions = {
	url: string;
	maxReconnectAttempts?: number;
	minDelay?: number;
	maxDelay?: number;
	factor?: number;
	binaryType?: 'blob' | 'arraybuffer';
	onOpen?: (socket: WebSocket) => void;
	onMessage?: (event: MessageEvent) => void | Promise<void>;
	onError?: (error: Event) => void;
	onClose?: (event: CloseEvent, willReconnect: boolean) => void;
};

export class WebSocketManager {
	private _url: string;
	private _socket: WebSocket | null = null;
	private _reconnectAttempts = 0;
	private _maxReconnectAttempts = 5;
	private _minDelay = 1000;
	private _maxDelay = 30000;
	private _factor = 2;
	private _binaryType: 'blob' | 'arraybuffer' = 'arraybuffer';
	private _state: 'connecting' | 'reconnecting' | 'connected' | 'disconnected' = 'connecting';

	private _onOpen?: (socket: WebSocket) => void;
	private _onMessage?: (event: MessageEvent) => void | Promise<void>;
	private _onError?: (error: Event) => void;
	private _onClose?: (event: CloseEvent, willReconnect: boolean) => void;

	constructor(options: WebSocketManagerOptions) {
		this._url = options.url;

		options.maxReconnectAttempts !== undefined &&
			(this._maxReconnectAttempts = options.maxReconnectAttempts);
		options.minDelay !== undefined && (this._minDelay = options.minDelay);
		options.maxDelay !== undefined && (this._maxDelay = options.maxDelay);
		options.factor !== undefined && (this._factor = options.factor);
		options.binaryType && (this._binaryType = options.binaryType);

		this._onOpen = options.onOpen;
		this._onMessage = options.onMessage;
		this._onError = options.onError;
		this._onClose = options.onClose;
	}

	get state() {
		return this._state;
	}

	connect() {
		this._socket = new WebSocket(this._url);
		this._socket.binaryType = this._binaryType;

		this._socket.onopen = () => {
			this._state = 'connected';
			this._reconnectAttempts = 0;
			this._onOpen?.(this._socket!);
		};

		this._socket.onmessage = (event) => {
			this._onMessage?.(event);
		};

		this._socket.onerror = (error) => {
			if (this._onError) {
				this._onError(error);
			} else {
				console.error(error);
			}
		};

		this._socket.onclose = (event) => {
			const willReconnect = this._reconnectAttempts < this._maxReconnectAttempts;
			this._onClose?.(event, willReconnect);

			if (willReconnect) {
				this._state = 'reconnecting';
				this._reconnectAttempts++;
				const delay = this.getReconnectDelay(this._reconnectAttempts);
				setTimeout(() => this.connect(), delay);
			} else {
				this._state = 'disconnected';
			}
		};
	}

	// Exponential backoff.
	private getReconnectDelay(attempt: number): number {
		const expDelay = this._minDelay * Math.pow(this._factor, attempt - 1);
		return Math.min(expDelay, this._maxDelay);
	}

	send(data: string | ArrayBuffer | Blob | ArrayBufferView) {
		if (this._socket && this._socket.readyState === WebSocket.OPEN) {
			this._socket.send(data);
		}
	}

	disconnect() {
		if (this._socket) {
			this._socket.onopen = null;
			this._socket.onmessage = null;
			this._socket.onerror = null;
			this._socket.onclose = null;
			this._socket.close();
			this._socket = null;
		}
	}
}
