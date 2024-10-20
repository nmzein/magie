import { ExplorerState } from './explorer.svelte';
import { RepositoryState } from './repository.svelte';
import { SelectionBoxState } from './selection-box.svelte';
import { UploaderState } from './uploader.svelte';
import { ImageState } from './image.svelte';
import { Transformer } from './transformer.svelte';
import { ContextMenuState } from './context-menu.svelte';

export { SelectionBoxState };

export const repository = new RepositoryState();
export const explorer = new ExplorerState();
export const uploader = new UploaderState();
export const image = new ImageState();
export const transformer = Transformer();
export const contextMenu = new ContextMenuState();
