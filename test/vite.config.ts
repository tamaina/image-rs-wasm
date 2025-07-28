import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vite.dev/config/
export default defineConfig({
	base: '/image-rs-wasm/',
  plugins: [vue()],
})
