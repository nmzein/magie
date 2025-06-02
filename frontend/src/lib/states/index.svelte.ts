import { SvelteMap } from 'svelte/reactivity';
import { Registry } from './registry.svelte.ts';
import { Repository } from './repository.svelte.ts';
import { SelectionBoxState } from './selection-box.svelte.ts';
import { ContextMenu } from './context-menu.svelte.ts';
import { Clipboard } from './clipboard.svelte.ts';
import type { View } from '$lib/types/views.ts';

export { SelectionBoxState };

export const registry = new Registry();
export const repository = new Repository();
export const clipboard = new Clipboard();
export const contextMenu = new ContextMenu();

//////////////////////

export const views: View[] = $state([]);
