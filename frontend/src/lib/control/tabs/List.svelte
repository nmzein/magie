<script lang="ts">
	let { class: className, children }: { class: string; children: any } = $props();
</script>

<div class={className}>
	{@render children()}
</div>
