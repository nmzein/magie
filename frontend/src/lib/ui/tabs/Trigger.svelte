<script lang="ts">
	let {
		class: className,
		activeClass: activeClassName,
		value,
		currentTab = $bindable(),
		children
	}: {
		class: string;
		activeClass: string;
		value: string;
		currentTab: string | undefined;
		children: any;
	} = $props();
</script>

<button
	class="{className} {currentTab === value ? activeClassName : ''}"
	onclick={() => {
		if (currentTab !== value) {
			currentTab = value;
		} else {
			currentTab = undefined;
		}
	}}
>
	{@render children()}
</button>
