import { http } from '$api';
import type { Directory, Store, Asset } from '$types';
import { defined } from '$helpers';
import { SvelteMap } from 'svelte/reactivity';

export class Registry {
	#registry: SvelteMap<number, Store> | undefined = $state();
	#stores = new SvelteMap<number, SvelteMap<number, Directory | Asset>>();

	get registry(): Store[] | undefined {
		if (this.#registry) {
			return Array.from(this.#registry.values());
		}
	}

	storeProperties(storeId: number): Store | undefined {
		return this.#registry?.get(storeId);
	}

	store(storeId: number): SvelteMap<number, Directory | Asset> | undefined {
		return this.#stores.get(storeId);
	}

	add(type: 'Directory' | 'Asset', storeId: number, parentId: number, id: number, name: string) {
		const store = this.#stores.get(storeId);
		if (!defined(store)) return;

		const parent = store.get(parentId);
		if (!defined(parent) || parent.type === 'Asset') return;

		if (type === 'Directory') {
			store.set(id, {
				type,
				parentId,
				id,
				name,
				children: []
			});
		} else {
			store.set(id, {
				type,
				parentId,
				id,
				name
			});
		}

		store.set(parent.id, {
			...parent,
			children: parent.children.concat(id)
		});
	}

	delete(storeId: number, id: number) {
		const store = this.#stores.get(storeId);
		if (!defined(store)) return;

		const target = store.get(id);
		if (!defined(target)) return;

		const parent = store.get(target.parentId);
		if (!defined(parent) || parent.type === 'Asset') return;

		store.delete(id);

		store.set(parent.id, {
			...parent,
			children: parent.children.filter((id) => id !== target.id)
		});
	}

	move(storeId: number, id: number, destinationId: number) {
		const store = this.#stores.get(storeId);
		if (!defined(store)) return;

		const target = store.get(id);
		if (!defined(target)) return;

		const parent = store.get(target.parentId);
		if (!defined(parent) || parent.type === 'Asset') return;

		const destination = store.get(destinationId);
		if (!defined(destination) || destination.type === 'Asset') return;

		store.set(id, {
			...target,
			parentId: destination.id
		});

		store.set(parent.id, {
			...parent,
			children: parent.children.filter((id) => id !== target.id)
		});

		store.set(destinationId, {
			...destination,
			children: destination.children.concat(id)
		});
	}

	constructor() {
		$effect.root(() => {
			$effect(() => {
				http.registry().then((registry) => {
					if (!defined(registry)) return;
					this.#registry = new SvelteMap<number, Store>();
					registry.forEach((store) => {
						http.store.get(store.id).then((root) => {
							if (!defined(root)) return;
							this.#registry!.set(store.id, store);
							const rootMap = new SvelteMap<number, Directory | Asset>();
							root.forEach((item) => {
								rootMap.set(item.id, item);
							});
							this.#stores.set(store.id, rootMap);
						});
					});
				});
			});
		});
	}
}
