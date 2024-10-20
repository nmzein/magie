import type { ImageLayer, Image, TileRequest, Properties } from '$types';
import { http, websocket } from '$api';
import { transformer } from '$states';
import { defined } from '$helpers';

export class ImageState {
	public info: Image | undefined = $state();
	public properties: Properties | undefined = $state();
	// TODO: Figure out why this scaling is needed.
	public width = $derived.by(() => {
		if (!defined(this.properties) || this.properties.metadata.length === 0) return undefined;
		return this.properties.metadata[0].width * 1.003;
	});

	public height = $derived.by(() => {
		if (!defined(this.properties) || this.properties.metadata.length === 0) return undefined;
		return this.properties.metadata[0].height * 1.003;
	});
	public levels: number = $derived(this.properties?.metadata.length ?? 0);
	public tiles: ImageLayer[] = $state([]);
	public initialised: boolean = $state(false);

	// TODO: Move into constructor and create new image by invoking new ImageViewer()
	// Run as soon as metadata is parsed and loaded in GetProperties.
	public async load(info: Image) {
		this.reset();

		const properties = await http.GetProperties(info.id);

		if (!defined(properties) || properties.metadata.length === 0) return;

		this.info = info;
		this.properties = properties;
		this.tiles = new Array(this.levels).fill([]);

		for (let level = 0; level < this.levels; level++) {
			this.tiles[level] = new Array(properties.metadata[level].rows)
				.fill(0)
				.map(() => new Array(properties.metadata[level].cols).fill(new Image()));
		}

		// Ready to receive new image tiles.
		this.initialised = true;
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
		this.properties = undefined;
		this.tiles = [];
		transformer.resetScale();
	}
}
