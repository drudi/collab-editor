/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{js,ts,jsx,tsx}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        editor: {
          bg: '#1e1e2e',
          fg: '#cdd6f4',
          gutter: '#313244',
          line: '#45475a',
          selection: '#585b70',
          cursor: '#f5e0dc',
        },
        surface: {
          base: '#181825',
          overlay: '#313244',
          elevated: '#1e1e2e',
        },
      },
    },
  },
  plugins: [],
}
