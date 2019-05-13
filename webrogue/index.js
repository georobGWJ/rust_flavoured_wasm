import { Engine, PlayerCore } from './roguewasm';

var Game = {
    display: null,
    engine: null,
    player: null,
    enemy: null,

    init: function () {
        this.display = new ROT.Display({ width: 125, height: 40 });
        document.getElementById("rogueCanvas".appendChild(this.display.getContainer));

        this.engine = new Engine(this.display);
        this.generateMap();

        var scheduler = new ROT.Scheduler.Simple();

        scheduler.add(this.player, true);
        scheduler.add(this.enemy, true);

        this.rotengine = new ROT.Engine(scheduler);
        this.rotengine.start();
    },

    generateMap: function() {
        var digger = new ROT.Map.Digger();
        var freeCells = [];

        var digCallback = function (x, y, value) {
            if (!value) {
                var key = x + ", " + y;
                freeCells.push(key);
            }
            this.engine.on_dig(x, y, value);
        }
        // Invoke on_dig() on the Rust 'Engine' struct
        digger.create(digCallback.bind(this));

        this.generateBoxes(freeCells);
        // Invoke draw_map() on the Rust 'Engine' struct
        this.engine.draw_map();

        // 'this' refers to the JS Game class instance. _createBeing is a
        // helper utillity function defined below
        this.player = this._createBeing(Player, freeCells);
        this.enemy = this._createBeing(Checko, freeCells);
    }
}

var Player = function(x, y) {
    this._core = new PlayerCore(x, y, "@", "#ff0", Game.display);
    this._core.draw();
}

Player.prototype.act = function () {
    Game.rotengine.lock();
    window.addEventListener("keydown", this);
}

Player.prototype.handleEvent = function (e) {
    var keyMap = {};
    keyMap[38] = 0;  // up arrow
    keyMap[33] = 1;  // page up
    keyMap[39] = 2;  // right arrow
    keyMap[34] = 3;  // page down
    keyMap[40] = 4;  // down arrow
    keyMap[35] = 5;  // end
    keyMap[37] = 6;  // left arrow
    keyMap[36] = 7;  // home

    var code = e.keyCode;

    if (code == 13 || code == 32) {
        Game.engine.open_box(this._core, this._core.x(), this._core.y());
        return;
    }

    /* numpad directions */
    if (!(code in keyMap)) { return; }

    /* is there a free space? */
    var dir = ROT.DIRS[8][keyMap[code]];
    var newX = this._core.x() + dir[0];
    var newY = this._core.y() + dir[1];

    if (!Game.engine.free_cell(newX, newY)) { return; }

    Game.engine.move_player(this._core, newX, newY);
    window.removeEventListener("keydown", this);
    Game.rotengine.unlock();
}

Player.prototype.getX = function () { return this._core.x(); }

Player.prototype.getY = function () { return this._core.y(); }
