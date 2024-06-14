<script lang="ts">
	let { children }: { children: any } = $props();
</script>

<div>
	{@render children()}
</div>
