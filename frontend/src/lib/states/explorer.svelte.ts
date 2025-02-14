import type { Directory, Image, Point } from '$types';
import { repository, clipboard } from '$states';
import { http } from '$api';
import { UploaderState } from './uploader.svelte';
import { StateHistory } from 'runed';
import { defined } from '$helpers';
import { SvelteSet } from 'svelte/reactivity';

export const ROOT_ID = 0;
export const BIN_ID = 1;

export class ExplorerState {
	position: Point = $state({ x: -1, y: -1 });
	#selected = new SvelteSet<number>();
	#pinned = new SvelteSet<number>();
	// TODO: This will be selected from a top level stores page.
	#storeId: number = $state(1);
	#store = $derived(repository.store(this.#storeId));
	#directoryId: number = $state(ROOT_ID); // TODO: Default to directory last opened by the user.
	#directory: Directory = $derived(this.#store?.get(this.#directoryId) as Directory);
	inBin: boolean = $derived(this.#directoryId === BIN_ID);
	#history!: StateHistory<number>;
	uploader = new UploaderState();
	directoryCreator = new DirectoryCreator();
	path = $derived.by(() => {
		const path: [string, number][] = [];
		let currentDirectory = this.#directory;

		while (defined(currentDirectory.parentId)) {
			path.unshift([currentDirectory.name, currentDirectory.id]);
			currentDirectory = this.#store?.get(currentDirectory.parentId) as Directory;
		}

		const storeName = repository.storeName(this.#storeId);
		if (!defined(storeName)) return;
		path.unshift([storeName, ROOT_ID]);

		return path;
	});

	get selected() {
		return this.#selected;
	}

	get pinned() {
		return this.#pinned;
	}

	get storeId() {
		return this.#storeId;
	}

	get directoryId() {
		return this.#directoryId;
	}

	get directory() {
		return this.#directory;
	}

	constructor() {
		$effect.root(() => {
			this.#history = new StateHistory(
				() => this.#directoryId,
				(r) => (this.#directoryId = r)
			);
		});
	}

	#recurse(levels: number): Directory {
		let currentDirectory = this.#directory;

		while (levels > 0 && defined(currentDirectory.parentId)) {
			currentDirectory = this.#store?.get(currentDirectory.parentId) as Directory;
			levels -= 1;
		}

		return currentDirectory;
	}

	get(id: number): Directory | Image | undefined {
		return this.#store?.get(id);
	}

	// Defaults to going up to parent #directory.
	up(levels: number = 1) {
		if (this.#directoryId === ROOT_ID) return;

		this.deselectAll();

		const directory = this.#recurse(levels);
		if (!defined(directory)) return;

		this.#directoryId = directory.id;
		this.select(this.#directoryId);
	}

	undo() {
		this.deselectAll();
		this.#history.undo();
		this.select(this.#directoryId);
	}

	redo() {
		this.deselectAll();
		this.#history.redo();
		this.select(this.#directoryId);
	}

	goto(id: number) {
		if (id === this.#directoryId) return;

		this.deselectAll();

		const directory = this.#store?.get(id);
		if (!defined(directory) || directory.type === 'File') return;

		this.#directoryId = directory.id;
	}

	isSelected(id: number): boolean {
		return this.#selected.has(id);
	}

	select(id: number) {
		this.#selected.add(id);
	}

	selectGroup(ids: number[]) {
		ids.forEach((id) => this.#selected.add(id));
	}

	selectAll() {
		this.selectGroup(this.#directory.children);
	}

	deselect(id: number) {
		this.#selected.delete(id);
	}

	deselectAll() {
		this.#selected.clear();
	}

	isPinned(id: number): boolean {
		return this.#pinned.has(id);
	}

	pinSelected() {
		this.#selected.forEach((id) => this.#pinned.add(id));
	}

	unpinSelected() {
		this.#selected.forEach((id) => this.#pinned.delete(id));
	}

	pin(id: number) {
		this.#pinned.add(id);
	}

	unpin(id: number) {
		this.#pinned.delete(id);
	}

	deleteSelected(mode: 'soft' | 'hard') {
		this.#selected.forEach((id) => {
			switch (this.#store?.get(id)?.type) {
				case 'Directory':
					http.directory.remove(this.#storeId, id, mode);
					break;
				case 'File':
					http.image.remove(this.#storeId, id, mode);
					break;
			}
		});
	}

	clipSelected(mode: 'cut' | 'copy') {
		clipboard.mode = mode;
		clipboard.items = new SvelteSet<number>([...this.selected]);
	}

	paste() {
		switch (clipboard.mode) {
			case 'cut': {
				clipboard.items.forEach((id) => {
					// Return if the item is already in the current #directory.
					if (this.#directory.children.some((i) => i === id)) {
						return;
					}

					switch (this.#store?.get(id)?.type) {
						case 'Directory':
							http.directory.move(this.#storeId, id, this.#directoryId);
							break;
						case 'File':
							http.image.move(this.#storeId, id, this.#directoryId);
							break;
					}
				});
				this.deselectAll();
				clipboard.clear();
				break;
			}
			case 'copy': {
				console.log('TODO!');
				break;
			}
		}
	}
}

class DirectoryCreator {
	#show = $state(false);

	get show() {
		return this.#show;
	}

	open() {
		this.#show = true;
	}

	async create(storeId: number, parentId: number, name: string) {
		this.#show = false;
		await http.directory.create(storeId, parentId, name);
	}

	close() {
		this.#show = false;
	}
}
