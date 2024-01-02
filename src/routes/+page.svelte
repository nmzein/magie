<script lang="ts">
	import '../global.scss';
	import View from '$lib/components/view/View.svelte';
	import Controls from '$lib/components/control/Controls.svelte';
	import { onMount } from 'svelte';
	import { ConnectWebSocket, GetAnnotationGenerators, GetImagesList } from '$lib/api';

	onMount(() => {
		ConnectWebSocket();
		// TODO: Have the server update this dynamically.
		GetImagesList();
		GetAnnotationGenerators();
	});
</script>

<View />
<Controls />
