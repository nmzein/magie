import { ContextMenu } from '$ui/ContextMenu/state.svelte.ts';
import { Clipboard } from './clipboard.svelte.ts';
import { Registry } from './registry.svelte.ts';
import { Repository } from './repository.svelte.ts';
import { ViewerManager } from './viewer-manager.svelte.ts';

export const registry = new Registry();
export const repository = new Repository();
export const clipboard = new Clipboard();
export const contextMenu = new ContextMenu();
export const viewerManager = new ViewerManager();
