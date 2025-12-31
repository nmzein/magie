export class ByteReader {
	#view: DataView<ArrayBuffer>;
	#offset: number;

	constructor(data: MessageEvent['data']) {
		const array = new Uint8Array(data);
		this.#view = new DataView(array.buffer, array.byteOffset, array.byteLength);
		this.#offset = 0;
	}

	u8() {
		return this.#view.getUint8(this.#offset++);
	}

	u16(le = true) {
		const v = this.#view.getUint16(this.#offset, le);
		this.#offset += 2;
		return v;
	}

	u32(le = true) {
		const v = this.#view.getUint32(this.#offset, le);
		this.#offset += 4;
		return v;
	}

	i8() {
		const v = this.#view.getInt8(this.#offset);
		this.#offset += 1;
		return v;
	}

	i16(le = true) {
		const v = this.#view.getInt16(this.#offset, le);
		this.#offset += 2;
		return v;
	}

	i32(le = true) {
		const v = this.#view.getInt32(this.#offset, le);
		this.#offset += 4;
		return v;
	}

	bytes(le = true): Uint8Array<ArrayBuffer> {
		const len = this.u32(le);
		const _cap = this.u32(le);
		const start = this.#offset;
		this.#offset += len;

		return new Uint8Array(this.#view.buffer, this.#view.byteOffset + start, len);
	}

	string(le = true): string {
		const bytes = this.bytes(le);
		return new TextDecoder('utf-8').decode(bytes);
	}

	skip(len: number) {
		this.#offset += len;
	}

	remaining() {
		return this.#view.byteLength - this.#offset;
	}
}

export class ByteWriter {
	#buf: Uint8Array;
	#view: DataView;
	#offset: number;

	constructor(size: number) {
		this.#buf = new Uint8Array(size);
		this.#view = new DataView(this.#buf.buffer);
		this.#offset = 0;
	}

	u8(v: number) {
		this.#view.setUint8(this.#offset, v);
		this.#offset += 1;
	}

	u16(v: number, le = true) {
		this.#view.setUint16(this.#offset, v, le);
		this.#offset += 2;
	}

	u32(v: number, le = true) {
		this.#view.setUint32(this.#offset, v, le);
		this.#offset += 4;
	}

	i8(v: number) {
		this.#view.setInt8(this.#offset, v);
		this.#offset += 1;
	}

	i16(v: number, le = true) {
		this.#view.setInt16(this.#offset, v, le);
		this.#offset += 2;
	}

	i32(v: number, le = true) {
		this.#view.setInt32(this.#offset, v, le);
		this.#offset += 4;
	}

	finish(): Uint8Array {
		return this.#buf;
	}
}
