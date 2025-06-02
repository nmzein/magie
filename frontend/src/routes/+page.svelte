<svelte:options runes={true} />

<script lang="ts">
	import '../app.css';
	import Stats from 'three/examples/jsm/libs/stats.module.js';
	import View from '$view/View.svelte';
	import ControlPanel from '$ui/ControlPanel.svelte';
	import { websocket } from '$api';
	import { views } from '$states';

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

{#each views as view, idx (view.state.id)}
	<View bind:view={views[idx]} />
{/each}

<ControlPanel />
