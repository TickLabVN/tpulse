/** @type {import('tailwindcss').Config} */

export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    fontFamily: {
      sans: ['Inter', 'sans-serif'],
    },
    colors: {
      background: "#071A29",
      white: "#FFFFFF",
      primary: "#11834F",
      secondary: "#BDFBBA",
      navy: "#012F2F",
      green: "#2A9665",
      gray: "#779A90",
      "dark-gray": "#404040",
      "light-gray": "#D0D7DE",
      "accent-light": "#BDEBFB",
      "accent-blue": "#1DB1F7",
      "accent-purple": "#907BFD",
      "accent-skin": "#E9D8FC",
    }
  },
  plugins: [],
}

