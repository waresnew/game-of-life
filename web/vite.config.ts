import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import checker from "vite-plugin-checker";
import { viteSingleFile } from "vite-plugin-singlefile";

export default defineConfig({
	build: {
		outDir: "../dist",
		emptyOutDir: true,
	},
	plugins: [
		viteSingleFile(),
		tailwindcss(),
		checker({
			typescript: true,
		}),
	],
});
