import { WEBSOCKET_BASE_URL } from '$constants';
import { ByteReader } from '$lib/helpers/byte';
import { WebSocketManager } from '$lib/helpers/network';
import { registry } from '$states';
import { DirectoryServerMsgTag, GeneralServerMsgTag } from '$types';

export const socket = new WebSocketManager({
	url: `${WEBSOCKET_BASE_URL}/api/broadcast`,
	onMessage: (event: MessageEvent) => {
		const r = new ByteReader(event.data);
		const tag = r.u8();

		switch (tag) {
			case GeneralServerMsgTag.Error: {
				console.log('Error', r.string());
				break;
			}
			case GeneralServerMsgTag.Directory: {
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
		}

		if (r.remaining() > 0) throw Error('Unexpected data remaining');
	}
});
