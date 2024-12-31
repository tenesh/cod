import defaultTheme from 'tailwindcss/defaultTheme';

// eslint-disable-next-line @typescript-eslint/no-require-imports
const tailwindcssForms = require('@tailwindcss/forms');

/** @type {import('tailwindcss').Config} */
export default {
    content: [
        './vendor/laravel/framework/src/Illuminate/Pagination/resources/views/*.blade.php',
        './storage/framework/views/*.php',
        './resources/**/*.blade.php',
        './resources/js/**/*.svelte',
    ],
    theme: {
        extend: {
            fontFamily: {
                sans: ['Plus Jakarta Sans', ...defaultTheme.fontFamily.sans],
            },
        },
    },
    plugins: [
        tailwindcssForms,
    ],
};
