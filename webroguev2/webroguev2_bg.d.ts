/* tslint:disable */
export const memory: WebAssembly.Memory;
export function __wbg_rustengine_free(a: number): void;
export function rustengine_new(a: number): number;
export function rustengine_on_dig(a: number, b: number, c: number, d: number): void;
export function rustengine_draw_map(a: number): void;
export function rustengine_redraw_at(a: number, b: number, c: number): void;
export function rustengine_place_box(a: number, b: number, c: number): void;
export function rustengine_open_box(a: number, b: number, c: number, d: number): void;
export function rustengine_mark_wasmprize(a: number, b: number, c: number): void;
export function rustengine_move_player(a: number, b: number, c: number, d: number): void;
export function rustengine_free_cell(a: number, b: number, c: number): number;
export function __wbg_playercore_free(a: number): void;
export function playercore_new(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number): number;
export function playercore_x(a: number): number;
export function playercore_y(a: number): number;
export function playercore_hp(a: number): number;
export function playercore_draw(a: number): void;
export function playercore_move_to(a: number, b: number, c: number): void;
export function playercore_emit_stats(a: number): void;
export function playercore_take_damage(a: number, b: number): number;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number): number;
export function __wbindgen_free(a: number, b: number): void;
