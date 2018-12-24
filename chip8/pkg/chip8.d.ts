/* tslint:disable */
export enum RunMode {Legacy,SuperChip,}
export function run(arg0: any): ClosureHandle;

export class Ram {
free(): void;

static  new(): Ram;

 get_meta_address(): number;

 get_length(): number;

 print(arg0: boolean): void;

 write_rom(arg0: Uint8Array): void;

 read(arg0: number): number;

 write(arg0: number, arg1: number): void;

}
export class ClosureHandle {
free(): void;

}
