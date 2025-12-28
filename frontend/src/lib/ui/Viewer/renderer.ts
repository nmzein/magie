import type { Asset } from '$lib/states/viewer-manager.svelte';
import type { ImageBitmapCache } from './cache';
import type { TileIdentifier } from './worker';

// FIXME: Don't hardcode.
const TILE_SIZE = 1024;

export class Renderer {
	asset: Asset;
	offscreenCanvas: OffscreenCanvas;
	ctx: OffscreenCanvasRenderingContext2D;
	offset = { x: 0, y: 0 };
	scale = 1;

	constructor(asset: Asset, offscreenCanvas: OffscreenCanvas, width: number, height: number) {
		this.asset = asset;

		this.offscreenCanvas = offscreenCanvas;
		offscreenCanvas.width = width;
		offscreenCanvas.height = height;

		const ctx = offscreenCanvas.getContext('2d', { alpha: false });
		if (!ctx) throw new Error('Failed to create 2D rendering context');

		this.ctx = ctx;
		this.ctx.imageSmoothingEnabled = false; // TODO: Look into this option.

		// const lowestResolution =
		// 	asset.layers[this.#maxLevel].width * asset.layers[this.#maxLevel].height;

		// // Start at highest resolution (minLevel) and go till second lowest (maxLevel - 1).
		// for (let i = this.#minLevel; i < this.#maxLevel; i++) {
		// 	this.#scaleBreakpoints.push(
		// 		Math.sqrt((asset.layers[i].width * asset.layers[i].height) / lowestResolution)
		// 	);
		// }
	}

	updateCanvasDimensions(width: number, height: number) {
		this.offscreenCanvas.width = width;
		this.offscreenCanvas.height = height;
	}

	updateTransforms(offset: { x: number; y: number }, scale: number) {
		this.offset = offset;
		this.scale = scale;
	}

	calculateVisibleTiles(): TileIdentifier[] {
		const CTS = TILE_SIZE * this.scale;
		const layer = this.asset.layers[0];
		if (!layer) return [];

		const startX = Math.max(0, Math.floor(-this.offset.x / CTS));
		const endX = Math.min(
			layer.cols - 1,
			Math.ceil((this.offscreenCanvas.width - this.offset.x) / CTS)
		);
		const startY = Math.max(0, Math.floor(-this.offset.y / CTS));
		const endY = Math.min(
			layer.rows - 1,
			Math.ceil((this.offscreenCanvas.height - this.offset.y) / CTS)
		);

		const visible: TileIdentifier[] = [];
		const level = 0;

		for (let x = startX; x <= endX; x++) {
			for (let y = startY; y <= endY; y++) {
				visible.push({ level, x, y });
			}
		}

		return visible;
	}

	renderVisibleTiles(
		cache: ImageBitmapCache,
		tiles: TileIdentifier[],
		debugCallback?: (ctx: OffscreenCanvasRenderingContext2D) => void
	) {
		// Reset and clear.
		this.ctx.setTransform(1, 0, 0, 1, 0, 0);
		this.ctx.clearRect(0, 0, this.offscreenCanvas.width, this.offscreenCanvas.height);

		// Draw tiles.
		this.ctx.setTransform(this.scale, 0, 0, this.scale, this.offset.x, this.offset.y);
		for (const tile of tiles) {
			const key = `${tile.level}_${tile.x}_${tile.y}`;
			const bmp = cache.get(key);
			if (!bmp) continue;
			this.ctx.drawImage(bmp, tile.x * TILE_SIZE, tile.y * TILE_SIZE, TILE_SIZE, TILE_SIZE);
		}

		debugCallback?.(this.ctx);

		this.ctx.restore();
	}
}
