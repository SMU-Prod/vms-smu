/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        vms: {
          primary: '#3b82f6',
          secondary: '#1e40af',
          dark: '#0f172a',
          darker: '#020617',
          accent: '#22d3ee',
          success: '#22c55e',
          warning: '#f59e0b',
          danger: '#ef4444',
          muted: '#64748b',
        }
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
