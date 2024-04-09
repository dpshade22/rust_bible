/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
  ],
  safelist: [{
    pattern: /^(text|bg)-[A-Za-z]+-(\d+)$/,
    variants: ['hover', 'focus'],
  }, {
    pattern: /^rounded-(r|l)-lg$/,
  }, {
    pattern: /^m[A-Za-z]-(\d+)$/,
  }, {
    pattern: /^font-[A-Za-z]+$/,
  },

    'text-gray-*', 'rotate-*', 'whitespace-nowrap', 'rounded-b-lg', 'no-scrollbar', 'hidden'],
  theme: {
    extend: {

      animation: {
        'spin-slow': 'spin 3s linear infinite',
      },
      spacing: {
        '10%': '10%',
        '20%': '20%',
        '25%': '25%',
        '30%': '30%',
        '40%': '40%',
        '50%': '50%',
        '60%': '60%',
        '70%': '70%',
        '75%': '75%',
        '80%': '80%',
        '90%': '90%',
        '4': '1rem',
      },
      padding: {
        '4': '1rem',
      },
    },
  },
  plugins: [],
}
