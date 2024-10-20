import type { Image, Route, Directory, Navigable, Clipboard, Point } from '$types';
import { defined } from '$helpers';
import { repository } from '$states';
import { http } from '$api';

export class ExplorerState {
	public positionSet = false;
	public position: Point = $state({ x: 0, y: 0 });
	// Selected directories (in main panel).
	public selected: (Image | Directory)[] = $state([]);
	// Clipboard for cut/copy/paste.
	public clipboard: Clipboard = $state({
		mode: undefined,
		items: []
	});
	public emptyClipboard = $derived(this.clipboard.items.length === 0);
	// Pinned directories (in side panel).
	public pinned: Navigable[] = $state([]);
	// Stack of directories to keep track of navigation.
	// TODO: Default to directory last opened by the user.
	private stack: Route[] = $state([[2]]);
	// Pointer to current directory in stack (for back and forward).
	private stackPointer = $state(0);
	// Route to current directory in stack pointed to by stackPointer.
	private _currentRoute = $derived.by(() => {
		return this.stack[this.stackPointer];
	});
	get currentRoute() {
		return this._currentRoute;
	}
	// Actual current directory information obtained from registry.
	public currentDirectory: Navigable<Directory> | undefined = $derived.by(() => {
		if (!defined(repository.registry) || !defined(this._currentRoute)) return;

		let path = [];
		let currentDirectory = repository.registry; // Initial root node.

		for (const id of this._currentRoute) {
			const potentialDir = currentDirectory.subdirectories.find((value) => value.id === id);
			if (potentialDir === undefined) return;
			currentDirectory = potentialDir;
			path.push(currentDirectory.name);
		}

		return { path, route: this._currentRoute, data: currentDirectory };
	});
	public showUploader: boolean = $state(false);
	public showDirectoryCreator: boolean = $state(false);

	constructor() {
		if (!defined(repository.registry)) return;

		this.stack = [[repository.registry.subdirectories[0].id]];
	}

	private insertIntoStack(route: Route) {
		// Slice stack to current pointer and insert new directory.
		this.stack = this.stack?.slice(0, this.stackPointer + 1);
		this.stack?.push(route);
		this.stackPointer += 1;
	}

	// Defaults to going up to parent directory.
	public up(index: number = this._currentRoute.length - 2) {
		if (this._currentRoute.length <= 1 || index === this._currentRoute.length - 1) return;

		this.deselectAll();

		const route = this._currentRoute.slice(0, index + 1);

		const current = this.currentDirectory?.data;
		this.insertIntoStack(route);

		if (!defined(current)) return;
		this.select(current);
	}

	public backward() {
		if (this.stackPointer <= 0) return;

		this.deselectAll();

		const current = this.currentDirectory?.data;
		this.stackPointer -= 1;

		if (!defined(current)) return;
		this.select(current);
	}

	public forward() {
		if (this.stackPointer >= this.stack.length - 1) return;

		this.deselectAll();

		const current = this.currentDirectory?.data;
		this.stackPointer += 1;

		if (!defined(current)) return;
		this.select(current);
	}

	public routeTo(route: Route) {
		if (route.length === 0 || this.currentDirectory?.data.id == route[route.length - 1]) return;

		this.deselectAll();

		this.insertIntoStack(route);
	}

	public navigateTo(id: number) {
		if (this.currentDirectory?.data.id === id) return;

		this.deselectAll();

		// Important: concat() creates a copy of current.
		const route = this._currentRoute.concat(id);

		this.insertIntoStack(route);
	}

	public isSelected(item: Directory | Image): boolean {
		return this.selected.includes(item);
	}

	public select(item: Directory | Image) {
		this.selected.push(item);
	}

	public selectAll() {
		if (!defined(this.currentDirectory)) return;
		this.selected = this.currentDirectory.data.subdirectories;
		this.selected = this.selected.concat(this.currentDirectory.data.files);
	}

	public deselect(item: Directory | Image) {
		this.selected = this.selected.filter((i) => i !== item);
	}

	public deselectAll() {
		this.selected = [];
	}

	public pinSelected() {
		this.selected.forEach((item) => {
			if (!defined(this.currentDirectory)) return;

			if (this.pinned.some((i) => i.data.id === item.id)) return;

			this.pin({
				path: this.currentDirectory.path.concat(item.name),
				route: this._currentRoute.concat(item.id),
				data: item
			});
		});
	}

	public pin(item: Navigable) {
		// Check not already pinned.
		if (this.pinned.some((i) => i === item)) return;
		this.pinned.push(item);
	}

	public unpin(item: Navigable) {
		// Search for index of dir in pinned.
		const index = this.pinned.findIndex((i) => i === item);
		if (index === -1) return;
		this.pinned.splice(index, 1);
	}

	public deleteSelected(mode: 'soft' | 'hard') {
		this.selected.forEach((item) => {
			switch (item.type) {
				case 'directory':
					http.DeleteDirectory(item.id, mode);
					break;
				case 'image':
					http.DeleteImage(item.id, mode);
					break;
			}
		});
	}

	public clipSelected(mode: 'cut' | 'copy') {
		this.clipboard = {
			mode,
			items: this.selected
		};
	}

	public clearClipboard() {
		this.clipboard = {
			mode: undefined,
			items: []
		};
	}

	public paste() {
		if (this.clipboard.mode === 'cut') {
			this.clipboard.items.forEach((item) => {
				// Return if the item is already in the current directory.
				if (
					!defined(this.currentDirectory) ||
					this.currentDirectory?.data.subdirectories.some((i) => i.id === item.id)
				) {
					this.deselectAll();
					this.clearClipboard();
					return;
				}

				switch (item.type) {
					case 'directory':
						http.MoveDirectory(item.id, this.currentDirectory?.data.id);
						break;
					case 'image':
						http.MoveImage(item.id, this.currentDirectory?.data.id);
						break;
				}
			});
			this.deselectAll();
			this.clearClipboard();
		} else if (this.clipboard.mode === 'copy') {
			console.log('TODO!');
			// TODO
		}
	}
}
