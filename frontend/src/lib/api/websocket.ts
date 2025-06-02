import {
	WEBSOCKET_URL,
	S_ERROR_TAG,
	S_DIRECTORY_CREATE_TAG,
	S_DIRECTORY_DELETE_TAG,
	S_DIRECTORY_MOVE_TAG,
	S_DIRECTORY_RENAME_TAG,
	S_DIRECTORY_TAG,
	S_TILE_TAG
} from '$constants';
import { views, registry } from '$states';
import type { Image2DView } from '$view/Image2D/types';

let socket: WebSocket;

export function send(data: Uint8Array): boolean {
	if (socket.readyState !== WebSocket.OPEN) return false;
	socket.send(data);
	return true;
}

async function receive(event: MessageEvent) {
	const data = new Uint8Array(await event.data.arrayBuffer());
	const dataView = new DataView(data.buffer);

	switch (dataView.getUint8(0)) {
		case S_ERROR_TAG:
			console.log('Error');
			break;
		case S_TILE_TAG:
			const storeId = dataView.getUint32(1);
			const id = dataView.getUint32(5);
			const level = dataView.getUint32(9);
			const x = dataView.getUint32(13);
			const y = dataView.getUint32(17);
			const tile = data.slice(29);

			views[0].state.insertTile(level, x, y, tile);
			break;
		case S_DIRECTORY_TAG:
			switch (dataView.getUint8(1)) {
				case S_DIRECTORY_CREATE_TAG: {
					const storeId = dataView.getUint32(6);
					const parentId = dataView.getUint32(10);
					const id = dataView.getUint32(14);
					const name = new TextDecoder().decode(data.slice(26)); // Skip 8 bytes encoding length.

					registry.add('Directory', storeId, parentId, id, name);
					break;
				}
				case S_DIRECTORY_DELETE_TAG: {
					const storeId = dataView.getUint32(6);
					const id = dataView.getUint32(10);

					registry.delete(storeId, id);
					break;
				}
				case S_DIRECTORY_MOVE_TAG: {
					const storeId = dataView.getUint32(6);
					const id = dataView.getUint32(10);
					const destinationId = dataView.getUint32(14);

					registry.move(storeId, id, destinationId);
					break;
				}
				case S_DIRECTORY_RENAME_TAG: {
					console.log('TODO: Implement rename.');
					break;
				}
			}
			break;
	}
}

export function connect() {
	socket = new WebSocket(WEBSOCKET_URL);
	socket.addEventListener('message', receive);
}
