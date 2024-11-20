/// <reference types="bun-types" />
import vue from "@vitejs/plugin-vue";
import fs from "node:fs";
import { defineConfig } from "vite";

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  server: {
    host: "0.0.0.0",
    port: 5173,
    https: {
      key: fs.readFileSync(".certificate/key.pem"),
      cert: fs.readFileSync(".certificate/cert.pem"),
    },
  },
});
