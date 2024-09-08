import type { MetadataLayer, AnnotationLayer, ImageLayer, Image, TileRequest } from '$types';
import { http, websocket } from '$api';
import { transformer } from '$states';
import { defined } from '$helpers';

export class ImageViewer {
	public info: Image | undefined = $state();
	public metadata: MetadataLayer[] = $state([]);
	// TODO: Figure out why this scaling is needed.
	public width = $derived.by(() => {
		if (this.metadata.length === 0) return undefined;
		return this.metadata[0].width * 1.003;
	});

	public height = $derived.by(() => {
		if (this.metadata.length === 0) return undefined;
		return this.metadata[0].height * 1.003;
	});
	public levels: number = $derived(this.metadata.length);
	public tiles: ImageLayer[] = $state([]);
	public annotations: AnnotationLayer[] = $state([]);
	public initialised: boolean = $state(false);

	// Run as soon as metadata is parsed and loaded in GetMetadata.
	public async load(info: Image) {
		this.reset();

		this.info = info;

		const metadata = await http.GetMetadata(info.id);

		if (!defined(metadata) || metadata.length === 0) {
			this.reset();
			return;
		}
		this.metadata = metadata;

		this.tiles = new Array(this.levels).fill([]);

		for (let level = 0; level < this.levels; level++) {
			this.tiles[level] = new Array(metadata[level].rows)
				.fill(0)
				.map(() => new Array(metadata[level].cols).fill(new Image()));
		}

		// Ready to receive new image tiles.
		this.initialised = true;

		console.log('Initialised', this.initialised, this.tiles);

		// TODO: Move annotation metadata into GetMetadata request
		// TODO: and have the actual geometry info be requested by each
		// TODO: layer individually inside of a web worker.
		// const annotations = await http.GetAnnotations(this.info.id);
		// if (annotations === undefined) {
		// 	this.reset();
		// 	return;
		// }
		// this.annotations = annotations;
	}

	public async getTile(data: TileRequest): Promise<boolean> {
		if (!this.initialised) {
			return false;
		}

		return websocket.send(data);
	}

	public async insertTile(event: MessageEvent) {
		if (!this.initialised) return;

		const data: Blob = event.data;
		const arr = new Uint8Array(await data.arrayBuffer());

		const level = arr[0];
		const x = arr[1];
		const y = arr[2];

		const newTile = new Image();
		// Remove position and level values from start of array.
		const blob = new Blob([arr.slice(3)], { type: 'image/jpeg' });
		newTile.src = URL.createObjectURL(blob);
		this.tiles[level][y][x] = newTile;
	}

	reset() {
		this.initialised = false;
		this.info = undefined;
		this.metadata = [];
		this.tiles = [];
		this.annotations = [];
		transformer.resetScale();
	}
}
