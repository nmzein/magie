export class ImageBitmapCache {
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
