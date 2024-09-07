import { DEFAULT_BOUND, type Bounds, DEFAULT_POINT, type Point } from '$types';
import { appendPx, defined } from '$helpers';
import { SvelteSet } from 'svelte/reactivity';

export class SelectionBox<T = any> {
	private _dragging: boolean = $state(false);
	private startPosition: Point = DEFAULT_POINT;
	private selectionBox: HTMLElement;
	private bounds: Bounds = $state(DEFAULT_BOUND);
	private parentBounds: DOMRect | Bounds;
	private intersected: SvelteSet<T> = new SvelteSet();

	constructor(selectionBox: HTMLElement, parentBounds: DOMRect | Bounds) {
		this.selectionBox = selectionBox;
		this.parentBounds = parentBounds;
	}

	public get dragging(): boolean {
		return this._dragging;
	}

	public start(cursor: Point) {
		if (this._dragging) return;

		this._dragging = true;

		this.startPosition = {
			x: cursor.x - this.parentBounds.left,
			y: cursor.y - this.parentBounds.top
		};

		this.bounds = {
			width: 0,
			height: 0,
			left: this.startPosition.x,
			top: this.startPosition.y
		};

		Object.assign(this.selectionBox.style, appendPx(this.bounds));
	}

	public update(cursor: Point) {
		if (!this._dragging) return;

		// Clamp current mouse position between 0 and parent's width/height.
		const currentX = Math.max(
			0,
			Math.min(cursor.x - this.parentBounds.left, this.parentBounds.width)
		);
		const currentY = Math.max(
			0,
			Math.min(cursor.y - this.parentBounds.top, this.parentBounds.height)
		);

		const width = currentX - this.startPosition.x;
		const height = currentY - this.startPosition.y;

		this.bounds = {
			width: Math.abs(width),
			height: Math.abs(height),
			left: width < 0 ? currentX : this.startPosition.x,
			top: height < 0 ? currentY : this.startPosition.y
		};

		Object.assign(this.selectionBox.style, appendPx(this.bounds));
	}

	public stop(): T[] {
		if (!this._dragging) return [];

		const intersected = Array.from(this.intersected);

		this.startPosition = DEFAULT_POINT;
		this.bounds = DEFAULT_BOUND;
		this.intersected.clear();

		this._dragging = false;

		return intersected;
	}

	public intersecting(target: DOMRect | Bounds, item: T | undefined = undefined): boolean {
		if (!this._dragging) return false;

		const targetLeft = target.left - this.parentBounds.left;
		const targetTop = target.top - this.parentBounds.top;

		const isIntersecting = !(
			this.bounds.left + this.bounds.width < targetLeft ||
			targetLeft + target.width < this.bounds.left ||
			this.bounds.top + this.bounds.height < targetTop ||
			targetTop + target.height < this.bounds.top
		);

		if (defined(item)) {
			const isTracked = this.intersected.has(item);

			if (isIntersecting && !isTracked) {
				this.intersected.add(item);
			} else if (!isIntersecting && isTracked) {
				this.intersected.delete(item);
			}
		}

		return isIntersecting;
	}
}
