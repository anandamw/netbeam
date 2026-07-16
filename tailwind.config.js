/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#FACC15',
          hover: '#FFD54A',
          press: '#EAB308',
        },
        background: {
          main: '#121212',
          secondary: '#1A1A1A',
        },
        card: '#1E1E1E',
        sidebar: '#181818',
        navbar: '#1D1D1D',
        text: {
          primary: '#FFFFFF',
          secondary: '#BDBDBD',
          muted: '#8A8A8A',
          disabled: '#666666',
        },
        border: {
          DEFAULT: '#2F2F2F',
          divider: '#343434',
        },
        status: {
          success: '#22C55E',
          warning: '#FACC15',
          error: '#EF4444',
          info: '#3B82F6',
        }
      },
      fontFamily: {
        sans: ['Poppins', 'sans-serif'],
      },
      boxShadow: {
        'sm-ui': '0 2px 6px rgba(0,0,0,.2)',
        'md-ui': '0 8px 20px rgba(0,0,0,.3)',
        'lg-ui': '0 20px 40px rgba(0,0,0,.35)',
      },
      borderRadius: {
        'btn': '10px',
        'input': '10px',
        'card': '16px',
        'modal': '18px',
      }
    },
  },
  plugins: [],
}
