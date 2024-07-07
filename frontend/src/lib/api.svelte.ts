/// None of the functions in the api module should alter
/// state directly. Instead, they should return data that
/// can be processed to update state in other parts of the app.

import { image, registry } from '$states';
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

import type { AnnotationLayer, MetadataLayer, Directory, UploaderSettings } from './types';

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

export const api = (() => {
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

	async function CreateDirectory(parent_id: number | undefined, name: string) {
		let data = await POST<{ parent_id: number | undefined; name: string }, void>(
			'Create Directory',
			DIRECTORY_CREATE_URL,
			{ parent_id, name }
		);

		// TODO: Have endpoint actually return new directory
		// TODO: and just insert it into the registry.
		if (data !== undefined) {
			registry.reload();
		}
	}

	async function SendUploadAssets(
		parentDirectoryID: number,
		imageFile: File,
		annotationsFile: File | undefined,
		settings: UploaderSettings
	) {
		const formData = new FormData();

		formData.append('parent_directory_id', parentDirectoryID.toString());
		formData.append('image_file', imageFile);
		if (annotationsFile) {
			formData.append('annotations_file', annotationsFile);
		}
		formData.append('generator_name', settings.generator);

		try {
			const response = await fetch(IMAGE_UPLOAD_URL, {
				method: 'POST',
				body: formData
			});

			if (response.ok) {
				api.GetRegistry();
			} else {
				console.error('Response Error <Upload>:', response.status, response.statusText);
			}
		} catch (error) {
			console.error('Fetch Error <Upload>:', error);
		}
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

	async function POST<Req, Resp>(name: string, url: string, data: Req) {
		try {
			const response = await fetch(url, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(data)
			});

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

	return {
		GetGenerators,
		GetRegistry,
		GetMetadata,
		GetAnnotations,
		CreateDirectory,
		SendUploadAssets
	};
})();

const _websocket = () => {
	let socket: WebSocket = $state(new WebSocket(WEBSOCKET_URL));

	function init() {
		socket.addEventListener('message', (event: MessageEvent) => {
			image.insertTile(event).catch((error) => {
				console.error('Tile Processing Error:', error);
			});
		});
	}

	function send(data: string): boolean {
		if (socket?.readyState !== WebSocket.OPEN) return false;
		socket.send(data);
		return true;
	}

	return { init, send };
};

export let websocket: ReturnType<typeof _websocket>;

export function ConnectWebSocket() {
	websocket = _websocket();
	websocket.init();
}
