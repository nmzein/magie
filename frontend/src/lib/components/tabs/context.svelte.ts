import { applyDefaults, type DeepRequired } from '$helpers';
import { setContext, getContext } from 'svelte';

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

export function setTabState(id: string, mode?: Modes, currentTab?: string, classes?: TabClasses) {
	return setContext(id, new TabState(mode, currentTab, classes));
}

export function getTabState(id: string) {
	return getContext<ReturnType<typeof setTabState>>(id);
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
		this._classes = applyDefaults(classes, this._classes);
	}
}

export function setTabListState(_id: string) {
	let id = $state(_id);
	return setContext('TAB-LIST', {
		get id() {
			return id;
		}
	});
}

export function getTabListState() {
	return getContext<ReturnType<typeof setTabListState>>('TAB-LIST');
}
