import { setContext, getContext } from 'svelte';

const PAGES_KEY = Symbol('PAGES');

export function setPagesState(classes?: PagesClasses) {
	return setContext(PAGES_KEY, new PagesState(classes));
}

export function getPagesState() {
	return getContext<ReturnType<typeof setPagesState>>(PAGES_KEY);
}

export type PagesClasses = { trigger: string; list: string; item: string };
const DEFAULT_PAGES_CLASSES: PagesClasses = {
	trigger: '',
	list: '',
	item: ''
};

class PagesState {
	#currentPage: number = $state(0);
	#numberOfPages: number = $state(0);
	public firstPage: boolean = $derived(this.#currentPage === 0);
	public lastPage: boolean = $derived(this.#currentPage === this.#numberOfPages - 1);
	public classes: PagesClasses;

	constructor(classes?: PagesClasses) {
		this.classes = classes ?? DEFAULT_PAGES_CLASSES;
	}

	get currentPage() {
		return this.#currentPage;
	}

	public back() {
		if (this.firstPage) return;

		this.#currentPage -= 1;
	}

	public next() {
		if (this.lastPage) return;

		this.#currentPage += 1;
	}

	public registerPage(): number {
		const id = this.#numberOfPages;
		this.#numberOfPages += 1;
		return id;
	}
}
