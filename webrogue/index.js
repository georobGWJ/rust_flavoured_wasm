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
}