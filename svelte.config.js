import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { preprocessMeltUI, sequence } from '@melt-ui/pp';
import path from 'path';

const config = {
    preprocess: sequence([vitePreprocess(), preprocessMeltUI()]),
    kit: {
        adapter: adapter(),
        alias: {
            $modules: path.resolve('./src/modules'),
        },
    },
};

export default config;
