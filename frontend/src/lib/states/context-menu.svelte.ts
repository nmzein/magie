import { DEFAULT_POINT, type Point } from '$types';

export class ContextMenuState {
	public show = $state(false);
	public position: Point = $state(DEFAULT_POINT);
	public items: { name: string; action: () => void }[] = $state([]);
}
