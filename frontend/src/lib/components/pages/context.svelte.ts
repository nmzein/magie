import { setContext, getContext } from 'svelte';

const PAGES_KEY = Symbol('PAGES');

export type PagesClasses = { page: string; nav: string };

export function setPagesState(classes?: PagesClasses) {
	return setContext(PAGES_KEY, new PagesState(classes));
}

export function getPagesState() {
	return getContext<ReturnType<typeof setPagesState>>(PAGES_KEY);
}

class PagesState {
	private _currentPage: number = $state(0);
	private _numberOfPages: number = $state(0);
	private _firstPage: boolean = $derived(this._currentPage === 0);
	private _lastPage: boolean = $derived(this._currentPage === this._numberOfPages - 1);
	public classes: PagesClasses;

	constructor(classes?: PagesClasses) {
		this.classes = classes ?? { page: '', nav: '' };
	}

	public back() {
		if (this._firstPage) return;

		this._currentPage -= 1;
	}

	public next() {
		if (this._lastPage) return;

		this._currentPage += 1;
	}

	public registerPage(): number {
		const id = this._numberOfPages;
		this._numberOfPages += 1;
		return id;
	}

	get currentPage() {
		return this._currentPage;
	}

	get firstPage() {
		return this._firstPage;
	}

	get lastPage() {
		return this._lastPage;
	}
}
