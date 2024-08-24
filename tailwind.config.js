/** @type {import('tailwindcss').Config} */
export default {
  darkMode: ['class'],
  content: [
    './pages/**/*.{ts,tsx}',
    './components/**/*.{ts,tsx}',
    './app/**/*.{ts,tsx}',
    './src/**/*.{ts,tsx}',
    './index.html'
  ],
  theme: {
    fontFamily: {
      sans: ['Inter', 'sans-serif']
    },
    extend: {
      colors: {
        background: '#071A29',
        primary: '#11834F',
        secondary: '#BDFBBA',
        'accent-light': '#BDEBFB',
        'accent-blue': '#1DB1F7',
        'accent-skin': '#E9D8FC'
      }
    }
  },
  plugins: [require('tailwindcss-animate')]
};
