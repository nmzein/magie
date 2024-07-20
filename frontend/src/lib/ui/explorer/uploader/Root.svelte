<script lang="ts">
	import { explorer, uploader } from '$states';
	import { scale } from 'svelte/transition';

	import TopBar from './TopBar.svelte';
	import UploadAsset from './UploadAsset.svelte';

	let page: number = $state(1);
</script>

{#if explorer.showUploader}
	<div id="outer" transition:scale={{ duration: 200 }}>
		<div id="inner" class="panel flex-column" role="dialog" aria-modal="true">
			<TopBar />

			<div id="bottom" class="flex-row">
				{#if page === 1}
					<UploadAsset asset={uploader.image} placeholder="Image" />
				{:else if page === 2}{/if}
			</div>
		</div>
	</div>
{/if}

<style lang="scss">
	#outer {
		position: absolute;
		z-index: 5;
		width: calc(100% - 80px);
		height: calc(100% - 80px);
		margin: 40px;

		border-radius: var(--border-radius);
	}

	#inner {
		z-index: 6;
		box-shadow: 0 0 10px 0 rgba(0, 0, 0, 0.2);
		width: 100%;
		height: 100%;
		border-radius: inherit;
		border: 2px solid #3e3e3e;
	}

	#bottom {
		flex: 1;
		justify-content: center;
		align-items: center;
		margin: 20px;
	}
</style>
