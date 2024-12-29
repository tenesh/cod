import {defineConfig} from 'vite';
import {svelte, vitePreprocess} from '@sveltejs/vite-plugin-svelte'
import laravel from 'laravel-vite-plugin';
import {preprocessMeltUI, sequence} from '@melt-ui/pp'

export default defineConfig({
    plugins: [
        laravel({
            input: 'resources/js/app.ts',
            refresh: true,
        }),
        svelte({
            preprocess: sequence([
                vitePreprocess(),
                preprocessMeltUI()
            ])
        }),
    ],
});
