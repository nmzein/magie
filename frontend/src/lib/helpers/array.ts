export function* zip<T extends readonly [unknown, unknown, ...unknown[]]>(
	...arrays: { [K in keyof T]: readonly T[K][] }
): IterableIterator<Readonly<T>> {
	const len = Math.min(...arrays.map((a) => a.length));

	for (let i = 0; i < len; i++) {
		yield arrays.map((a) => a[i]) as unknown as Readonly<T>;
	}
}
