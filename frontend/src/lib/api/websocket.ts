import { BROADCAST_URL } from '$constants';
import { BinaryReader } from '$lib/helpers/codec';
import { WebSocketManager } from '$lib/helpers/network';
import { registry } from '$states';
import { DirectoryServerMsgTag, GeneralServerMsgTag } from '$types';

export let socket = new WebSocketManager({
	url: BROADCAST_URL,
	onMessage: receive,
	onError: (error) => console.error(error)
});

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
