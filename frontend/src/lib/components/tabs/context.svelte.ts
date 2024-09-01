import { applyDefaults } from '$helpers';
import { setContext, getContext } from 'svelte';

const TAB_KEY = Symbol('TAB');

export type Modes = '0' | '1' | '<=1' | '>1';
export type TabClasses = {
	list?: string;
	trigger?: { regular?: string; active?: string; disabled?: string };
	content?: string;
};

export function setTabState(mode?: Modes, initialTab?: string, classes?: TabClasses) {
	return setContext(TAB_KEY, new TabState(mode, initialTab, classes));
}

export function getTabState() {
	return getContext<ReturnType<typeof setTabState>>(TAB_KEY);
}

class TabState {
	private _mode: Modes;
	public currentTab: string | undefined = $state();
	public classes: TabClasses;

	constructor(mode?: Modes, initialTab?: string, classes?: TabClasses) {
		this._mode = mode ?? '1';
		this.currentTab = initialTab ?? '';
		this.classes = applyDefaults(classes, {
			list: '',
			content: ''
		});
		this.classes.trigger = applyDefaults(classes?.trigger, {
			regular: '',
			active: '',
			disabled: ''
		});
	}

	get mode() {
		return this._mode;
	}
}
