import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react'
import fs from "vite-plugin-fs";
import svgrPlugin from 'vite-plugin-svgr'

// https://vitejs.dev/config/
export default defineConfig({
    base: '/',
    plugins: [fs(),
        svgrPlugin({
            svgrOptions: {
                icon: true
                // ...svgr options (https://react-svgr.com/docs/options/)
            }
        }), react()],
    server: {
        open: true,
    },
    resolve: {
        preserveSymlinks: true
    }
})
