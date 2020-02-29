import { Universe } from "game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const gen = document.getElementById("game-of-life-gen");
const x = window.innerWidth / 10;
const y = window.innerHeight / 20;
const universe = Universe.new(x, y);
const renderLoop = () => {
  pre.textContent = universe.render();
  let stats = universe.tick();
  gen.textContent = `generation ${stats.generation}; changes to last generation ${stats.changes}`;

  requestAnimationFrame(renderLoop);
};

// kick it off.
requestAnimationFrame(renderLoop);
