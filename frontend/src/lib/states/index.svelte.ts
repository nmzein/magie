import { ExplorerState } from './explorer.svelte';
import { RepositoryState } from './repository.svelte';
import { SelectionBoxState } from './selection-box.svelte';
import { ImageViewer } from './image.svelte';
import { ContextMenu } from './context-menu.svelte';
import { Clipboard } from './clipboard.svelte';
import type { Image } from '$types';

export { SelectionBoxState };

export const repository = new RepositoryState();
export const clipboard = new Clipboard();
export let explorer: ExplorerState | undefined;
export function InitExplorerState() {
	explorer = new ExplorerState();
}
export const images: ImageViewer[] = $state([]);
export function NewImageViewer(storeId: number, info: Image) {
	images[0] = new ImageViewer(storeId, info);
}
export const contextMenu = new ContextMenu();
