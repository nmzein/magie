import { Context } from 'runed';
import type { Directory, Asset, Point, UploaderOptions } from '$types';
import { registry, repository, clipboard } from '$states';
import { http } from '$api';
import { StateHistory } from 'runed';
import { defined } from '$helpers';
import { SvelteSet } from 'svelte/reactivity';

export const ROOT_ID = 0;
export const BIN_ID = 1;

export const context = new Context<Explorer>('');

export class Explorer {
	position: Point = $state({ x: -1, y: -1 }); // TODO: Put this in separate Window class.
	#selected = new SvelteSet<number>();
	#pinned = new SvelteSet<number>();
	#storeId: number = $state(1); // TODO: This will be selected from a top level stores page.
	#store = $derived(registry.store(this.#storeId));
	#directoryId: number = $state(ROOT_ID); // TODO: Default to directory last opened by the user.
	#directory: Directory = $derived(this.#store?.get(this.#directoryId) as Directory);
	inBin: boolean = $derived(this.#directoryId === BIN_ID);
	#history!: StateHistory<number>;
	uploader = new Uploader();
	directoryCreator = new DirectoryCreator();
	path = $derived.by(() => {
		const path: [string, number][] = [];
		let currentDirectory = this.#directory;

		while (defined(currentDirectory.parentId)) {
			path.unshift([currentDirectory.name, currentDirectory.id]);
			currentDirectory = this.#store?.get(currentDirectory.parentId) as Directory;
		}

		const properties = registry.storeProperties(this.#storeId);
		if (!defined(properties)) return;
		path.unshift([properties.name, ROOT_ID]);

		return path;
	});
	searchQuery: string = $state('');
	items = $derived.by(() => {
		let children = this.#directory.children;

		const query = this.searchQuery.toLowerCase();
		if (!query) return children;

		children = this.#directory.children?.filter((child) =>
			this.#store?.get(child)?.name.toLowerCase().includes(query)
		);

		return children;
	});

	async upload() {
		await this.uploader.upload(this.#storeId, this.#directoryId);
	}

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

	get(id: number): Directory | Asset | undefined {
		return this.#store?.get(id);
	}

	// Defaults to going up to parent #directory.
	up(levels: number = 1) {
		if (this.#directoryId === ROOT_ID) return;

		const directory = this.#recurse(levels);
		if (!defined(directory)) return;

		this.deselectAll();
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

	gotoStore(storeId: number) {
		if (storeId === this.storeId && this.#directoryId === ROOT_ID) return;

		const directory = this.#store?.get(ROOT_ID);
		if (!defined(directory) || directory.type === 'Asset') return;

		this.deselectAll();
		this.#storeId = storeId;
		this.#directoryId = directory.id;
	}

	goto(id: number) {
		if (id === this.#directoryId) return;

		const directory = this.#store?.get(id);
		if (!defined(directory) || directory.type === 'Asset') return;

		this.deselectAll();
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
				case 'Asset':
					http.asset.remove(this.#storeId, id, mode);
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
						case 'Asset':
							http.asset.move(this.#storeId, id, this.#directoryId);
							break;
					}
				});
				this.deselectAll();
				clipboard.clear();
				break;
			}
			case 'copy': {
				console.log('TODO: Implement copy.');
				break;
			}
		}
	}
}

class Uploader {
	image: File | undefined = $state();
	annotations: File | undefined = $state();
	options: UploaderOptions = $state({
		name: '',
		encoder: repository.encoders[0],
		decoder: repository.decoders[0],
		generator: repository.generators[0],
		annotations: 'none'
	});
	imageSatisfied: boolean = $derived(defined(this.image) && this.options.name !== '');
	annotationsSatisfied: boolean = $derived(
		this.options.annotations === 'none' ||
			(this.options.annotations === 'provide' && defined(this.annotations))
	);
	currentPage: number = $state(0);
	#show = $state(false);

	get show() {
		return this.#show;
	}

	open() {
		this.#show = true;
	}

	close() {
		this.#show = false;
	}

	async upload(storeId: number, parentId: number) {
		this.#show = false;

		if (
			!defined(this.image) ||
			(['provide', 'generate'].includes(this.options.annotations) &&
				!defined(this.options.generator))
		)
			return;

		if (this.options.annotations === 'provide') {
			await http.asset.upload(storeId, parentId, this.image, this.annotations, this.options);
		} else {
			await http.asset.upload(storeId, parentId, this.image, undefined, this.options);
		}

		this.reset();
	}

	reset() {
		this.image = undefined;
		this.annotations = undefined;
		this.currentPage = 0;
		this.options.name = '';
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
