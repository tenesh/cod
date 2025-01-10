import defaultTheme from 'tailwindcss/defaultTheme';

// eslint-disable-next-line @typescript-eslint/no-require-imports
const tailwindcssForms = require('@tailwindcss/forms');

/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            fontFamily: {
                sans: ['Plus Jakarta Sans', ...defaultTheme.fontFamily.sans],
            },
            colors: {
                'primary-brand': '#3b1e54',
                'secondary-brand': '#eeeeee',
                primary: {
                    50: "#EBE1F4",
                    100: "#D7C3EA",
                    200: "#B28AD5",
                    300: "#8B4EC0",
                    400: "#64338F",
                    500: "#3B1E54",
                    600: "#2F1844",
                    700: "#221131",
                    800: "#180C22",
                    900: "#0B050F",
                    950: "#050308"
                },
                secondary: {
                    50: '#FCFCFC',
                    100: '#FCFCFC',
                    200: '#F7F7F7',
                    300: '#F5F5F5',
                    400: '#F2F2F2',
                    500: '#EEEEEE',
                    600: '#BFBFBF',
                    700: '#8F8F8F',
                    800: '#5E5E5E',
                    900: '#303030',
                    950: '#171717',
                },
            },
        },
    },
    plugins: [tailwindcssForms],
};
