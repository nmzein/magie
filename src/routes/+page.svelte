<script lang="ts">
	import '../global.scss';
	import View from '$lib/components/View.svelte';
	import Controls from '$lib/components/Controls.svelte';

	import type { PageData } from './$types';
	export let data: PageData;

	console.log(data);
</script>

<View />
<Controls />
