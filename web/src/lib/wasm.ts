import { get_config, Renderer } from '$wasm/game_of_life.js';
export const config = get_config();
export const renderer = new Renderer(0);
