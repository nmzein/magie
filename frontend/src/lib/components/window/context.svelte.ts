import { Context } from '$lib/helpers/context.ts';

export const context = new Context<{ drag: (e: MouseEvent) => void }>();
