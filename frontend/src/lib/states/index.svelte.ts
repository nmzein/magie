import { ExplorerState } from './explorer.svelte';
import { RepositoryState } from './repository.svelte';
import { SelectionBoxState } from './selection-box.svelte';
import { ImageViewer } from './image.svelte';
import { ContextMenuState } from './context-menu.svelte';
import type { Image } from '$types';

export { SelectionBoxState };

export const repository = new RepositoryState();
export let explorer: ExplorerState | undefined;
export function InitExplorerState() {
	explorer = new ExplorerState();
}
export const images: ImageViewer[] = $state([]);
export function NewImageViewer(info: Image) {
	images[0] = new ImageViewer(info);
}
export const contextMenu = new ContextMenuState();
