import js from '@eslint/js';
import eslintPluginSvelte from 'eslint-plugin-svelte';
import prettier from 'eslint-config-prettier';
import tseslint from "typescript-eslint";
import globals from 'globals';

export default [
    js.configs.recommended,
    ...tseslint.configs.recommended,
    ...eslintPluginSvelte.configs["flat/recommended"],
    prettier,
    ...eslintPluginSvelte.configs["flat/prettier"],
    {
        languageOptions: {
            globals: {
                ...globals.browser,
                ...globals.node
            }
        }
    },
    {
        files: ["**/*.svelte", "*.svelte"],
        languageOptions: {
            parserOptions: {
                parser: tseslint.parser
            }
        }
    },
    {
        ignores: ['vendor', "src-tauri", "build", "node_modules"]
    },
    {
        rules: {
            "no-undef": "off",
            "@typescript-eslint/no-unused-vars": "off",
            "@typescript-eslint/no-explicit-any": "off",
            "@typescript-eslint/ban-ts-comment": "off"
        }
    },
];