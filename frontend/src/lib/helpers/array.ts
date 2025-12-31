export function* zip<A, B>(a: readonly A[], b: readonly B[]): IterableIterator<readonly [A, B]> {
	const len = Math.min(a.length, b.length);
	for (let i = 0; i < len; i++) {
		yield [a[i], b[i]];
	}
}
