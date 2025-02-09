import defaultTheme from 'tailwindcss/defaultTheme';

// eslint-disable-next-line @typescript-eslint/no-require-imports
const tailwindcssForms = require('@tailwindcss/forms');

/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            animation: {
                ripple: 'ripple var(--duration,2s) ease calc(var(--i, 0)*.2s) infinite',
            },
            keyframes: {
                ripple: {
                    '0%, 100%': {
                        transform: 'translate(-50%, -50%) scale(1)',
                    },
                    '50%': {
                        transform: 'translate(-50%, -50%) scale(0.9)',
                    },
                },
            },
            fontFamily: {
                sans: ['Plus Jakarta Sans', ...defaultTheme.fontFamily.sans],
            },
            colors: {
                'primary-brand': '#7d79b8',
                'secondary-brand': '#5a7080',
                primary: {
                    50: '#f9f9fc',
                    100: '#e7e7f2',
                    200: '#d4d4e9',
                    300: '#b6b7da',
                    400: '#9393c7',
                    500: '#7d79b8',
                    600: '#6f67a9',
                    700: '#665b9a',
                    800: '#574e7f',
                    900: '#494266',
                    950: '#2f2b40',
                },
                secondary: {
                    50: '#f4f6f7',
                    100: '#e2e7eb',
                    200: '#c8d2d9',
                    300: '#a2b3be',
                    400: '#758b9b',
                    500: '#5a7080',
                    600: '#4f6070',
                    700: '#434f5b',
                    800: '#3c454e',
                    900: '#353c44',
                    950: '#20252c',
                },
            },
        },
    },
    plugins: [tailwindcssForms],
};
