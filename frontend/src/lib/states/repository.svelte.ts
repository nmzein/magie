import type { Directory, Store, Image } from '$types';
import { http } from '$api';
import { InitExplorerState } from '$states';
import { defined } from '$helpers';
import { SvelteMap } from 'svelte/reactivity';

export class RepositoryState {
	#registry: Store[] = $state([]);
	#stores = new SvelteMap<number, SvelteMap<number, Directory | Image>>();
	#storeNames = new SvelteMap<number, string>();
	#generators: string[] = $state([]);
	#decoders: string[] = $state(['Auto (default)']);
	#encoders: string[] = $state(['OMEZarr']);
	currentStore = new SvelteMap<number, Directory | Image>();

	get storeNames(): string[] {
		return Array.from(this.#storeNames.values());
	}

	get registry() {
		return this.#registry;
	}

	get generators() {
		return this.#generators;
	}

	get decoders() {
		return this.#decoders;
	}

	get encoders() {
		return this.#encoders;
	}

	constructor() {
		http.registry().then((registry) => {
			if (!defined(registry)) return;
			this.#registry = registry;

			registry.forEach((store) => {
				http.store.get(store.id).then((root) => {
					if (!defined(root)) return;
					const rootMap = new SvelteMap<number, Directory | Image>();
					root.forEach((item) => {
						rootMap.set(item.id, item);
					});
					this.#stores.set(store.id, rootMap);
					this.#storeNames.set(store.id, store.name);
				});
			});

			InitExplorerState();
		});

		http.generators().then((generators) => {
			if (!defined(generators)) return;
			this.#generators = generators;
		});
	}

	store(storeId: number): SvelteMap<number, Directory | Image> | undefined {
		return this.#stores.get(storeId);
	}

	storeName(storeId: number): string | undefined {
		return this.#storeNames.get(storeId);
	}
}
