import { setContext, getContext } from 'svelte';

const DROPDOWN_KEY = Symbol('DROPDOWN');

export type DropdownClasses = { trigger: string; list: string; item: string };

export function setDropdownState(classes?: DropdownClasses) {
	return setContext(DROPDOWN_KEY, new DropdownState(classes));
}

export function getDropdownState() {
	return getContext<ReturnType<typeof setDropdownState>>(DROPDOWN_KEY);
}

class DropdownState {
	private _show: boolean = $state(false);
	public classes: DropdownClasses;

	constructor(classes?: DropdownClasses) {
		this.classes = classes ?? { trigger: '', list: '', item: '' };
	}

	public toggle() {
		this._show = !this._show;
	}

	public close() {
		this._show = false;
	}

	get show() {
		return this._show;
	}
}
