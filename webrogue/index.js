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