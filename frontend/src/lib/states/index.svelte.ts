import { Registry } from './registry.svelte.ts';
import { Repository } from './repository.svelte.ts';
import { SelectionBoxState } from './selection-box.svelte.ts';
import { ImageViewer } from './image.svelte.ts';
import { ContextMenu } from './context-menu.svelte.ts';
import { Clipboard } from './clipboard.svelte.ts';
import type { Image } from '$types';

export { SelectionBoxState };

export const registry = new Registry();
export const repository = new Repository();
export const clipboard = new Clipboard();
export const images: ImageViewer[] = $state([]);
export function NewImageViewer(storeId: number, info: Image) {
	images[0] = new ImageViewer(storeId, info);
}
export const contextMenu = new ContextMenu();
