/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"]
  },
  daisyui: {
    themes: [
      "business"
    ],
  },
  plugins: [require("daisyui")],
};
