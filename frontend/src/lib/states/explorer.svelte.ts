import type { Image, Route, Directory, Navigable, Clipboard, Point } from '$types';
import { repository } from '$states';
import { http } from '$api';
import { UploaderState } from './uploader.svelte';
import { StateHistory } from 'runed';

export class ExplorerState {
	position: Point = $state({ x: -1, y: -1 });
	#clipboard: Clipboard = $state({
		mode: undefined,
		items: []
	});
	clipboardIsEmpty = $derived(this.#clipboard.items.length === 0);
	#selected: (Image | Directory)[] = $state([]);
	#pinned: Navigable[] = $state([]);
	// TODO: Default to directory last opened by the user.
	#directory: Navigable<Directory> = $state({
		path: [repository.registry!.subdirectories[1].name],
		route: [repository.registry!.subdirectories[1].id],
		data: repository.registry!.subdirectories[1]
	});
	// @ts-ignore
	#history: StateHistory<Navigable<Directory>>;
	uploader = new UploaderState();
	directoryCreator = new DirectoryCreator();

	get selected() {
		return this.#selected;
	}

	get pinned() {
		return this.#pinned;
	}

	get directory() {
		return this.#directory;
	}

	constructor() {
		$effect.root(() => {
			this.#history = new StateHistory(
				() => this.#directory,
				(r) => (this.#directory = r)
			);
		});
	}

	#findRoute(route: Route): Navigable<Directory> | undefined {
		let path = [];
		let currentDirectory = repository.registry!; // Initial root node.

		for (const id of route) {
			const directory = currentDirectory.subdirectories.find((value) => value.id === id);
			if (directory === undefined) return;
			currentDirectory = directory;
			path.push(currentDirectory.name);
		}

		return { path, route, data: currentDirectory };
	}

	// Defaults to going up to parent #directory.
	up(index: number = this.#directory.route.length - 2) {
		// Return if in data or bin directories.
		if (this.#directory.data.id === 1 || this.#directory.data.id === 2) return;

		this.deselectAll();

		const directory = this.#findRoute(this.#directory.route.slice(0, index + 1));
		if (!directory) return;

		this.#directory = directory;
		this.select(this.#directory.data);
	}

	undo() {
		this.deselectAll();
		this.#history.undo();
		this.select(this.#directory.data);
	}

	redo() {
		this.deselectAll();
		this.#history.redo();
		this.select(this.#directory.data);
	}

	routeTo(route: Route) {
		if (route.length === 0 || this.#directory.data.id === route[route.length - 1]) return;
		this.deselectAll();

		const directory = this.#findRoute(route);
		if (!directory) return;

		this.#directory = directory;
	}

	navigateTo(id: number) {
		if (this.#directory.data.id === id) return;

		this.deselectAll();

		const directory = this.#directory.data.subdirectories.find((value) => value.id === id);
		if (!directory) return;

		this.#directory.path.push(directory.name);
		this.#directory.route.push(directory.id);
		this.#directory.data = directory;
	}

	isSelected(item: Directory | Image): boolean {
		return this.selected.includes(item);
	}

	select(item: Directory | Image) {
		this.selected.push(item);
	}

	selectGroup(items: (Directory | Image)[]) {
		this.#selected = this.selected.concat(items);
	}

	selectAll() {
		this.#selected = this.#directory.data.subdirectories;
		this.#selected = this.#selected.concat(this.#directory.data.files);
	}

	deselect(item: Directory | Image) {
		this.#selected = this.#selected.filter((i) => i !== item);
	}

	deselectAll() {
		this.#selected = [];
	}

	isPinned(item: Directory | Image): boolean {
		return this.#pinned.some((i) => i.data.id === item.id);
	}

	pinSelected() {
		this.selected.forEach((item) => {
			if (this.#pinned.some((i) => i.data.id === item.id)) return;

			this.pin({
				path: this.#directory.path.concat(item.name),
				route: this.#directory.route.concat(item.id),
				data: item
			});
		});
	}

	unpinSelected() {
		this.selected.forEach((item) => {
			const index = this.#pinned.findIndex((i) => i.data.id === item.id);
			if (index === -1) return;
			this.#pinned.splice(index, 1);
		});
	}

	pin(item: Navigable) {
		// Check not already #pinned.
		if (this.#pinned.some((i) => i === item)) return;
		this.#pinned.push(item);
	}

	unpin(item: Navigable) {
		// Search for index of dir in #pinned.
		const index = this.#pinned.findIndex((i) => i === item);
		if (index === -1) return;
		this.#pinned.splice(index, 1);
	}

	deleteSelected(mode: 'soft' | 'hard') {
		this.selected.forEach((item) => {
			switch (item.type) {
				case 'directory':
					http.directory.remove(item.id, mode);
					break;
				case 'image':
					http.image.remove(item.id, mode);
					break;
			}
		});
	}

	clipSelected(mode: 'cut' | 'copy') {
		this.#clipboard = {
			mode,
			items: this.selected
		};
	}

	clearClipboard() {
		this.#clipboard = {
			mode: undefined,
			items: []
		};
	}

	paste() {
		if (this.#clipboard.mode === 'cut') {
			this.#clipboard.items.forEach((item) => {
				// Return if the item is already in the current #directory.
				if (this.#directory.data.subdirectories.some((i) => i.id === item.id)) {
					this.deselectAll();
					this.clearClipboard();
					return;
				}

				switch (item.type) {
					case 'directory':
						http.directory.move(item.id, this.#directory.data.id);
						break;
					case 'image':
						http.image.move(item.id, this.#directory.data.id);
						break;
				}
			});
			this.deselectAll();
			this.clearClipboard();
		} else if (this.#clipboard.mode === 'copy') {
			console.log('TODO!');
			// TODO
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

	async create(parentId: number, name: string) {
		this.#show = false;
		await http.directory.create(parentId, name);
	}

	close() {
		this.#show = false;
	}
}
