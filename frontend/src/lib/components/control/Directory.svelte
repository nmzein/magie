<script lang="ts">
	let {
		content,
		showContent = false,
		depth
	} = $props<{ content: DirectoryNode; showContent: boolean; depth: number }>();

	import type { DirectoryNode } from '$lib/types';
	import Directory from '$control/Directory.svelte';
	import File from './File.svelte';

	let indent = (depth * 10).toString() + 'px';
	let nextIndent = ((depth + 1) * 10).toString() + 'px';

	let src = $derived(showContent ? '/default_folder_opened.svg' : '/default_folder.svg');
	let alt = $derived(showContent ? 'Open Directory' : 'Directory');
	let displayContent = $derived(showContent ? 'visible' : 'none');
</script>

<div>
	<button
		id="label"
		class="secondary-text"
		style="padding: 4px {indent}; {depth == 1 && showContent ? 'margin-bottom: 3px;' : ''}"
		onclick={() => (showContent = !showContent)}
	>
		<!-- Directory icon for directories under root -->
		{#if depth > 1}
			<img {src} {alt} />
		{/if}
		{content.name}
	</button>

	<div style="display: {displayContent};">
		{#each content.files as file}
			<File {file} indent={nextIndent} />
		{/each}
		{#each content.children as directory}
			<Directory content={directory} showContent={false} depth={depth + 1} />
		{/each}
	</div>
</div>

<style lang="scss">
	#label {
		user-select: none;
		display: flex;
		flex-direction: row;
		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}
	}

	div {
		display: flex;
		flex-direction: column;
	}

	img {
		margin-right: 5px;
		width: 20px;
		height: 20px;
	}
</style>
