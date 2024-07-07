<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		class: className,
		activeClass: activeClassName,
		disabledClass: disabledClassName,
		mode = '=1',
		value,
		sideEffect = undefined,
		disabled = false,
		currentTab = $bindable(),
		children
	}: {
		class: string;
		activeClass: string;
		disabledClass: string;
		mode: '0' | '=1' | '<=1';
		value: string;
		sideEffect: (() => void) | undefined;
		disabled: boolean;
		currentTab: string | undefined;
		children: Snippet;
	} = $props();
</script>

<button
	class="
		{className}
		{!disabled && currentTab === value ? activeClassName : ''}
		{disabled ? disabledClassName : ''}
	"
	onclick={() => {
		if (sideEffect !== undefined) sideEffect();

		if (mode !== '0' && currentTab !== value) {
			// Only allow toggle if mode is '=1' or '<=1'.
			currentTab = value;
		} else if (mode === '<=1') {
			// Only allow untoggle if mode is '<=1'.
			currentTab = undefined;
		}
	}}
>
	{@render children()}
</button>
