/** @type {import('tailwindcss').Config} */
const defaultTheme = require('tailwindcss/defaultTheme');
module.exports = {
    content: ["src/**/*.rs", "index.html"],
    plugins: [require("daisyui")],
    daisyui: {
        themes: true,
    }
}