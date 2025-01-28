import { ExplorerState } from './explorer.svelte';
import { RepositoryState } from './repository.svelte';
import { SelectionBoxState } from './selection-box.svelte';
import { ImageViewer } from './image.svelte';
import { Transformer } from './transformer.svelte';
import { ContextMenuState } from './context-menu.svelte';

export { SelectionBoxState };

export const repository = new RepositoryState();
export let explorer: ExplorerState | undefined;
export function InitExplorerState() {
	explorer = new ExplorerState();
}
export const image = new ImageViewer();
export const transformer = Transformer();
export const contextMenu = new ContextMenuState();
