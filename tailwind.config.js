/** @type {import('tailwindcss').Config} */
const defaultTheme = require('tailwindcss/defaultTheme');
module.exports = {
    mode: "all",
    content: ["./input.css", "./packages/app/src/**/*.{rs,html,css}"],
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
    daisyui: {
        themes: true,
        logs: false,
    }
}