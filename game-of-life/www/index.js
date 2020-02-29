import { Universe } from "game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const gen = document.getElementById("game-of-life-gen");
const x = window.innerWidth / 10;
const y = window.innerHeight / 17;
const universe = Universe.new(x, y);
const renderLoop = () => {
  pre.textContent = universe.render();
  let generation = universe.tick();
  gen.textContent = `generation ${generation}`;

  requestAnimationFrame(renderLoop);
};

// kick it off.
requestAnimationFrame(renderLoop);
