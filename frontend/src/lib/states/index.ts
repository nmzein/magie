import { Explorer } from './explorer.svelte';
import { Repository } from './repository.svelte';
import { SelectionBox } from './selection-box.svelte';
import { Uploader } from './uploader.svelte';
import { ImageViewer } from './image.svelte';
import { Transformer } from './transformer.svelte';

export { SelectionBox };

export const repository = new Repository();
// TODO: Make this a list of open file explorers and image viewers.
export const explorer = new Explorer();
export const uploader = Uploader();
export const image = ImageViewer();
export const transformer = Transformer();