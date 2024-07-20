<script lang="ts">
	import { defined } from '$helpers';

	let {
		variant,
		width: w,
		height: h,
		onclick = undefined
	}: { variant: string; width: number; height: number; onclick?: () => void } = $props();

	let width = `${w}rem`;
	let height = `${h}rem`;

	import Directory from '~icons/material-symbols-light/folder';
	import Image from '~icons/ph/image-light';

	import Back from '~icons/material-symbols-light/chevron-left-rounded';
	import Forward from '~icons/material-symbols-light/chevron-right-rounded';
	import Up from '~icons/material-symbols-light/keyboard-arrow-up-rounded';

	import New from '~icons/ant-design/plus-circle-outlined';
	import View from '~icons/material-symbols/view-day-outline';
	import Sort from '~icons/fluent/arrow-sort-28-filled';
	import DownArrow from '~icons/mdi/chevron-down';

	import NewDirectory from '~icons/mdi/folder-add';
	import NewImage from '~icons/mdi/image-add-outline';

	import ListView from '~icons/ic/round-format-list-bulleted';
	import GridView from '~icons/material-symbols/grid-view-outline-rounded';

	import Close from '~icons/material-symbols/close';
</script>

{#if defined(onclick)}
	<button {onclick} style="width: {width}; height: {height};">
		{@render icon()}
	</button>
{:else}
	{@render icon()}
{/if}

{#snippet icon()}
	{#if variant === 'directory'}
		<Directory {width} {height} />
	{:else if variant === 'image'}
		<Image {width} {height} />
	{:else if variant === 'back'}
		<Back {width} {height} />
	{:else if variant === 'forward'}
		<Forward {width} {height} />
	{:else if variant === 'up'}
		<Up {width} {height} />
	{:else if variant === 'new'}
		<New {width} {height} />
	{:else if variant === 'view'}
		<View {width} {height} />
	{:else if variant === 'sort'}
		<Sort {width} {height} />
	{:else if variant === 'down-arrow'}
		<DownArrow {width} {height} />
	{:else if variant === 'new-image'}
		<NewImage {width} {height} />
	{:else if variant === 'new-directory'}
		<NewDirectory {width} {height} />
	{:else if variant === 'list-view'}
		<ListView {width} {height} />
	{:else if variant === 'grid-view'}
		<GridView {width} {height} />
	{:else if variant === 'close'}
		<Close {width} {height} />
	{/if}
{/snippet}
