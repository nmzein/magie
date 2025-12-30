export const NUM_FIELDS = 8;

export const Fields = {
	Dirty: 0,
	Width: 1,
	Height: 2,
	OffsetX: 3,
	OffsetY: 4,
	Scale: 5,
	Debug: {
		VisibleTiles: 6,
		PendingTiles: 7
	}
};

class Shared {
	shared!: Int32Array;

	init(shared: SharedArrayBuffer) {
		this.shared = new Int32Array(shared);
	}

	get(field: number): number {
		return this.shared[field];
	}

	set(field: number, value: number): void {
		this.shared[field] = value;
	}

	setDirty(): void {
		Atomics.store(this.shared, Fields.Dirty, 1);
	}

	setClean(): number {
		return Atomics.compareExchange(this.shared, Fields.Dirty, 1, 0);
	}
}

export let shared = new Shared();
