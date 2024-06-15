<script lang="ts">
	let {
		showContent = $bindable(),
		class: className,
		children
	}: { showContent: boolean; class: string; children: any } = $props();
</script>

<button
	class={className}
	onclick={() => {
		showContent = !showContent;
	}}
>
	{@render children()}
</button>
