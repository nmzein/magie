export class Cache<T> {
	get(key: string): T | undefined {
		throw new Error('get(1) must be implemented');
	}

	set(key: string, value: T): void {
		throw new Error('set(2) must be implemented');
	}

	has(key: string): boolean {
		throw new Error('has(1) must be implemented');
	}

	clear(): void {
		throw new Error('clear(0) must be implemented');
	}
}

export class ImageBitmapCache extends Cache<ImageBitmap> {
	cache: { [key: string]: ImageBitmap } = {};

	get(key: string): ImageBitmap | undefined {
		return this.cache[key];
	}

	set(key: string, value: ImageBitmap): void {
		this.cache[key] = value;
	}

	has(key: string): boolean {
		return key in this.cache;
	}

	clear(): void {
		for (const bmp of Object.values(this.cache)) {
			try {
				bmp.close();
			} catch {}
		}
		this.cache = {};
	}
}
