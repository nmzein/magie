import { DEFAULT_POINT, type Point } from '$types';

export type ContextMenuItem =
	| { name: string; action?: () => void; disabled?: boolean; hidden?: boolean }
	| 'separator';

export class ContextMenu {
	public show = $state(false);
	public position: Point = $state(DEFAULT_POINT);
	public items: ContextMenuItem[] = $state([]);
}
