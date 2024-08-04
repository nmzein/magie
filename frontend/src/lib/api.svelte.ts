/// None of the functions in the api module should alter
/// state directly. Instead, they should return data that
/// can be processed to update state in other parts of the app.

import { image, repository } from '$states';
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
	PUBLIC_IMAGE_METADATA_SUBDIR,
	PUBLIC_IMAGE_ANNOTATIONS_SUBDIR,
	PUBLIC_IMAGE_TILES_SUBDIR,
	// General routes.
	PUBLIC_REGISTRY_SUBDIR,
	PUBLIC_GENERATORS_SUBDIR
} from '$env/static/public';

import type {
	AnnotationLayer,
	MetadataLayer,
	Directory,
	UploaderSettings,
	WebSocketRequest
} from './types';

const URL = '://' + PUBLIC_DOMAIN + ':' + PUBLIC_BACKEND_PORT;
const HTTP_URL = PUBLIC_HTTP_SCHEME + URL;
const WS_URL = PUBLIC_WS_SCHEME + URL;

// Directory routes.
const DIRECTORY_CREATE_URL = HTTP_URL + PUBLIC_DIRECTORY_CREATE_SUBDIR;
const DIRECTORY_DELETE_URL = HTTP_URL + PUBLIC_DIRECTORY_DELETE_SUBDIR;
const DIRECTORY_RENAME_URL = HTTP_URL + PUBLIC_DIRECTORY_RENAME_SUBDIR;
const DIRECTORY_MOVE_URL = HTTP_URL + PUBLIC_DIRECTORY_MOVE_SUBDIR;

// Image routes.
const IMAGE_UPLOAD_URL = HTTP_URL + PUBLIC_IMAGE_UPLOAD_SUBDIR;
const IMAGE_DELETE_URL = HTTP_URL + PUBLIC_IMAGE_DELETE_SUBDIR;
const IMAGE_METADATA_URL = HTTP_URL + PUBLIC_IMAGE_METADATA_SUBDIR;
const IMAGE_ANNOTATIONS_URL = HTTP_URL + PUBLIC_IMAGE_ANNOTATIONS_SUBDIR;
const WEBSOCKET_URL = WS_URL + PUBLIC_IMAGE_TILES_SUBDIR;

// General routes.
const REGISTRY_URL = HTTP_URL + PUBLIC_REGISTRY_SUBDIR;
const GENERATORS_URL = HTTP_URL + PUBLIC_GENERATORS_SUBDIR;

export const http = (() => {
	async function GetGenerators() {
		return await GET<string[]>('Generators', GENERATORS_URL);
	}

	async function GetRegistry() {
		return await GET<Directory>('Registry', REGISTRY_URL);
	}

	async function GetMetadata(id: number) {
		return await POST<number, MetadataLayer[]>('Metadata', IMAGE_METADATA_URL, id);
	}

	async function GetAnnotations(id: number) {
		return await POST<number, AnnotationLayer[]>('Annotations', IMAGE_ANNOTATIONS_URL, id);
	}

	async function CreateDirectory(parent_id: number, name: string) {
		const registry = await POST<{ parent_id: number; name: string }, Directory>(
			'Create Directory',
			DIRECTORY_CREATE_URL,
			{ parent_id, name }
		);

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function DeleteDirectory(id: number, mode: 'soft' | 'hard') {
		const registry = await DELETE<Directory>(
			'Delete Directory',
			`${DIRECTORY_DELETE_URL}/${id}?mode=${mode}`
		);

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function MoveDirectory(target_id: number, dest_id: number) {
		const registry = await POST<{ target_id: number; dest_id: number }, Directory>(
			'Move Directory',
			DIRECTORY_MOVE_URL,
			{ target_id, dest_id }
		);

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function SendUploadAssets(
		parent_directory_id: number,
		image_file: File,
		annotations_file: File | undefined,
		settings: UploaderSettings
	) {
		const formData = new FormData();

		formData.append('parent_directory_id', parent_directory_id.toString());
		formData.append('image_file', image_file);
		if (annotations_file !== undefined) {
			formData.append('annotations_file', annotations_file);
		}
		formData.append('generator_name', settings.generator);

		const registry = await POST<FormData, Directory>(
			'Send Upload Assets',
			IMAGE_UPLOAD_URL,
			formData,
			'multipart'
		);

		if (registry === undefined) return;

		repository.registry = registry;
	}

	async function GET<Resp>(name: string, url: string) {
		try {
			const response = await fetch(url, { method: 'GET' });

			if (response.ok) {
				try {
					const data: Resp = await response.json();
					return data;
				} catch (error) {
					console.error(`Parse Error <${name}>:`, error);
				}
			} else {
				console.error(`Response Error <${name}>:`, response.status, response.statusText);
			}
		} catch (error) {
			console.error(`Fetch Error <${name}>:`, error);
		}
	}

	async function POST<Req, Resp>(
		name: string,
		url: string,
		data: Req,
		contentType: 'json' | 'multipart' = 'json'
	) {
		try {
			let response: Response;

			switch (contentType) {
				case 'json':
					response = await fetch(url, {
						method: 'POST',
						headers: { 'Content-Type': 'application/json' },
						body: JSON.stringify(data)
					});
					break;
				case 'multipart':
					response = await fetch(url, {
						method: 'POST',
						body: data as FormData
					});
			}

			if (response.ok) {
				try {
					const data: Resp = await response.json();
					return data;
				} catch (error) {
					console.error(`Parse Error <${name}: ${data}>:`, error);
				}
			} else {
				console.error(`Response Error <${name}: ${data}>:`, response.status, response.statusText);
			}
		} catch (error) {
			console.error(`Fetch Error <${name}: ${data}>:`, error);
		}
	}

	async function DELETE<Resp>(name: string, url: string) {
		try {
			const response = await fetch(url, { method: 'DELETE' });

			if (response.ok) {
				try {
					const data: Resp = await response.json();
					return data;
				} catch (error) {
					console.error(`Parse Error <${name}>:`, error);
				}
			} else {
				console.error(`Response Error <${name}>:`, response.status, response.statusText);
			}
		} catch (error) {
			console.error(`Fetch Error <${name}>:`, error);
		}
	}

	return {
		GetGenerators,
		GetRegistry,
		GetMetadata,
		GetAnnotations,
		CreateDirectory,
		DeleteDirectory,
		MoveDirectory,
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
