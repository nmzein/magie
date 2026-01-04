import { SvelteMap } from 'svelte/reactivity';
import { http } from '$api';
import { defined } from '$helpers';
import type { Entry, Store } from '$types';

type RegistryEntry = { properties: Store; items: SvelteMap<Entry['id'], Entry> };

export class Registry {
	#loaded = $state(false);
	#registry = new SvelteMap<Store['id'], RegistryEntry>();

	get loaded(): boolean {
		return this.#loaded;
	}

	get stores(): Store[] {
		if (this.#registry) {
			return Array.from(this.#registry.values()).map(({ properties }) => properties);
		}
		return [];
	}

	store(storeId: number): RegistryEntry | undefined {
		return this.#registry.get(storeId);
	}

	add(type: 'Directory' | 'Asset', storeId: number, parentId: number, id: number, name: string) {
		const store = this.#registry.get(storeId);
		if (!defined(store)) return;

		const parent = store.items.get(parentId);
		if (!defined(parent) || parent.type === 'Asset') return;

		if (type === 'Directory') {
			store.items.set(id, {
				type,
				storeId,
				parentId,
				id,
				name,
				children: []
			});
		} else {
			store.items.set(id, {
				type,
				storeId,
				parentId,
				id,
				name
			});
		}

		store.items.set(parent.id, {
			...parent,
			children: parent.children.concat(id)
		});
	}

	delete(storeId: number, id: number) {
		const store = this.store(storeId);
		if (!defined(store)) return;

		const target = store.items.get(id);
		if (!defined(target)) return;

		const parent = store.items.get(target.parentId);
		if (!defined(parent) || parent.type === 'Asset') return;

		store.items.delete(id);

		store.items.set(parent.id, {
			...parent,
			children: parent.children.filter((id) => id !== target.id)
		});
	}

	move(storeId: number, id: number, destinationId: number) {
		const store = this.store(storeId);
		if (!defined(store)) return;

		const target = store.items.get(id);
		if (!defined(target)) return;

		const parent = store.items.get(target.parentId);
		if (!defined(parent) || parent.type === 'Asset') return;

		const destination = store.items.get(destinationId);
		if (!defined(destination) || destination.type === 'Asset') return;

		store.items.set(id, {
			...target,
			parentId: destination.id
		});

		store.items.set(parent.id, {
			...parent,
			children: parent.children.filter((id) => id !== target.id)
		});

		store.items.set(destination.id, {
			...destination,
			children: destination.children.concat(id)
		});
	}

	constructor() {
		$effect.root(() => {
			$effect(() => {
				http.registry().then(async (registry) => {
					if (!defined(registry)) return;

					await Promise.all(
						registry.map(async (store) => {
							return await http.store.get(store.id).then((items) => {
								if (!defined(items)) return;

								const itemsMap: RegistryEntry['items'] = new SvelteMap();

								for (const item of items) {
									itemsMap.set(item.id, { ...item, storeId: store.id });
								}

								this.#registry.set(store.id, { properties: store, items: itemsMap });
							});
						})
					);

					this.#loaded = true;
				});
			});
		});
	}
}
