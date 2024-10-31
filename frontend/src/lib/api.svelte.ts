import {
	PUBLIC_HTTP_SCHEME,
	PUBLIC_WS_SCHEME,
	PUBLIC_DOMAIN,
	PUBLIC_BACKEND_PORT
} from '$env/static/public';
import { image, repository } from '$states';
import type { Properties, Directory, WebSocketRequest } from '$types';

const BASE_URL = '://' + PUBLIC_DOMAIN + ':' + PUBLIC_BACKEND_PORT;
const HTTP_URL = PUBLIC_HTTP_SCHEME + BASE_URL + '/api';
const WS_URL = PUBLIC_WS_SCHEME + BASE_URL + '/api';
const DIRECTORY_URL = new URL(HTTP_URL + '/directory');
export const IMAGE_URL = new URL(HTTP_URL + '/image');
const WEBSOCKET_URL = new URL(WS_URL + '/websocket');

export const http = (() => {
	async function GetGenerators(): Promise<string[] | undefined> {
		return await GET(`${HTTP_URL}/generators`);
	}

	async function GetRegistry(): Promise<Directory | undefined> {
		return await GET(`${HTTP_URL}/registry`);
	}

	async function GetProperties(image_id: number): Promise<Properties | undefined> {
		return await GET(`${IMAGE_URL}/${image_id}/properties`);
	}

	async function GetThumbnail(image_id: number): Promise<HTMLImageElement | undefined> {
		const blob: Blob | undefined = await GET(`${IMAGE_URL}/${image_id}/thumbnail`);

		if (blob === undefined) return;

		const image = new Image();
		image.src = URL.createObjectURL(blob);
		return image;
	}

	async function CreateDirectory(parent_id: number, name: string) {
		// Using POST here because even though this is idempotent,
		// the client does not specify the created directory's id.
		const registry: Directory | undefined = await POST(`${DIRECTORY_URL}/${parent_id}/${name}`);

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function DeleteDirectory(id: number, mode: 'soft' | 'hard') {
		const registry: Directory | undefined = await DELETE(`${DIRECTORY_URL}/${id}?mode=${mode}`);

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function DeleteImage(id: number, mode: 'soft' | 'hard') {
		const registry: Directory | undefined = await DELETE(`${IMAGE_URL}/${id}?mode=${mode}`);

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function MoveDirectory(id: number, parent_id: number) {
		const registry: Directory | undefined = await PATCH(`${DIRECTORY_URL}/${id}`, { parent_id });

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function MoveImage(id: number, parent_id: number) {
		const registry: Directory | undefined = await PATCH(`${IMAGE_URL}/${id}`, { parent_id });

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function SendUploadAssets(
		parent_id: number,
		image_file: File,
		annotations_file: File | undefined,
		generator: string
	) {
		// TODO
		let name = '';
		const formData = new FormData();

		formData.append('image_file', image_file);
		if (annotations_file !== undefined) {
			formData.append('annotations_file', annotations_file);
		}
		formData.append('generator_name', generator);

		const registry: Directory | undefined = await POST(
			`${IMAGE_URL}/${parent_id}/${name}`,
			formData,
			'multipart'
		);

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function GET<Resp = any>(_url: string): Promise<Resp | undefined> {
		return await FETCH('GET', _url);
	}

	async function DELETE<Resp = any>(_url: string): Promise<Resp | undefined> {
		return await FETCH('DELETE', _url);
	}

	async function PATCH<Resp = any>(
		_url: string,
		body?: Record<string, any>,
		content_type: ContentType = 'application/json'
	): Promise<Resp | undefined> {
		return await FETCH('PATCH', _url, body, content_type);
	}

	async function POST<Resp = any>(
		_url: string,
		body?: Record<string, any> | FormData,
		content_type: ContentType = 'application/json'
	): Promise<Resp | undefined> {
		return await FETCH('POST', _url, body, content_type);
	}

	type ContentType = 'application/json' | 'multipart';

	async function FETCH<Resp = any>(
		method: string,
		_url: string,
		_body?: Record<string, any> | FormData,
		content_type?: ContentType
	): Promise<Resp | undefined> {
		const url = new URL(_url);

		try {
			let body: string | FormData | undefined;
			let headers: Record<string, string> | undefined;

			if (_body) {
				switch (content_type) {
					case 'application/json':
						body = JSON.stringify(_body);
						headers = { 'Content-Type': content_type };
						break;
					case 'multipart':
						body = _body as FormData;
						headers = { 'Content-Type': content_type };
						break;
				}
			}

			const response = await fetch(url, { method, headers, body });

			if (response.ok) {
				try {
					// Check if the generic type Resp is Blob and handle accordingly
					if (response.headers.get('Content-Type')?.includes('image/jpeg')) {
						const data = (await response.blob()) as Resp;
						return data;
					} else if (response.headers.get('Content-Type')?.includes('json')) {
						const data: Resp = await response.json();
						return data;
					} else {
						return undefined;
					}
				} catch (error) {
					console.error(`Parse Error [${url.pathname + url.search}]:`, error);
				}
			} else {
				console.error(
					`Response Error [${url.pathname + url.search}]:`,
					response.status,
					response.statusText
				);
			}
		} catch (error) {
			console.error(`Fetch Error [${url.pathname + url.search}]:`, error);
		}
	}

	return {
		GetGenerators,
		GetRegistry,
		GetProperties,
		GetThumbnail,
		CreateDirectory,
		DeleteDirectory,
		DeleteImage,
		MoveDirectory,
		MoveImage,
		SendUploadAssets
	};
})();

export class WebSocketState {
	private socket: WebSocket;

	constructor() {
		this.socket = new WebSocket(WEBSOCKET_URL);

		this.socket.addEventListener('message', (event: MessageEvent) => {
			image.insertTile(event).catch((error) => {
				console.error('Tile Processing Error:', error);
			});
		});
	}

	public send(data: WebSocketRequest): boolean {
		if (this.socket?.readyState !== WebSocket.OPEN) return false;
		this.socket.send(JSON.stringify(data));
		return true;
	}
}

export let websocket: WebSocketState;

export function ConnectWebSocket() {
	websocket = new WebSocketState();
}
