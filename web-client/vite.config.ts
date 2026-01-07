import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';

export default defineConfig({
    plugins: [solidPlugin()],
    server: {
        port: 3000,
        proxy: {
            '/api': {
                target: 'http://localhost:9095',
                changeOrigin: true,
            },
            '/playback': {
                target: 'http://localhost:9092',
                changeOrigin: true,
            },
        },
    },
    build: {
        target: 'esnext',
    },
});
