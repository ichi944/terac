import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '^/$': 'http://localhost:3001',
      '/login': 'http://localhost:3001',
      '/logout': 'http://localhost:3001',
      '/auth': 'http://localhost:3001',
      '/graphql': 'http://localhost:3001',
      '/app': 'http://localhost:3001',
    }
  }
})
