import type {
	AssetMetadata,
	Dimensions,
	GltfAssetMetadata,
	Point,
	TiledImageAssetMetadata
} from '$types';
import type { GLTF } from 'three/examples/jsm/Addons.js';
import type { Store, GltfStore, ImageBitmapStore } from './stores';
import type { GltfLayerIdentifier, TileIdentifier } from './worker';

const TILE_SIZE = 1024; // FIXME: Don't hardcode.
const TARGET_PIXELS_PER_LAYER_PIXEL = 1;

export class Renderer<T, S> {
	constructor(metadata: AssetMetadata, store: Store<S>, ctx: OffscreenCanvasRenderingContext2D) {
		if (new.target === Renderer) {
			throw new Error('Renderer is abstract and cannot be instantiated');
		}
	}

	render(dims: Dimensions, offset: Point, scale: number): T[] {
		throw new Error('render(3) must be implemented');
	}
}

export class TiledImageRenderer extends Renderer<TileIdentifier, ImageBitmap> {
	#metadata: TiledImageAssetMetadata;
	#ctx: OffscreenCanvasRenderingContext2D;
	#currentLevel = 0;
	#store: ImageBitmapStore;

	constructor(
		metadata: TiledImageAssetMetadata,
		store: ImageBitmapStore,
		ctx: OffscreenCanvasRenderingContext2D
	) {
		super(metadata, store, ctx);

		this.#metadata = metadata;
		this.#store = store;
		this.#ctx = ctx;
	}

	#chooseLayer(scale: number): number {
		const layers = this.#metadata.layers;

		let bestLevel = this.#currentLevel;
		let bestError = Infinity;

		for (let i = 0; i < layers.length; i++) {
			const layer = layers[i];

			// How many base pixels one layer pixel represents
			const layerPixelScale = this.#metadata.width / layer.width;

			// How many screen pixels one layer pixel occupies
			const screenPixelsPerLayerPixel = scale * layerPixelScale;

			const error = Math.abs(screenPixelsPerLayerPixel - TARGET_PIXELS_PER_LAYER_PIXEL);

			if (error < bestError) {
				bestError = error;
				bestLevel = i;
			}
		}

		this.#currentLevel = bestLevel;
		return bestLevel;
	}

	render(dims: Dimensions, offset: Point, scale: number): TileIdentifier[] {
		const level = this.#chooseLayer(scale);
		const layer = this.#metadata.layers[level];
		if (!layer) return [];

		const layerPixelScale = this.#metadata.width / layer.width;

		// Tile size in screen space.
		const tileScreenSize = TILE_SIZE * scale * layerPixelScale;

		const startX = Math.max(0, Math.floor(-offset.x / tileScreenSize));
		const endX = Math.min(layer.cols - 1, Math.ceil((dims.width - offset.x) / tileScreenSize));

		const startY = Math.max(0, Math.floor(-offset.y / tileScreenSize));
		const endY = Math.min(layer.rows - 1, Math.ceil((dims.height - offset.y) / tileScreenSize));

		const visible: TileIdentifier[] = [];

		for (let x = startX; x <= endX; x++) {
			for (let y = startY; y <= endY; y++) {
				visible.push({ level, x, y });
			}
		}

		if (visible.length === 0) return [];

		// FIXME: Shouldn't be done here.
		this.#ctx.setTransform(1, 0, 0, 1, 0, 0);
		this.#ctx.clearRect(0, 0, dims.width, dims.height);

		// FIXME: Shouldn't be done here.
		// Final transform:
		// layer pixels → base pixels → screen pixels
		this.#ctx.setTransform(
			scale * layerPixelScale,
			0,
			0,
			scale * layerPixelScale,
			offset.x,
			offset.y
		);

		for (const tile of visible) {
			const key = `${tile.level}_${tile.x}_${tile.y}`;
			const bmp = this.#store.get(key);
			if (!bmp) continue;

			this.#ctx.drawImage(bmp, tile.x * TILE_SIZE, tile.y * TILE_SIZE, TILE_SIZE, TILE_SIZE);
		}

		return visible;
	}
}

export class GltfRenderer extends Renderer<GltfLayerIdentifier, GLTF> {
	#metadata: GltfAssetMetadata;
	#ctx: OffscreenCanvasRenderingContext2D;
	#store: GltfStore;

	constructor(
		metadata: GltfAssetMetadata,
		store: GltfStore,
		ctx: OffscreenCanvasRenderingContext2D
	) {
		super(metadata, store, ctx);
		this.#metadata = metadata;
		this.#ctx = ctx;
		this.#store = store;
	}

	render(dims: Dimensions, offset: Point, scale: number): GltfLayerIdentifier[] {
		return [];
	}
}
