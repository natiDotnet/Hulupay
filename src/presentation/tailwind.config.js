/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{rs,html}", "./index.html"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        brand: {
          500: "#6366F1",
          600: "#4F46E5"
        }
      },
      boxShadow: {
        panel: "0 10px 30px rgba(2, 6, 23, 0.35)"
      }
    }
  },
  plugins: []
};
