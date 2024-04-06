/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    fontFamily: {
      sans: ['Inter', 'sans-serif'],
    },
    colors: {
      white: "#FFFFFF",
      primary: "#11834F",
      secondary: "#BDFBBA",
      "accent-light": "#BDEBFB",
      "accent-blue": "#1DB1F7",
      "accent-purple": "#907BFD",
      "accent-skin": "#E9D8FC",
    }
  },
  plugins: [],
}

