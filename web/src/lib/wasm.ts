import {
	get_config,
	type PerfStats,
	Renderer,
	Point as RustCellPoint
} from '../../../pkg/game_of_life.js';
export const config = get_config();
export const renderer = new Renderer(0);
