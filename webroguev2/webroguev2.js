import * as wasm from './webroguev2_bg';

const lTextDecoder = typeof TextDecoder === 'undefined' ? require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbg_alert_8f39b604fe0a2418(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    alert(varg0);
}

export function __wbg_log_35393fce4b435c27(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    console.log(varg0);
}

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

export function __wbg_statsupdated_2e9eddeb9a4db041(arg0) {
    stats_updated(takeObject(arg0));
}

export function __wbg_draw_ab7a52b94535f93e(arg0, arg1, arg2, arg3, arg4) {
    let varg3 = getStringFromWasm(arg3, arg4);
    getObject(arg0).draw(arg1, arg2, varg3);
}

export function __wbg_draw_f465bfbb141acf91(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    let varg3 = getStringFromWasm(arg3, arg4);
    let varg5 = getStringFromWasm(arg5, arg6);
    getObject(arg0).draw(arg1, arg2, varg3, varg5);
}
/**
*/
export const PlayerCoreType = Object.freeze({ Player:0,Friendly:1,Neutral:2,Hostile:3,Unknown:4, });

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? require('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {

        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let writeOffset = 0;
        while (true) {
            const view = getUint8Memory().subarray(ptr + writeOffset, ptr + size);
            const { read, written } = cachedTextEncoder.encodeInto(arg, view);
            writeOffset += written;
            if (read === arg.length) {
                break;
            }
            arg = arg.substring(read);
            ptr = wasm.__wbindgen_realloc(ptr, size, size += arg.length * 3);
        }
        WASM_VECTOR_LEN = writeOffset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {

        const buf = cachedTextEncoder.encode(arg);
        const ptr = wasm.__wbindgen_malloc(buf.length);
        getUint8Memory().set(buf, ptr);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    };
}

export function __wbindgen_json_parse(ptr, len) { return addHeapObject(JSON.parse(getStringFromWasm(ptr, len))); }

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

function freePlayerCore(ptr) {

    wasm.__wbg_playercore_free(ptr);
}
/**
*/
export class PlayerCore {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freePlayerCore(ptr);
    }

    /**
    * @param {number} x
    * @param {number} y
    * @param {string} icon
    * @param {string} color
    * @param {any} display
    * @param {string} player_type
    * @returns {}
    */
    constructor(x, y, icon, color, display, player_type) {
        const ptr2 = passStringToWasm(icon);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm(color);
        const len3 = WASM_VECTOR_LEN;
        const ptr5 = passStringToWasm(player_type);
        const len5 = WASM_VECTOR_LEN;
        try {
            this.ptr = wasm.playercore_new(x, y, ptr2, len2, ptr3, len3, addHeapObject(display), ptr5, len5);

        } finally {
            wasm.__wbindgen_free(ptr2, len2 * 1);
            wasm.__wbindgen_free(ptr3, len3 * 1);
            wasm.__wbindgen_free(ptr5, len5 * 1);

        }

    }
    /**
    * @returns {number}
    */
    x() {
        return wasm.playercore_x(this.ptr);
    }
    /**
    * @returns {number}
    */
    y() {
        return wasm.playercore_y(this.ptr);
    }
    /**
    * @returns {number}
    */
    hp() {
        return wasm.playercore_hp(this.ptr);
    }
    /**
    * @returns {void}
    */
    draw() {
        return wasm.playercore_draw(this.ptr);
    }
    /**
    * @param {number} x
    * @param {number} y
    * @returns {void}
    */
    move_to(x, y) {
        return wasm.playercore_move_to(this.ptr, x, y);
    }
    /**
    * @returns {void}
    */
    emit_stats() {
        return wasm.playercore_emit_stats(this.ptr);
    }
    /**
    * @param {number} hits
    * @returns {number}
    */
    take_damage(hits) {
        return wasm.playercore_take_damage(this.ptr, hits);
    }
}

function freeRustEngine(ptr) {

    wasm.__wbg_rustengine_free(ptr);
}
/**
*/
export class RustEngine {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeRustEngine(ptr);
    }

    /**
    * @param {any} display
    * @returns {}
    */
    constructor(display) {
        this.ptr = wasm.rustengine_new(addHeapObject(display));
    }
    /**
    * @param {number} x
    * @param {number} y
    * @param {number} val
    * @returns {void}
    */
    on_dig(x, y, val) {
        return wasm.rustengine_on_dig(this.ptr, x, y, val);
    }
    /**
    * @returns {void}
    */
    draw_map() {
        return wasm.rustengine_draw_map(this.ptr);
    }
    /**
    * @param {number} x
    * @param {number} y
    * @returns {void}
    */
    redraw_at(x, y) {
        return wasm.rustengine_redraw_at(this.ptr, x, y);
    }
    /**
    * @param {number} x
    * @param {number} y
    * @returns {void}
    */
    place_box(x, y) {
        return wasm.rustengine_place_box(this.ptr, x, y);
    }
    /**
    * @param {PlayerCore} pc
    * @param {number} x
    * @param {number} y
    * @returns {void}
    */
    open_box(pc, x, y) {
        return wasm.rustengine_open_box(this.ptr, pc.ptr, x, y);
    }
    /**
    * @param {number} x
    * @param {number} y
    * @returns {void}
    */
    mark_wasmprize(x, y) {
        return wasm.rustengine_mark_wasmprize(this.ptr, x, y);
    }
    /**
    * @param {PlayerCore} pc
    * @param {number} x
    * @param {number} y
    * @returns {void}
    */
    move_player(pc, x, y) {
        return wasm.rustengine_move_player(this.ptr, pc.ptr, x, y);
    }
    /**
    * @param {number} x
    * @param {number} y
    * @returns {boolean}
    */
    free_cell(x, y) {
        return (wasm.rustengine_free_cell(this.ptr, x, y)) !== 0;
    }
}

export function __wbindgen_object_drop_ref(i) { dropObject(i); }

