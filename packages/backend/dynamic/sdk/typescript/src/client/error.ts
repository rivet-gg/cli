/** Custom error class for API errors */
export class RivetRequestError extends Error {
	public code: string;
	public status: number;
	public meta?: any;

	constructor(code: string, message: string, status: number, meta?: any) {
		super(message);
		this.name = "RivetRequestError";
		this.code = code;
		this.status = status;
		this.meta = meta;
	}
}
