import {
	PUBLIC_HTTP_SCHEME,
	PUBLIC_WS_SCHEME,
	PUBLIC_DOMAIN,
	PUBLIC_BACKEND_PORT,
	// Directory routes.
	PUBLIC_DIRECTORY_CREATE_SUBDIR,
	PUBLIC_DIRECTORY_DELETE_SUBDIR,
	PUBLIC_DIRECTORY_RENAME_SUBDIR,
	PUBLIC_DIRECTORY_MOVE_SUBDIR,
	// Image routes.
	PUBLIC_IMAGE_UPLOAD_SUBDIR,
	PUBLIC_IMAGE_DELETE_SUBDIR,
	PUBLIC_IMAGE_MOVE_SUBDIR,
	PUBLIC_IMAGE_PROPERTIES_SUBDIR,
	PUBLIC_IMAGE_THUMBNAIL_SUBDIR,
	PUBLIC_IMAGE_ANNOTATIONS_SUBDIR,
	PUBLIC_IMAGE_TILES_SUBDIR,
	// General routes.
	PUBLIC_REGISTRY_SUBDIR,
	PUBLIC_GENERATORS_SUBDIR
} from '$env/static/public';
import { image, repository } from '$states';
import type { Properties, Directory, WebSocketRequest } from '$types';
import { defined } from '$helpers';

const BASE_URL = '://' + PUBLIC_DOMAIN + ':' + PUBLIC_BACKEND_PORT;
const HTTP_URL = PUBLIC_HTTP_SCHEME + BASE_URL;
const WS_URL = PUBLIC_WS_SCHEME + BASE_URL;

// Directory routes.
const DIRECTORY_CREATE_URL = new URL(HTTP_URL + PUBLIC_DIRECTORY_CREATE_SUBDIR);
const DIRECTORY_DELETE_URL = new URL(HTTP_URL + PUBLIC_DIRECTORY_DELETE_SUBDIR);
const DIRECTORY_RENAME_URL = new URL(HTTP_URL + PUBLIC_DIRECTORY_RENAME_SUBDIR);
const DIRECTORY_MOVE_URL = new URL(HTTP_URL + PUBLIC_DIRECTORY_MOVE_SUBDIR);

// Image routes.
const IMAGE_UPLOAD_URL = new URL(HTTP_URL + PUBLIC_IMAGE_UPLOAD_SUBDIR);
const IMAGE_DELETE_URL = new URL(HTTP_URL + PUBLIC_IMAGE_DELETE_SUBDIR);
const IMAGE_MOVE_URL = new URL(HTTP_URL + PUBLIC_IMAGE_MOVE_SUBDIR);
const IMAGE_PROPERTIES_URL = new URL(HTTP_URL + PUBLIC_IMAGE_PROPERTIES_SUBDIR);
const IMAGE_THUMBNAIL_URL = new URL(HTTP_URL + PUBLIC_IMAGE_THUMBNAIL_SUBDIR);
export const IMAGE_ANNOTATIONS_URL = new URL(HTTP_URL + PUBLIC_IMAGE_ANNOTATIONS_SUBDIR);
const WEBSOCKET_URL = new URL(WS_URL + PUBLIC_IMAGE_TILES_SUBDIR);

// General routes.
const REGISTRY_URL = new URL(HTTP_URL + PUBLIC_REGISTRY_SUBDIR);
const GENERATORS_URL = new URL(HTTP_URL + PUBLIC_GENERATORS_SUBDIR);

export const http = (() => {
	async function GetGenerators(): Promise<string[] | undefined> {
		return await GET(GENERATORS_URL);
	}

	async function GetRegistry(): Promise<Directory | undefined> {
		return await GET(REGISTRY_URL);
	}

	async function GetProperties(image_id: number): Promise<Properties | undefined> {
		return await GET(IMAGE_PROPERTIES_URL, [image_id]);
	}

	async function GetThumbnail(image_id: number): Promise<HTMLImageElement | undefined> {
		const blob: Blob | undefined = await GET(IMAGE_THUMBNAIL_URL, [image_id]);

		if (blob === undefined) return;

		const image = new Image();
		image.src = URL.createObjectURL(blob);
		return image;
	}

	async function CreateDirectory(parent: number, name: string) {
		const registry: Directory | undefined = await PUT(DIRECTORY_CREATE_URL, [], { name, parent });

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function DeleteDirectory(directory_id: number, mode: 'soft' | 'hard') {
		const registry: Directory | undefined = await DELETE(DIRECTORY_DELETE_URL, [directory_id], {
			mode
		});

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function DeleteImage(image_id: number, mode: 'soft' | 'hard') {
		const registry: Directory | undefined = await DELETE(IMAGE_DELETE_URL, [image_id], {
			mode
		});

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function MoveDirectory(target_id: number, dest_id: number) {
		const registry: Directory | undefined = await POST(DIRECTORY_MOVE_URL, { target_id, dest_id });

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function MoveImage(target_id: number, dest_id: number) {
		const registry: Directory | undefined = await POST(IMAGE_MOVE_URL, { target_id, dest_id });

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function SendUploadAssets(
		parent_directory_id: number,
		image_file: File,
		annotations_file: File | undefined,
		generator: string
	) {
		const formData = new FormData();

		formData.append('parent_directory_id', parent_directory_id.toString());
		formData.append('image_file', image_file);
		if (annotations_file !== undefined) {
			formData.append('annotations_file', annotations_file);
		}
		formData.append('generator_name', generator);

		const registry: Directory | undefined = await POST(IMAGE_UPLOAD_URL, formData, 'multipart');

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function GET<Resp = any>(
		_url: URL,
		paths?: (string | number)[],
		params?: Record<string, any>
	): Promise<Resp | undefined> {
		const url = constructUrl(_url, paths, params);

		try {
			const response = await fetch(url, { method: 'GET' });

			if (response.ok) {
				try {
					// Check if the generic type Resp is Blob and handle accordingly
					if (response.headers.get('Content-Type')?.includes('image/jpeg')) {
						const data = (await response.blob()) as Resp;
						return data;
					} else if (response.headers.get('Content-Type')?.includes('json')) {
						const data: Resp = await response.json();
						return data;
					}
				} catch (error) {
					console.error(`Parse Error [${url.pathname}]:`, error);
				}
			} else {
				console.error(`Response Error [${url.pathname}]:`, response.status, response.statusText);
			}
		} catch (error) {
			console.error(`Fetch Error [${url.pathname}]:`, error);
		}
	}

	async function PUT<Resp = any>(
		_url: URL,
		paths?: (string | number)[],
		params?: Record<string, any>
	): Promise<Resp | undefined> {
		const url = constructUrl(_url, paths, params);

		try {
			const response = await fetch(url, { method: 'PUT' });

			if (response.ok) {
				try {
					const data: Resp = await response.json();
					return data;
				} catch (error) {
					console.error(`Parse Error [${url}]:`, error);
				}
			} else {
				console.error(`Response Error [${url}]:`, response.status, response.statusText);
			}
		} catch (error) {
			console.error(`Fetch Error [${url}]:`, error);
		}
	}

	async function POST<Resp = any>(
		url: URL,
		body: any,
		contentType: 'json' | 'multipart' = 'json'
	): Promise<Resp | undefined> {
		try {
			let response: Response;

			switch (contentType) {
				case 'json':
					response = await fetch(url, {
						method: 'POST',
						headers: { 'Content-Type': 'application/json' },
						body: JSON.stringify(body)
					});
					break;
				case 'multipart':
					response = await fetch(url, {
						method: 'POST',
						body: body as FormData
					});
			}

			if (response.ok) {
				try {
					const data: Resp = await response.json();
					return data;
				} catch (error) {
					console.error(`Parse Error [${url.pathname}: ${JSON.stringify(body)}]:`, error);
				}
			} else {
				console.error(
					`Response Error [${url.pathname}: ${JSON.stringify(body)}]:`,
					response.status,
					response.statusText
				);
			}
		} catch (error) {
			console.error(`Fetch Error [${url.pathname}: ${JSON.stringify(body)}]:`, error);
		}
	}

	async function DELETE<Resp = any>(
		_url: URL,
		paths?: (string | number)[],
		params?: Record<string, any>
	): Promise<Resp | undefined> {
		const url = constructUrl(_url, paths, params);

		try {
			const response = await fetch(url, { method: 'DELETE' });

			if (response.ok) {
				try {
					const data: Resp = await response.json();
					return data;
				} catch (error) {
					console.error(`Parse Error [${url.pathname}]:`, error);
				}
			} else {
				console.error(`Response Error [${url.pathname}]:`, response.status, response.statusText);
			}
		} catch (error) {
			console.error(`Fetch Error [${url.pathname}]:`, error);
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

function constructUrl(_url: URL, paths: (string | number)[] = [], params?: Record<string, any>) {
	let url = new URL(_url);

	if (paths.length > 0) {
		const fullPath = paths.join('/');

		// Create a new URL object using the base URL and the full path
		url = new URL(`${_url}/${fullPath}`);
	}

	if (defined(params)) {
		for (const [key, value] of Object.entries(params)) {
			if (value !== undefined && value !== null) {
				url.searchParams.append(key, value.toString());
			}
		}
	}

	return url;
}
