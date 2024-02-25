<script lang="ts">
	import { stores } from '$stores';
	import Directory from '$control/Directory.svelte';
	import type { DirectoryNode, Image } from '$lib/types';

	let showFilesystem = $state(true);

	// Example list of images
	let images: Image[] = [
		{ id: 1, path: 'path/to/component1/file1' },
		{ id: 2, path: 'path/to/component1/file2' },
		{ id: 3, path: 'path/to/component2' },
		{ id: 4, path: 'another/path/to/component3' }
	];

	const root: DirectoryNode = { name: 'LOCAL STORAGE', children: [], files: [] };

	// TODO: Move to server?
	$effect.pre(() => {
		if (!stores.value) return;
		images = images.concat(stores.value);

		images.forEach((image) => {
			let segments = image.path.split('/');
			let currentNode = root;

			segments.forEach((segment, index) => {
				const isFile = index === segments.length - 1;
				let existingNode = currentNode.children.find((child) => child.name === segment && !isFile);

				if (!existingNode) {
					existingNode = { name: segment, children: [], files: [] };
					// Add leaf node to files array if it's the last segment
					if (isFile) {
						currentNode.files.push({ name: segment, metadata: image });
						return;
					}
					currentNode.children.push(existingNode);
				}

				currentNode = existingNode;
			});
		});
	});
</script>

<div class="outer-container">
	{#if stores.value}
		{#if stores.value.length === 0}
			<div style="padding: 10px;">Upload an image to get started.</div>
		{:else}
			<div class="inner-container" style="padding: 5px 0 !important;">
				<Directory content={root} bind:showContent={showFilesystem} depth={1} />
			</div>
		{/if}
	{/if}
</div>
