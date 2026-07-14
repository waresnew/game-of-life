import { playwright } from '@vitest/browser-playwright';
import { resolve } from 'path';
import { defineConfig, mergeConfig } from 'vitest/config';
import viteConfig from './vite.config.ts';
export default mergeConfig(viteConfig, {
	test: {
		browser: {
			enabled: true,
			headless: true,
			provider: playwright(),
			// https://vitest.dev/config/browser/playwright
			instances: [{ browser: 'chromium' }]
		}
	}
});
