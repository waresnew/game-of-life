import { next_step, renderer, world } from "./app";

document.getElementById("stepsize-less")!.addEventListener("click", (event) => {
	world.stepExp = Math.max(0, world.stepExp - 1);
	renderer.change_step_exp(world.stepExp);
});
document.getElementById("stepsize-more")!.addEventListener("click", (event) => {
	world.stepExp = Math.min(49, world.stepExp + 1); //(MAX_HEIGHT+1)-2=49
	renderer.change_step_exp(world.stepExp);
});
document.getElementById("once-button")!.addEventListener("click", (event) => {
	world.ticking = false;
	playButton.textContent = "Play";
	const start = performance.now();
	next_step();
	const elapsed = (performance.now() - start) / 1000;
	document.getElementById("once-runtime")!.textContent = `Took ${elapsed}s`;
});
const playButton = document.getElementById("play-button")!;
playButton.addEventListener("click", (event) => {
	if (!world.ticking) {
		world.ticking = true;
		playButton.textContent = "Stop";
	} else {
		world.ticking = false;
		playButton.textContent = "Play";
	}
});
const debugButton = document.getElementById("toggle-debug")!;
debugButton.addEventListener("click", (event) => {
	const debug = document.getElementById("debug-content")!;
	if (debug.style.visibility == "hidden") {
		debugButton.textContent = "Hide debug info";
		debug.style.visibility = "visible";
	} else {
		debug.style.visibility = "hidden";
		debugButton.textContent = "Show debug info";
	}
});
