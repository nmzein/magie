import { applyDefaults } from '$helpers';
import { setContext, getContext } from 'svelte';

const DROPDOWN_KEY = Symbol('DROPDOWN');

export type DropdownClasses = { trigger?: string; list?: string; item?: string };
const DEFAULT_CLASSES: DropdownClasses = { trigger: '', list: '', item: '' };

export function setDropdownState(classes?: DropdownClasses) {
	return setContext(DROPDOWN_KEY, new DropdownState(classes));
}

export function getDropdownState() {
	return getContext<ReturnType<typeof setDropdownState>>(DROPDOWN_KEY);
}

class DropdownState {
	public show: boolean = $state(false);
	private _classes: DropdownClasses;

	constructor(classes?: DropdownClasses) {
		this._classes = applyDefaults(classes, DEFAULT_CLASSES);
	}

	get classes() {
		return this._classes;
	}

	public toggle() {
		this.show = !this.show;
	}

	public close() {
		this.show = false;
	}
}
