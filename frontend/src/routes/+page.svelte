<svelte:options runes={true} />

<script lang="ts">
	import '../app.css';
	import Stats from 'three/examples/jsm/libs/stats.module.js';
	import ControlPanel from '$ui/ControlPanel.svelte';
	import Viewer from '$ui/Viewer/Viewer.svelte';
	import { broadcast } from '$api';
	import { viewerManager } from '$states';

	$effect(() => {
		if (broadcast.state === 'disconnected') {
			broadcast.connect();
		}

		let stats = new Stats();
		document.body.appendChild(stats.dom);
		requestAnimationFrame(function loop() {
			stats.update();
			requestAnimationFrame(loop);
		});
	});
</script>

<div class="absolute flex h-full w-full flex-col flex-wrap">
	{#each viewerManager.viewers as [id, state] (state.instance.id)}
		<Viewer viewer={viewerManager.viewers.get(id)!} />
	{/each}
</div>

<ControlPanel />
