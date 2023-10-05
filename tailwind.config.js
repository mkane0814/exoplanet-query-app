/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"]
  },
  daisyui: {
    themes: [
      {
        mytheme: {
          "primary": "#0e4394",
          "secondary": "#a8aabc",
          "accent": "#92389c",
          "neutral": "#6b74a7",
          "base-100": "#1d232a",
          "info": "#3abff8",
          "success": "#36d399",
          "warning": "#fbbd23",
          "error": "#f87272",
        }
      }
    ],
  },
  plugins: [require("daisyui")],
}
