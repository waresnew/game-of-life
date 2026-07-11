import { CELL_SIZE, canvas, config, next_step, renderer, world } from "./app";

const patterns: Record<string, string> = import.meta.glob("./patterns/*.rle", {
	query: "?raw",
	import: "default",
	eager: true,
});

document.getElementById("stepsize-less")!.addEventListener("click", (event) => {
	world.stepExp = Math.max(0, world.stepExp - 1);
	renderer.set_step_exp(world.stepExp);
});
document.getElementById("stepsize-more")!.addEventListener("click", (event) => {
	world.stepExp = Math.min(config.MAX_HEIGHT + 1 - 2, world.stepExp + 1);
	renderer.set_step_exp(world.stepExp);
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
const patternPresets = document.getElementById(
	"pattern-dropdown",
) as HTMLSelectElement;
for (const key of Object.keys(patterns)) {
	const filename = key.slice(key.lastIndexOf("/") + 1);
	patternPresets.add(new Option(filename, key));
}
patternPresets.addEventListener("change", async (event) => {
	const option = (event.target as HTMLSelectElement).value;
	applyRlePattern(patterns[option]!);
});
const patternFilePicker = document.getElementById("pattern-from-file")!;
patternFilePicker.addEventListener("change", async (event) => {
	const file = (event.target as HTMLInputElement).files![0];
	if (!file) return;
	applyRlePattern(await file.text());
});

//spec: https://conwaylife.com/wiki/Run_Length_Encoded
function applyRlePattern(pattern: string) {
	world.generation = 0n;
	world.ticking = false;
	playButton.textContent = "Play";
	renderer.clear_grid();
	const content = pattern
		.trim()
		.split("\n")
		.map((x) => x.trim())
		.filter((x) => x.length && !x.startsWith("#"));
	const header = /^x = (\d+), y = (\d+)/.exec(content.shift()!);
	const width = parseInt(header![1]!);
	const height = parseInt(header![2]!);
	let x = Math.floor(-width / 2);
	let y = Math.floor(height / 2);
	world.centre = [0, 0];
	world.zoomOutExpFloat = Math.max(
		0,
		Math.ceil(
			Math.log2(
				Math.max(
					(width * CELL_SIZE) / canvas.width,
					(height * CELL_SIZE) / canvas.height,
				),
			),
		),
	);
	const lines = content.join("").split("$");
	for (const line of lines) {
		let cnt_str = "";
		for (const c of line) {
			if (c == "!") return;
			if (/\d/.test(c)) {
				cnt_str += c;
			} else {
				const cnt = cnt_str ? parseInt(cnt_str) : 1;
				if (c != "b") {
					for (let _ = 0; _ < cnt; _++) {
						renderer.toggle_cell(BigInt(x), BigInt(y));
						x += 1;
					}
				} else {
					x += cnt;
				}
				cnt_str = "";
			}
		}
		y -= cnt_str ? parseInt(cnt_str) : 1;
		x = Math.floor(-width / 2);
	}
}
