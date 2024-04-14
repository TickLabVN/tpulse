/** @type {import('tailwindcss').Config} */
export default {
  darkMode: ["class"],
  content: [
    './pages/**/*.{ts,tsx}',
    './components/**/*.{ts,tsx}',
    './app/**/*.{ts,tsx}',
    './src/**/*.{ts,tsx}',
    './index.html'
  ],
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
    },
    container: {
      center: true,
      padding: "2rem",
      screens: {
        "2xl": "1400px",
      },
    },
    extend: {
      keyframes: {
        "accordion-down": {
          from: { height: "0" },
          to: { height: "var(--radix-accordion-content-height)" },
        },
        "accordion-up": {
          from: { height: "var(--radix-accordion-content-height)" },
          to: { height: "0" },
        },
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
      },
    },
  },
  plugins: [require("tailwindcss-animate")],
}

