import {
	type BufferGeometry,
	Color,
	Mesh,
	MeshBasicMaterial,
	OrthographicCamera,
	Scene,
	Vector2,
	WebGLRenderer
} from 'three';
import type {
	AssetMetadata,
	Dimensions,
	GeometryAssetMetadata,
	Point,
	TiledImageAssetMetadata
} from '$types';
import type { BufferGeometryStorer, ImageBitmapStorer, Storer } from './storers';
import type { GeometryLayerIdentifier, TileIdentifier } from './worker';

const TILE_SIZE = 1024; // FIXME: Don't hardcode.
const TARGET_PIXELS_PER_LAYER_PIXEL = 1;

export class Renderer<T, S> {
	constructor(
		_metadata: AssetMetadata,
		_store: Storer<S>,
		_canvas: OffscreenCanvas,
		_ctx: OffscreenCanvasRenderingContext2D | WebGL2RenderingContext
	) {
		if (new.target === Renderer) {
			throw new Error('Renderer is abstract and cannot be instantiated');
		}
	}

	render(_dims: Dimensions, _offset: Point, _scale: number): T[] {
		throw new Error('render(3) must be implemented');
	}
}

export class TiledImageRenderer extends Renderer<TileIdentifier, ImageBitmap> {
	#metadata: TiledImageAssetMetadata;
	#storer: ImageBitmapStorer;
	#ctx: OffscreenCanvasRenderingContext2D;
	#currentLevel = 0;

	constructor(
		metadata: TiledImageAssetMetadata,
		storer: ImageBitmapStorer,
		canvas: OffscreenCanvas,
		ctx: OffscreenCanvasRenderingContext2D
	) {
		super(metadata, storer, canvas, ctx);

		this.#metadata = metadata;
		this.#storer = storer;
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
			const bmp = this.#storer.get(key);
			if (!bmp) continue;

			this.#ctx.drawImage(bmp, tile.x * TILE_SIZE, tile.y * TILE_SIZE, TILE_SIZE, TILE_SIZE);
		}

		return visible;
	}
}

export class DracoGeometryRenderer extends Renderer<GeometryLayerIdentifier, BufferGeometry> {
	#metadata: GeometryAssetMetadata;
	#storer: BufferGeometryStorer;
	#scene: Scene;
	#camera: OrthographicCamera;
	#renderer: WebGLRenderer;

	constructor(
		metadata: GeometryAssetMetadata,
		storer: BufferGeometryStorer,
		canvas: OffscreenCanvas,
		ctx: WebGL2RenderingContext
	) {
		super(metadata, storer, canvas, ctx);

		this.#metadata = metadata;
		this.#storer = storer;
		this.#scene = new Scene();
		this.#camera = new OrthographicCamera(0, metadata.width, 0, -1 * metadata.height, 0.1, 10);
		this.#camera.position.z = 1;

		this.#renderer = new WebGLRenderer({
			canvas,
			context: ctx,
			alpha: true,
			precision: 'highp',
			powerPreference: 'high-performance'
		});
	}

	render(dims: Dimensions, offset: Point, scale: number): GeometryLayerIdentifier[] {
		this.#camera.right = dims.width / scale;
		this.#camera.bottom = -dims.height / scale;

		this.#camera.position.x = -offset.x / scale;
		this.#camera.position.y = offset.y / scale;

		this.#camera.updateProjectionMatrix();

		const requests: GeometryLayerIdentifier[] = [];

		for (const layer of this.#metadata.layers) {
			if (!layer.visible) continue;

			let mesh = this.#scene.getObjectByName(layer.tag) as Mesh | undefined;

			if (!mesh) {
				const geometry = this.#storer.consume(layer.url);
				if (!geometry) {
					requests.push({ url: layer.url });
					continue;
				}

				mesh = new Mesh(geometry);
				mesh.name = layer.tag;
				mesh.visible = layer.visible;
				mesh.material = new MeshBasicMaterial({
					color: layer.fill,
					opacity: layer.opacity,
					transparent: true
				});

				mesh.position.set(0, 0, 0);
				this.#scene.add(mesh);
			} else if (
				mesh.visible !== layer.visible ||
				(mesh.material as MeshBasicMaterial).color.getHexString() !== layer.fill ||
				(mesh.material as MeshBasicMaterial).opacity !== layer.opacity
			) {
				mesh.visible = layer.visible;
				(mesh.material as MeshBasicMaterial).color = new Color(layer.fill);
				(mesh.material as MeshBasicMaterial).opacity = layer.opacity;
			}
		}

		// Only resize if dimensions changed.
		const rendererSize = this.#renderer.getSize(new Vector2());
		if (rendererSize.x !== dims.width || rendererSize.y !== dims.height) {
			this.#renderer.setSize(dims.width, dims.height, false);
		}

		this.#renderer.render(this.#scene, this.#camera);

		return requests;
	}
}
