import { DEFAULT_BOUND, type Bounds, DEFAULT_POINT, type Point } from '$types';
import { appendPx, defined } from '$helpers';
import { SvelteSet } from 'svelte/reactivity';

export class SelectionBoxState<T = any> {
	#dragging: boolean = $state(false);
	#startPosition: Point = DEFAULT_POINT;
	#lastPosition: Point = DEFAULT_POINT;
	#intersected: SvelteSet<T> = new SvelteSet();
	#startScroll = { top: 0, left: 0 };
	#bounds: Bounds = $state(DEFAULT_BOUND);
	element: HTMLElement | undefined;
	parentBounds: DOMRect | undefined = $state();
	parentScroll = $state({ top: 0, left: 0 });
	show: boolean = $derived(this.#dragging && (this.#bounds.width > 5 || this.#bounds.height > 5));

	get dragging() {
		return this.#dragging;
	}

	start(cursor: Point) {
		if (this.#dragging || !defined(this.parentBounds) || !defined(this.element)) return;

		this.#dragging = true;

		this.#startPosition = {
			x: cursor.x - this.parentBounds.left,
			y: cursor.y - this.parentBounds.top
		};

		this.#startScroll = {
			top: this.parentScroll.top,
			left: this.parentScroll.left
		};

		this.#bounds = {
			width: 0,
			height: 0,
			left: this.#startPosition.x + this.parentScroll.left,
			top: this.#startPosition.y + this.parentScroll.top
		};

		Object.assign(this.element.style, appendPx(this.#bounds));
	}

	update(cursor: Point = this.#lastPosition) {
		if (!this.#dragging || !defined(this.parentBounds) || !defined(this.element)) return;

		this.#lastPosition = {
			x: cursor.x,
			y: cursor.y
		};

		const currentX = cursor.x - this.parentBounds.left + this.parentScroll.left;
		const currentY = cursor.y - this.parentBounds.top + this.parentScroll.top;

		const width = currentX - this.#startPosition.x - this.#startScroll.left;
		const height = currentY - this.#startPosition.y - this.#startScroll.top;

		this.#bounds = {
			width: Math.abs(width),
			height: Math.abs(height),
			left: width < 0 ? currentX : this.#bounds.left,
			top: height < 0 ? currentY : this.#bounds.top
		};

		Object.assign(this.element.style, appendPx(this.#bounds));
	}

	finish(): T[] {
		if (!this.#dragging) return [];

		const intersected = Array.from(this.#intersected);

		this.#startPosition = DEFAULT_POINT;
		this.#bounds = DEFAULT_BOUND;
		this.#intersected.clear();

		this.#dragging = false;

		return intersected;
	}

	intersecting(target: DOMRect | Bounds, item: T | undefined = undefined): boolean {
		if (!this.#dragging || !defined(this.parentBounds)) return false;

		const targetLeft = target.left - this.parentBounds.left + this.parentScroll.left;
		const targetTop = target.top - this.parentBounds.top + this.parentScroll.top;

		const isIntersecting = !(
			this.#bounds.left + this.#bounds.width < targetLeft ||
			targetLeft + target.width < this.#bounds.left ||
			this.#bounds.top + this.#bounds.height < targetTop ||
			targetTop + target.height < this.#bounds.top
		);

		if (defined(item)) {
			const isTracked = this.#intersected.has(item);

			if (isIntersecting && !isTracked) {
				this.#intersected.add(item);
			} else if (!isIntersecting && isTracked) {
				this.#intersected.delete(item);
			}
		}

		return isIntersecting;
	}
}
