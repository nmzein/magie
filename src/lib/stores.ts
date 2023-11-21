import { writable } from 'svelte/store';
import type { ImageMetadata } from '$lib/types';

export const imagesStore = writable<HTMLImageElement[]>([]);
export const metadataStore = writable<ImageMetadata | undefined>(undefined);
