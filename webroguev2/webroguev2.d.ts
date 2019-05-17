/* tslint:disable */
/**
*/
export class Engine {
  free(): void;
/**
* @param {any} display 
* @returns {} 
*/
  constructor(display: any);
/**
* @param {number} x 
* @param {number} y 
* @param {number} val 
* @returns {void} 
*/
  on_dig(x: number, y: number, val: number): void;
/**
* @returns {void} 
*/
  draw_map(): void;
/**
* @param {number} x 
* @param {number} y 
* @returns {void} 
*/
  redraw_at(x: number, y: number): void;
/**
* @param {number} x 
* @param {number} y 
* @returns {void} 
*/
  place_box(x: number, y: number): void;
/**
* @param {PlayerCore} pc 
* @param {number} x 
* @param {number} y 
* @returns {void} 
*/
  open_box(pc: PlayerCore, x: number, y: number): void;
/**
* @param {number} x 
* @param {number} y 
* @returns {void} 
*/
  mark_wasmprize(x: number, y: number): void;
/**
* @param {PlayerCore} pc 
* @param {number} x 
* @param {number} y 
* @returns {void} 
*/
  move_player(pc: PlayerCore, x: number, y: number): void;
/**
* @param {number} x 
* @param {number} y 
* @returns {boolean} 
*/
  free_cell(x: number, y: number): boolean;
}
/**
*/
export class PlayerCore {
  free(): void;
/**
* @param {number} x 
* @param {number} y 
* @param {string} icon 
* @param {string} color 
* @param {any} display 
* @param {string} player_type 
* @returns {} 
*/
  constructor(x: number, y: number, icon: string, color: string, display: any, player_type: string);
/**
* @returns {number} 
*/
  x(): number;
/**
* @returns {number} 
*/
  y(): number;
/**
* @returns {void} 
*/
  draw(): void;
/**
* @param {number} x 
* @param {number} y 
* @returns {void} 
*/
  move_to(x: number, y: number): void;
/**
* @returns {void} 
*/
  emit_stats(): void;
/**
* @param {number} hits 
* @returns {number} 
*/
  take_damage(hits: number): number;
}
