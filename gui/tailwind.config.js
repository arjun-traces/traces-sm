/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        background: '#0f172a',
        surface: '#1e293b',
        primary: '#38bdf8',
        secondary: '#818cf8',
        accent: '#c084fc',
        danger: '#f43f5e',
        success: '#10b981',
        warning: '#f59e0b',
        text: '#f8fafc',
        muted: '#94a3b8'
      }
    },
  },
  plugins: [],
}
