import type { GLTF } from 'three/addons/loaders/GLTFLoader.js';

export class Store<T> {
	get(_key: string): T | undefined {
		throw new Error('get(1) must be implemented');
	}

	getAll(): [string, T][] {
		throw new Error('getAll(0) must be implemented');
	}

	set(_key: string, _value: T): void {
		throw new Error('set(2) must be implemented');
	}

	has(_key: string): boolean {
		throw new Error('has(1) must be implemented');
	}

	clear(): void {
		throw new Error('clear(0) must be implemented');
	}
}

export class ImageBitmapStore extends Store<ImageBitmap> {
	#store: { [key: string]: ImageBitmap } = {};

	get(key: string): ImageBitmap | undefined {
		return this.#store[key];
	}

	getAll(): [string, ImageBitmap][] {
		return Object.entries(this.#store);
	}

	set(key: string, value: ImageBitmap): void {
		this.#store[key] = value;
	}

	has(key: string): boolean {
		return key in this.#store;
	}

	clear(): void {
		for (const bmp of Object.values(this.#store)) {
			try {
				bmp.close();
			} catch {}
		}
		this.#store = {};
	}
}

export class GltfStore extends Store<GLTF> {
	#store: { [key: string]: GLTF } = {};

	get(key: string): GLTF | undefined {
		return this.#store[key];
	}

	getAll(): [string, GLTF][] {
		return Object.entries(this.#store);
	}

	set(key: string, value: GLTF): void {
		this.#store[key] = value;
	}

	has(key: string): boolean {
		return key in this.#store;
	}

	clear(): void {
		this.#store = {};
	}
}
