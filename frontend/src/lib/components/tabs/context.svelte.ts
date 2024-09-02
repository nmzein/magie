import { applyDefaults, type DeepRequired } from '$helpers';
import { setContext, getContext } from 'svelte';

const TAB_KEY = Symbol('TAB');

export type Modes = 'buttons' | 'tab' | 'collapsible-tab' | 'toggle';
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
	public classes: DeepRequired<TabClasses>;

	constructor(mode?: Modes, initialTab?: string, classes?: Partial<TabClasses>) {
		this._mode = mode ?? 'tab';
		this.currentTab = initialTab ?? '';
		this.classes = applyDefaults(classes, {
			list: '',
			trigger: {
				regular: '',
				active: '',
				disabled: ''
			},
			content: ''
		});
	}

	get mode() {
		return this._mode;
	}
}
