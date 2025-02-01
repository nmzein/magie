import { Context } from 'runed';

export const context = new Context<PagesState>('');

export type PagesClasses = { page: string; nav: string };

export class PagesState {
	#currentPage: number = $state(0);
	#numberOfPages: number = $state(0);
	#firstPage: boolean = $derived(this.#currentPage === 0);
	#lastPage: boolean = $derived(this.#currentPage === this.#numberOfPages - 1);
	classes: PagesClasses;

	constructor(initialPage?: number, classes?: PagesClasses) {
		this.#currentPage = initialPage ?? 0;
		this.classes = classes ?? { page: '', nav: '' };
	}

	back() {
		if (this.#firstPage) return;

		this.#currentPage -= 1;
	}

	next() {
		if (this.#lastPage) return;

		this.#currentPage += 1;
	}

	registerPage(): number {
		const id = this.#numberOfPages;
		this.#numberOfPages += 1;
		return id;
	}

	get currentPage() {
		return this.#currentPage;
	}

	get firstPage() {
		return this.#firstPage;
	}

	get lastPage() {
		return this.#lastPage;
	}
}
