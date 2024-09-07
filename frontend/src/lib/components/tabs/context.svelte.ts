import { applyDefaults, defined, type DeepRequired } from '$helpers';
import { setContext, getContext } from 'svelte';

const TAB_KEY = Symbol('TAB');

export type Modes = 'buttons' | 'tab' | 'collapsible-tab' | 'toggle';
export type TabClasses = {
	list?: string;
	trigger?: { base?: string; inactive?: string; active?: string; disabled?: string };
	content?: string;
};
const DEFAULT_CLASSES: DeepRequired<TabClasses> = {
	list: '',
	trigger: { base: '', inactive: '', active: '', disabled: '' },
	content: ''
};

export function setTabState(mode?: Modes, currentTab?: string, classes?: TabClasses) {
	return setContext(TAB_KEY, new TabState(mode, currentTab, classes));
}

export function getTabState() {
	return getContext<ReturnType<typeof setTabState>>(TAB_KEY);
}

class TabState {
	private _mode: Modes;
	public currentTab: string | undefined = $state();
	private _classes: DeepRequired<TabClasses> = $state(DEFAULT_CLASSES);

	constructor(mode?: Modes, currentTab?: string, classes?: Partial<TabClasses>) {
		this._mode = mode ?? 'tab';
		this.currentTab = currentTab ?? '';
		this._classes = applyDefaults(classes, DEFAULT_CLASSES);
	}

	get mode() {
		return this._mode;
	}

	get classes() {
		return this._classes;
	}

	set classes(classes: Partial<TabClasses> | undefined) {
		if (!defined(classes)) return;

		this._classes = applyDefaults(classes, this._classes);
	}
}
