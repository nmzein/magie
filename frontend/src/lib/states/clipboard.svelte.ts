import { SvelteSet } from 'svelte/reactivity';

export class Clipboard {
	mode: 'cut' | 'copy' | undefined = $state();
	items = $state(new SvelteSet<number>());
	isEmpty = $derived(this.items.size === 0);

	clear() {
		this.mode = undefined;
		this.items.clear();
	}
}
