import { BROADCAST_URL } from '$constants';
import { BinaryReader } from '$lib/helpers/codec';
import { registry } from '$states';
import { DirectoryServerMsgTag, GeneralServerMsgTag } from '$types';

let socket: WebSocket;

export function send(data: Uint8Array): boolean {
	if (socket.readyState !== WebSocket.OPEN) return false;
	socket.send(data);
	return true;
}

async function receive(event: MessageEvent) {
	const r = new BinaryReader(event.data);
	const tag = r.u8();

	switch (tag) {
		case GeneralServerMsgTag.Error:
			console.log('Error', r.string());
			break;
		case GeneralServerMsgTag.Tile:
			const subtag = r.u8();
			switch (subtag) {
				case DirectoryServerMsgTag.Create: {
					const storeId = r.u32();
					const parentId = r.u32();
					const directoryId = r.u32();
					const name = r.string();

					registry.add('Directory', storeId, parentId, directoryId, name);
					break;
				}
				case DirectoryServerMsgTag.Delete: {
					const storeId = r.u32();
					const directoryId = r.u32();

					registry.delete(storeId, directoryId);
					break;
				}
				case DirectoryServerMsgTag.Move: {
					const storeId = r.u32();
					const directoryId = r.u32();
					const destinationId = r.u32();

					registry.move(storeId, directoryId, destinationId);
					break;
				}
				case DirectoryServerMsgTag.Rename: {
					console.log('TODO: Implement rename.');
					break;
				}
			}
			break;
	}

	if (r.remaining() > 0) throw Error('Unexpected data remaining');
}

export function connect() {
	socket = new WebSocket(BROADCAST_URL);
	socket.binaryType = 'arraybuffer';
	socket.addEventListener('message', receive);
}
