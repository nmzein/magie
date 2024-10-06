<script lang="ts">
	import '../app.css';
	import View from '$view/View.svelte';
	import ControlPanel from '$ui/ControlPanel.svelte';
	import { ConnectWebSocket } from '$api';
	import Stats from 'three/examples/jsm/libs/stats.module.js';

	$effect(() => {
		ConnectWebSocket();

		let stats = new Stats();
		document.body.appendChild(stats.dom);
		requestAnimationFrame(function loop() {
			stats.update();
			requestAnimationFrame(loop);
		});
	});
</script>

<View />
<ControlPanel />
