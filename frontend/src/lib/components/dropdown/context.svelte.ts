import { setContext, getContext } from 'svelte';

const DROPDOWN_KEY = Symbol('DROPDOWN');

export function setDropdownState(classes: DropdownClasses = DEFAULT_DROPDOWN_CLASSES) {
	return setContext(DROPDOWN_KEY, new DropdownState(classes));
}

export function getDropdownState() {
	return getContext<ReturnType<typeof setDropdownState>>(DROPDOWN_KEY);
}

export type DropdownClasses = { trigger: string; list: string; item: string };
const DEFAULT_DROPDOWN_CLASSES: DropdownClasses = {
	trigger: '',
	list: '',
	item: ''
};

class DropdownState {
	private _show: boolean = $state(false);
	public classes: DropdownClasses;

	constructor(classes: DropdownClasses = DEFAULT_DROPDOWN_CLASSES) {
		this.classes = classes;
	}

	get show() {
		return this._show;
	}

	public toggle() {
		this._show = !this._show;
	}

	public close() {
		this._show = false;
	}
}
