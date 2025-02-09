import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { preprocessMeltUI, sequence } from '@melt-ui/pp'

const config = {
  preprocess:
      sequence([
          vitePreprocess(),
          preprocessMeltUI()
      ]),
  kit: {
    adapter: adapter(),
  },
};

export default config;
