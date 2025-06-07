import { DEFAULT_POINT, type Point } from '$types';

export type ContextMenuItem =
	| { name: string; action?: () => void; disabled?: boolean; hidden?: boolean; shortcut?: string }
	| 'separator';

export class ContextMenu {
	show = $state(false);
	#position: Point = $state(DEFAULT_POINT);
	#items: ContextMenuItem[] = $state([]);

	get position() {
		return this.#position;
	}

	get items() {
		return this.#items;
	}

	open(position: Point, items: ContextMenuItem[]) {
		this.show = true;
		this.#position = position;
		this.#items = items;
	}

	close() {
		this.show = false;
	}
}
