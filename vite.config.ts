import { reactRouter } from "@react-router/dev/vite"
import tailwindcss from "@tailwindcss/vite"
import { defineConfig } from "vite"
import tsconfigPaths from "vite-tsconfig-paths"

// Set chokidar environment variables to avoid EMFILE errors on macOS
// // @ts-expect-error process is a nodejs global
// process.env.CHOKIDAR_USEPOLLING = 'true';
// // @ts-expect-error process is a nodejs global
// process.env.CHOKIDAR_INTERVAL = '300';

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [
    reactRouter({}),
    tsconfigPaths(),
    tailwindcss(),
    {
      name: "debug-watch",
      configureServer(server) {
        server.watcher.on("ready", () => {
          const watched = server.watcher.getWatched()
          const paths = Object.keys(watched)
          const watchCount = paths.reduce((sum, dir) => sum + watched[dir].length, 0)
          console.log("\n🔍 [DEBUG] Vite watcher ready")
          console.log(`📊 [DEBUG] Watching ${watchCount} files in ${paths.length} directories`)
          console.log(`📁 [DEBUG] Sample dirs:`, paths.slice(0, 10))
        })

        server.watcher.on("error", (error: Error) => {
          console.error("❌ [DEBUG] Watcher error:", error.message)
          console.error("Stack:", error.stack)
        })
      },
    },
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**", "**/node_modules/**", "**/target/**", "**/.git/**"],
    },
  },
}))
