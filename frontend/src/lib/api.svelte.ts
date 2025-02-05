import {
	PUBLIC_HTTP_SCHEME,
	PUBLIC_WS_SCHEME,
	PUBLIC_DOMAIN,
	PUBLIC_BACKEND_PORT
} from '$env/static/public';
import { request, defined } from '$helpers';
import { images } from '$states';
import type { Properties, Directory, WebSocketRequest, UploaderOptions } from '$types';
import { GLTFLoader, type GLTF } from 'three/addons/loaders/GLTFLoader.js';

const BASE_URL = '://' + PUBLIC_DOMAIN + ':' + PUBLIC_BACKEND_PORT;
const HTTP_URL = PUBLIC_HTTP_SCHEME + BASE_URL + '/api';
const DIRECTORY_URL = new URL(HTTP_URL + '/directory');
const IMAGE_URL = new URL(HTTP_URL + '/image');
const WEBSOCKET_URL = new URL(PUBLIC_WS_SCHEME + BASE_URL + '/api/websocket');

const gltfLoader = new GLTFLoader();

export const http = (() => {
	async function registry(): Promise<Directory | undefined> {
		return await request.get({ url: `${HTTP_URL}/registry` });
	}

	async function generators(): Promise<string[] | undefined> {
		return await request.get({ url: `${HTTP_URL}/generators` });
	}

	const image = (() => {
		async function properties(id: number): Promise<Properties | undefined> {
			return await request.get({ url: `${IMAGE_URL}/${id}/properties` });
		}

		async function thumbnail(id: number): Promise<HTMLImageElement | undefined> {
			const blob: Blob | undefined = await request.get({ url: `${IMAGE_URL}/${id}/thumbnail` });

			if (!defined(blob)) return;

			const image = new Image();
			image.src = URL.createObjectURL(blob);
			return image;
		}

		async function annotations(image_id: number, annotations_layer_id: number): Promise<GLTF>
			return await gltfLoader.loadAsync(
				`${IMAGE_URL}/${image_id}/annotations/${annotations_layer_id}`
			);
		}

		async function remove(id: number, mode: 'soft' | 'hard') {
			await request.delete({
				url: `${IMAGE_URL}/${id}`,
				query: { mode }
			});
		}

		async function move(id: number, parent_id: number) {
			await request.patch({
				url: `${IMAGE_URL}/${id}`,
				body: { parent_id },
				type: 'json'
			});
		}

		async function upload(
			parent_id: number,
			image_file: File,
			annotations_file: File | undefined,
			options: UploaderOptions
		) {
			await request.post({
				url: `${IMAGE_URL}/${parent_id}/${options.name}`,
				body: {
					decoder: options.decoder,
					encoder: options.encoder,
					generator: options.generator,
					image_file,
					annotations_file
				},
				type: 'form'
			});
		}

		return { properties, thumbnail, annotations, remove, move, upload };
	})();

	const directory = (() => {
		async function create(parent_id: number, name: string) {
			// Using POST here because even though this is idempotent,
			// the client does not specify the created directory's id.
			await request.post({
				url: `${DIRECTORY_URL}/${parent_id}/${name}`
			});
		}

		async function remove(id: number, mode: 'soft' | 'hard') {
			await request.delete({
				url: `${DIRECTORY_URL}/${id}`,
				query: { mode }
			});
		}

		async function move(id: number, parent_id: number) {
			await request.patch({
				url: `${DIRECTORY_URL}/${id}`,
				body: { parent_id },
				type: 'json'
			});
		}

		return { create, remove, move };
	})();

	return { registry, generators, image, directory };
})();

export class WebSocketHandler {
	#socket: WebSocket;

	constructor(url: URL | string) {
		this.#socket = new WebSocket(url);
		this.#socket.addEventListener('message', this.#receive);
	}

	send(data: WebSocketRequest): boolean {
		if (this.#socket.readyState !== WebSocket.OPEN) return false;
		this.#socket.send(JSON.stringify(data));
		return true;
	}

	#receive(event: MessageEvent) {
		if (images[0]?.initialised) images[0].insertTile(event);
	}
}

export let websocket: WebSocketHandler;

export function ConnectWebSocket() {
	websocket = new WebSocketHandler(WEBSOCKET_URL);
}
