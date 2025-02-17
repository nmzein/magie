<script lang="ts">
	let {
		name,
		class: classes = '',
		onclick = undefined
	}: {
		name: string;
		class?: string;
		onclick?: () => void;
	} = $props();

	import { defined } from '$helpers';
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
	import Trash from '~icons/material-symbols-light/delete-outline';
	import Close from '~icons/material-symbols/close';
	import ZoomIn from '~icons/mdi/plus';
	import ZoomOut from '~icons/mdi/minus';
	import Cursor from '~icons/material-symbols/point-scan-rounded';
	import Freehand from '~icons/material-symbols-light/draw';
	import Square from '~icons/material-symbols-light/square-outline';
	import Explorer from '~icons/material-symbols-light/folder-rounded';
	import Control from '~icons/codicon/settings';
	import Info from '~icons/material-symbols-light/info-rounded';
	import Settings from '~icons/material-symbols-light/settings-outline-rounded';
	import Button from './Button.svelte';
</script>

{#if defined(onclick)}
	<Button {onclick}>
		{@render Icon()}
	</Button>
{:else}
	{@render Icon()}
{/if}

{#snippet Icon()}
	{#if name === 'directory'}
		<Directory class={classes} />
	{:else if name === 'image'}
		<Image class={classes} />
	{:else if name === 'back'}
		<Back class={classes} />
	{:else if name === 'forward'}
		<Forward class={classes} />
	{:else if name === 'up'}
		<Up class={classes} />
	{:else if name === 'new'}
		<New class={classes} />
	{:else if name === 'view'}
		<View class={classes} />
	{:else if name === 'sort'}
		<Sort class={classes} />
	{:else if name === 'down-arrow'}
		<DownArrow class={classes} />
	{:else if name === 'new-image'}
		<NewImage class={classes} />
	{:else if name === 'new-directory'}
		<NewDirectory class={classes} />
	{:else if name === 'list-view'}
		<ListView class={classes} />
	{:else if name === 'grid-view'}
		<GridView class={classes} />
	{:else if name === 'trash'}
		<Trash class={classes} />
	{:else if name === 'close'}
		<Close class={classes} />
	{:else if name === 'zoom-in'}
		<ZoomIn class={classes} />
	{:else if name === 'zoom-out'}
		<ZoomOut class={classes} />
	{:else if name === 'cursor'}
		<Cursor class={classes} />
	{:else if name === 'freehand'}
		<Freehand class={classes} />
	{:else if name === 'square'}
		<Square class={classes} />
	{:else if name === 'explorer'}
		<Explorer class={classes} />
	{:else if name === 'control'}
		<Control class={classes} />
	{:else if name === 'info'}
		<Info class={classes} />
	{:else if name === 'settings'}
		<Settings class={classes} />
	{/if}
{/snippet}
