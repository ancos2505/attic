import { defineConfig } from 'vite'
import { VitePWA } from 'vite-plugin-pwa'
import subresourceIntegrity from 'vite-plugin-subresource-integrity'
import { svelte } from '@sveltejs/vite-plugin-svelte'


// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    VitePWA(),
    subresourceIntegrity(),
    svelte()
  ],
})
