import { Context } from 'runed';

export const context = new Context<{ drag: (e: MouseEvent) => void }>('');
