import type { MetadataLayer, AnnotationLayer, ImageLayer, Image, TileRequest } from '$types';
import { http, websocket } from '$api';
import { transformer } from '$states';

export const ImageViewer = () => {
	let info: Image | undefined = $state();
	let metadata: MetadataLayer[] = $state([]);
	// TODO: Figure out why this scaling is needed.
	let width = $derived.by(() => {
		if (metadata.length === 0) return undefined;
		return metadata[0].width * 1.003;
	});

	let height = $derived.by(() => {
		if (metadata.length === 0) return undefined;
		return metadata[0].height * 1.003;
	});
	let levels: number = $derived(metadata.length);
	let tiles: ImageLayer[] = $state([]);
	let annotations: AnnotationLayer[] = $state([]);
	let initialised: boolean = $state(false);

	// Run as soon as metadata is parsed and loaded in GetMetadata.
	async function load(_info: Image) {
		initialised = false;
		reset();
		transformer.resetScale();

		info = _info;

		const _metadata = await http.GetMetadata(info.id);
		if (_metadata === undefined || _metadata.length === 0) {
			reset();
			return;
		}
		metadata = _metadata;

		tiles = new Array(levels).fill([]);

		for (let level = 0; level < levels; level++) {
			tiles[level] = new Array(metadata[level].rows)
				.fill(0)
				.map(() => new Array(metadata[level].cols).fill(new Image()));
		}

		// Ready to receive new image tiles.
		initialised = true;

		// TODO: Move annotation metadata into GetMetadata request
		// TODO: and have the actual geometry info be requested by each
		// TODO: layer individually inside of a web worker.
		const _annotations = await http.GetAnnotations(info.id);
		if (_annotations === undefined) {
			reset();
			return;
		}
		annotations = _annotations;
	}

	async function getTile(data: TileRequest): Promise<boolean> {
		if (!initialised) {
			return false;
		}

		return websocket.send(data);
	}

	async function insertTile(event: MessageEvent) {
		if (!initialised) return;

		const data: Blob = event.data;
		const arr = new Uint8Array(await data.arrayBuffer());

		const level = arr[0];
		const x = arr[1];
		const y = arr[2];

		const newTile = new Image();
		// Remove position and level values from start of array.
		const blob = new Blob([arr.slice(3)], { type: 'image/jpeg' });
		newTile.src = URL.createObjectURL(blob);
		tiles[level][y][x] = newTile;
	}

	function reset() {
		info = undefined;
		metadata = [];
		tiles = [];
		annotations = [];
	}

	return {
		get initialised() {
			return initialised;
		},
		get info() {
			return info;
		},
		get metadata() {
			return metadata;
		},
		get width() {
			return width;
		},
		get height() {
			return height;
		},
		get levels() {
			return levels;
		},
		get tiles() {
			return tiles;
		},
		get annotations() {
			return annotations;
		},
		load,
		getTile,
		insertTile
	};
};
