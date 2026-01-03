import type { GLTF } from 'three/addons/loaders/GLTFLoader.js';

export class Storer<T> {
	keys(): string[] {
		throw new Error('keys(0) must be implemented');
	}

	entries(): [string, T][] {
		throw new Error('entries(0) must be implemented');
	}

	consume(_key: string): T | undefined {
		throw new Error('consume(1) must be implemented');
	}

	get(_key: string): T | undefined {
		throw new Error('get(1) must be implemented');
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

export class ImageBitmapStorer extends Storer<ImageBitmap> {
	#store: { [key: string]: ImageBitmap } = {};

	keys(): string[] {
		return Object.keys(this.#store);
	}

	entries(): [string, ImageBitmap][] {
		return Object.entries(this.#store);
	}

	consume(key: string): ImageBitmap | undefined {
		const bmp = this.#store[key];
		delete this.#store[key];
		return bmp;
	}

	get(key: string): ImageBitmap | undefined {
		return this.#store[key];
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

export class GltfStorer extends Storer<GLTF> {
	#store: { [key: string]: GLTF } = {};

	keys(): string[] {
		return Object.keys(this.#store);
	}

	entries(): [string, GLTF][] {
		return Object.entries(this.#store);
	}

	consume(key: string): GLTF | undefined {
		const gltf = this.#store[key];
		delete this.#store[key];
		return gltf;
	}

	get(key: string): GLTF | undefined {
		return this.#store[key];
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
