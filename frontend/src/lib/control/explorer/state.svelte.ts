import { registry, state } from '$stores';
import type { Directory } from '$types';

export const directoryStack = state<number[]>([0]);
