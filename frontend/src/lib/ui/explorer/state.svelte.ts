import { registry, state } from '$states';
import type { Directory } from '$types';

export const directoryStack = state<number[]>([0]);
