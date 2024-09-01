<script lang="ts">
	let {
		name,
		class: className = '',
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
		<Directory class={className} />
	{:else if name === 'image'}
		<Image class={className} />
	{:else if name === 'back'}
		<Back class={className} />
	{:else if name === 'forward'}
		<Forward class={className} />
	{:else if name === 'up'}
		<Up class={className} />
	{:else if name === 'new'}
		<New class={className} />
	{:else if name === 'view'}
		<View class={className} />
	{:else if name === 'sort'}
		<Sort class={className} />
	{:else if name === 'down-arrow'}
		<DownArrow class={className} />
	{:else if name === 'new-image'}
		<NewImage class={className} />
	{:else if name === 'new-directory'}
		<NewDirectory class={className} />
	{:else if name === 'list-view'}
		<ListView class={className} />
	{:else if name === 'grid-view'}
		<GridView class={className} />
	{:else if name === 'trash'}
		<Trash class={className} />
	{:else if name === 'close'}
		<Close class={className} />
	{:else if name === 'zoom-in'}
		<ZoomIn class={className} />
	{:else if name === 'zoom-out'}
		<ZoomOut class={className} />
	{:else if name === 'cursor'}
		<Cursor class={className} />
	{:else if name === 'freehand'}
		<Freehand class={className} />
	{:else if name === 'square'}
		<Square class={className} />
	{:else if name === 'explorer'}
		<Explorer class={className} />
	{:else if name === 'control'}
		<Control class={className} />
	{:else if name === 'info'}
		<Info class={className} />
	{:else if name === 'settings'}
		<Settings class={className} />
	{/if}
{/snippet}
