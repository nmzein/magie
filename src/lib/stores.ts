import { writable } from 'svelte/store';
import type { ImageMetadata } from '$lib/types';

export const imagesStore = writable<HTMLImageElement[]>([]);
export const metadataStore = writable<ImageMetadata | undefined>();

export const ImageUploadStore = writable<File | undefined>();
export const AnnotationFileUploadStore = writable<File | undefined>();

export const AnnotationGeneratorStore = writable<string>('');
export const AutogenerateAnnotationsStore = writable<boolean>(true);
