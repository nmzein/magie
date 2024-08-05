import { setContext, getContext } from 'svelte';

const TAB_KEY = Symbol('TAB');

export function setTabState(
	mode: Modes = DEFAULT_MODE,
	initialTab: string = '',
	classes: TabClasses = DEFAULT_TAB_CLASSES
) {
	return setContext(TAB_KEY, new TabState(mode, initialTab, classes));
}

export function getTabState() {
	return getContext<ReturnType<typeof setTabState>>(TAB_KEY);
}

export type Modes = '0' | '1' | '<=1' | '>1';
const DEFAULT_MODE: Modes = '1';

type Classes = { regular: string; active: string; disabled: string };
const DEFAULT_CLASSES: Classes = { regular: '', active: '', disabled: '' };

export type TabClasses = { list: string; trigger: Classes; content: string };
const DEFAULT_TAB_CLASSES: TabClasses = {
	list: '',
	trigger: DEFAULT_CLASSES,
	content: ''
};

class TabState {
	public mode: Modes;
	public currentTab: string | undefined = $state();
	public classes: TabClasses;

	constructor(
		mode: '0' | '1' | '<=1' | '>1' = DEFAULT_MODE,
		initialTab: string = '',
		classes: TabClasses = DEFAULT_TAB_CLASSES
	) {
		this.currentTab = initialTab;
		this.mode = mode;
		this.classes = classes;
	}
}
