import { RustEngine, PlayerCore } from './webroguev2';

var Game = {
    display: null,
    engine: null,
    player: null,
    enemy: null,

    init: function () {
        this.display = new ROT.Display({ width: 90, height: 30, bg: "#0f2851" });
        document.getElementById("rogueCanvas").appendChild(this.display.getContainer());

        this.engine = new RustEngine(this.display);
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
                var key = x + "," + y;
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
    },

    generateBoxes: function (freeCells) {
        for (var i = 0; i < 10; i++) {
            var index = Math.floor(ROT.RNG.getUniform() * freeCells.length);
            var key = freeCells.splice(index, 1)[0];
            var parts = key.split(",");
            var x = parseInt(parts[0]);
            var y = parseInt(parts[1]);

            this.engine.place_box(x, y);
            if (i == 9) {
                this.engine.mark_wasmprize(x, y);
            }
        }
    },

    generatePlayer: function (freeCells) {
        var index = Math.floor(ROT.RNG.getUniform() * freeCells.length);
        var key = freeCells.splice(index, 1)[0];
        var parts = key.split(",");
        var x = parseInt(parts[0]);
        var y = parseInt(parts[1]);

        console.log("Generating player...");
        this.player = new Player(x, y);
    },
};

Game._createBeing = function (what, freeCells) {
    var index = Math.floor(ROT.RNG.getUniform() * freeCells.length);
    var key = freeCells.splice(index, 1)[0];
    var parts = key.split(",");
    var x = parseInt(parts[0]);
    var y = parseInt(parts[1]);

    return new what(x, y);
}

// Check the Borrow Checker. Our Enemy
var Checko = function (x, y) {
    this._core = new PlayerCore(x, y, "B", "red", Game.display, "Hostile");
    this._core.draw();

    Checko.prototype.act = function () {
        var x = Game.player.getX();
        var y = Game.player.getY();

        var passableCallback = function (x, y) {
            return Game.engine.free_cell(x, y);
        }

        // A-Star pathfinding is a function of the Rot.js lobrary
        var astar = new ROT.Path.AStar(x, y, passableCallback, {topology: 4});

        var path = [];
        var pathCallback = function (x, y) {
            path.push([x, y]);
        }
        astar.compute(this._core.x(), this._core.y(), pathCallback);

        path.shift();
        // Check to see if Checko moves onto the player
        if (path.length <= 1) {
            Game.rotengine.lock();  // Stop all game schedulers
            alert("You were captured and consumed by the Borrow Checker!");
        } else {
            x = path[0][0];
            y = path[0][1];
            Game.engine.move_player(this._core, x, y);
        }
    }
}

var Player = function(x, y) {
    this._core = new PlayerCore(x, y, "@", "#ff0", Game.display, "Player");
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

    // 13 is 'Carriage Return' (Enter) and 32 is 'Space Bar'
    // This is our chosen command to open an in game Chest
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

    if (!Game.engine.free_cell(newX, newY)) { return; };

    Game.engine.move_player(this._core, newX, newY);
    window.removeEventListener("keydown", this);
    // TO-DO: Add checks for HP <= 0 or Win State achieved here!
    Game.rotengine.unlock();
}

Player.prototype.getX = function () { return this._core.x(); }

Player.prototype.getY = function () { return this._core.y(); }

Game.init();

export function stats_updated(stats) {
    document.getElementById("hitpoints").textContent = stats.hitpoints;
    document.getElementById("max_hitpoints").textContent = stats.max_hitpoints;
    document.getElementById("moves").textContent = stats.moves;
}