<svelte:options runes={true} />

<script lang="ts">
	import '../app.css';
	import Stats from 'three/examples/jsm/libs/stats.module.js';
	import ControlPanel from '$ui/ControlPanel.svelte';
	import Viewer from '$ui/Viewer/Viewer.svelte';
	import { websocket } from '$api';
	import { viewerManager } from '$states';

	$effect(() => {
		websocket.connect();

		let stats = new Stats();
		document.body.appendChild(stats.dom);
		requestAnimationFrame(function loop() {
			stats.update();
			requestAnimationFrame(loop);
		});
	});
</script>

{#each viewerManager.viewers as viewer (viewer.instance.asset.assetId)}
	<Viewer bind:instance={viewer.instance} />
{/each}

<ControlPanel />
