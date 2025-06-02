<script lang="ts">
	import type { View } from '$lib/types/views.ts';
	import Image2DView from './Image2D/View.svelte';

	let { view = $bindable() }: { view: View } = $props();
</script>

{#if view.type === 'Image2D'}
	<Image2DView bind:view />
{/if}
