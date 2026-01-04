import { createContext } from 'svelte';

type Getter<T> = () => T;
type Setter<T> = (value: T) => void;

export class Context<T> {
	private readonly getFn: Getter<T>;
	private readonly setFn: Setter<T>;

	constructor() {
		const [get, set] = createContext<T>();
		this.getFn = get;
		this.setFn = set;
	}

	get(): T {
		return this.getFn();
	}

	set(value: T): T {
		this.setFn(value);
		return this.getFn();
	}
}
