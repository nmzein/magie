<script lang="ts">
	let { file, indent } = $props<{ file: { name: string; metadata: Image }; indent: string }>();

	import type { Image } from '$lib/types';
	import { LoadImage } from '$api';
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div style="padding: 2px {indent};" onclick={async () => LoadImage(file.metadata)}>
	<img src="/default_file.svg" alt="File" />
	<p>
		{file.name}
	</p>
</div>

<style lang="scss">
	div {
		display: flex;
		cursor: pointer;

		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
			text-decoration: underline;
		}
	}

	p {
		margin: 0;
	}

	img {
		margin-left: 2px;
		margin-right: 5px;
		width: 18px;
		height: 18px;
	}
</style>
