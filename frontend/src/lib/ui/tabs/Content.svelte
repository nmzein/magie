<script lang="ts">
	let {
		class: className,
		value,
		currentTab,
		children
	}: { class: string; value: string; currentTab: string | undefined; children: any } = $props();
</script>

{#if value === currentTab}
	<div class={className}>
		{@render children()}
	</div>
{/if}
