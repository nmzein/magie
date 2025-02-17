import type { Directory, Store, Image } from '$types';
import { http } from '$api';
import { defined } from '$helpers';
import { SvelteMap } from 'svelte/reactivity';

export class Registry {
	#registry: SvelteMap<number, Store> | undefined = $state();
	#stores = new SvelteMap<number, SvelteMap<number, Directory | Image>>();

	get registry(): Store[] | undefined {
		if (this.#registry) {
			return Array.from(this.#registry.values());
		}
	}

	storeProperties(storeId: number): Store | undefined {
		return this.#registry?.get(storeId);
	}

	store(storeId: number): SvelteMap<number, Directory | Image> | undefined {
		return this.#stores.get(storeId);
	}

	constructor() {
		http.registry().then((registry) => {
			if (!defined(registry)) return;
			this.#registry = new SvelteMap<number, Store>();

			registry.forEach((store) => {
				http.store.get(store.id).then((root) => {
					if (!defined(root)) return;
					this.#registry!.set(store.id, store);

					const rootMap = new SvelteMap<number, Directory | Image>();
					root.forEach((item) => {
						rootMap.set(item.id, item);
					});
					this.#stores.set(store.id, rootMap);
				});
			});
		});
	}
}
