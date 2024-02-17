/** @type {import('tailwindcss').Config} */
const defaultTheme = require('tailwindcss/defaultTheme');
module.exports = {
    mode: "all",
    content: ["./tailwind.css", "./src/**/*.{rs,html,css}"],
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
    daisyui: {
        themes: true,
        logs: false,
    }
}