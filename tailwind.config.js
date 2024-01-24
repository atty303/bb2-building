/** @type {import('tailwindcss').Config} */
const defaultTheme = require('tailwindcss/defaultTheme');
module.exports = {
    mode: "all",
    content: ["./input.css", "./src/**/*.{rs,html,css}", "./dist/**/*.html"],
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
    daisyui: {
        themes: true,
        logs: false,
    }
}