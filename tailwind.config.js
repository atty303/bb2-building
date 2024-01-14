/** @type {import('tailwindcss').Config} */
const defaultTheme = require('tailwindcss/defaultTheme');
module.exports = {
    content: ["src/**/*.rs", "index.html"],
    theme: {
        extend: {
            fontFamily: {
                sans: ["DotGothic16", "sans-serif"],
            },
        },
    },
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
    daisyui: {
        themes: true,
        logs: false,
    }
}