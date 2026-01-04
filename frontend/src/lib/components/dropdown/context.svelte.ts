import { applyDefaults } from '$helpers';
import { Context } from '$lib/helpers/context.ts';

export const context = new Context<DropdownState>();

const DEFAULT_CLASSES: DropdownClasses = { trigger: '', list: '', item: '' };

export type DropdownClasses = { trigger?: string; list?: string; item?: string };
export class DropdownState {
	show: boolean = $state(false);
	triggerElement: HTMLButtonElement | undefined = $state();
	listElement: HTMLDivElement | undefined = $state();
	#classes: DropdownClasses;

	constructor(classes?: DropdownClasses) {
		this.#classes = applyDefaults(classes, DEFAULT_CLASSES);
	}

	get classes() {
		return this.#classes;
	}

	toggle() {
		this.show = !this.show;
	}

	close() {
		this.show = false;
	}
}
