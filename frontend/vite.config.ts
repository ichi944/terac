import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  base: '../public',
  build: {
    outDir: '../public',
    rollupOptions: {
      output: {
        entryFileNames: 'js/main.js',
      }
    }
  },
  server: {
    proxy: {
      '^/$': 'http://localhost:3001',
      '/auth': 'http://localhost:3001',
      '/app': 'http://localhost:3001',
      '/graphql': 'http://localhost:3001',
    }
  }
})
