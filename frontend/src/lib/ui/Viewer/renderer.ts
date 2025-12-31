import type { Point, TiledImageAssetMetadata } from '$types';
import type { ImageBitmapCache } from './cache';
import type { TileIdentifier } from './worker';

const TILE_SIZE = 1024; // FIXME: Don't hardcode.
const TARGET_PIXELS_PER_LAYER_PIXEL = 1;

export class TiledImageRenderer {
	#metadata: TiledImageAssetMetadata;
	#offscreenCanvas: OffscreenCanvas;
	#ctx: OffscreenCanvasRenderingContext2D;
	#offset = { x: 0, y: 0 };
	#scale = 1;
	#currentLevel = 0;
	#cache: ImageBitmapCache;

	constructor(
		metadata: TiledImageAssetMetadata,
		offscreenCanvas: OffscreenCanvas,
		width: number,
		height: number,
		cache: ImageBitmapCache
	) {
		this.#metadata = metadata;
		this.#offscreenCanvas = offscreenCanvas;
		this.#cache = cache;

		this.#offscreenCanvas.width = width;
		this.#offscreenCanvas.height = height;

		const ctx = this.#offscreenCanvas.getContext('2d', { alpha: false });
		if (!ctx) throw new Error('Failed to create 2D rendering context');

		this.#ctx = ctx;
		this.#ctx.imageSmoothingEnabled = false; // TODO: Look into this option.
	}

	updateTransforms(dims: { width: number; height: number }, offset: Point, scale: number) {
		this.#offscreenCanvas.width = dims.width;
		this.#offscreenCanvas.height = dims.height;
		this.#offset = offset;
		this.#scale = scale;
	}

	#chooseLayer(): number {
		const layers = this.#metadata.layers;

		let bestLevel = this.#currentLevel;
		let bestError = Infinity;

		for (let i = 0; i < layers.length; i++) {
			const layer = layers[i];

			// How many base pixels one layer pixel represents
			const layerPixelScale = this.#metadata.width / layer.width;

			// How many screen pixels one layer pixel occupies
			const screenPixelsPerLayerPixel = this.#scale * layerPixelScale;

			const error = Math.abs(screenPixelsPerLayerPixel - TARGET_PIXELS_PER_LAYER_PIXEL);

			if (error < bestError) {
				bestError = error;
				bestLevel = i;
			}
		}

		this.#currentLevel = bestLevel;
		return bestLevel;
	}

	visible(): TileIdentifier[] {
		const level = this.#chooseLayer();
		const layer = this.#metadata.layers[level];
		if (!layer) return [];

		const layerPixelScale = this.#metadata.width / layer.width;

		// Tile size in screen space.
		const tileScreenSize = TILE_SIZE * this.#scale * layerPixelScale;

		const startX = Math.max(0, Math.floor(-this.#offset.x / tileScreenSize));
		const endX = Math.min(
			layer.cols - 1,
			Math.ceil((this.#offscreenCanvas.width - this.#offset.x) / tileScreenSize)
		);

		const startY = Math.max(0, Math.floor(-this.#offset.y / tileScreenSize));
		const endY = Math.min(
			layer.rows - 1,
			Math.ceil((this.#offscreenCanvas.height - this.#offset.y) / tileScreenSize)
		);

		const visible: TileIdentifier[] = [];

		for (let x = startX; x <= endX; x++) {
			for (let y = startY; y <= endY; y++) {
				visible.push({ level, x, y });
			}
		}

		return visible;
	}

	render(tiles: TileIdentifier[]): TileIdentifier[] {
		if (tiles.length === 0) return [];

		this.#ctx.setTransform(1, 0, 0, 1, 0, 0);
		this.#ctx.clearRect(0, 0, this.#offscreenCanvas.width, this.#offscreenCanvas.height);

		const level = tiles[0].level;
		const layer = this.#metadata.layers[level];

		const layerPixelScale = this.#metadata.width / layer.width;

		// Final transform:
		// layer pixels → base pixels → screen pixels
		this.#ctx.setTransform(
			this.#scale * layerPixelScale,
			0,
			0,
			this.#scale * layerPixelScale,
			this.#offset.x,
			this.#offset.y
		);

		for (const tile of tiles) {
			const key = `${tile.level}_${tile.x}_${tile.y}`;
			const bmp = this.#cache.get(key);
			if (!bmp) continue;

			this.#ctx.drawImage(bmp, tile.x * TILE_SIZE, tile.y * TILE_SIZE, TILE_SIZE, TILE_SIZE);
		}

		return tiles;
	}
}
