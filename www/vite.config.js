import { defineConfig } from "vite";
import config from "./package.json";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

const htmlPlugin = (version) => {
  return {
    name: "html-transform",
    transformIndexHtml(html) {
      return html
        .replace("%APP_VERSION%", version)
        .replace("%COMMIT_SHA%", process.env.GITHUB_SHA?.substring(0, 8))
        .replace("%GEOCODE_API_KEY%", process.env.GEOAPIFY_API_KEY);
    },
  };
};

export default defineConfig({
  build: {
    outDir: "../dist",
  },
  plugins: [htmlPlugin(config.version), wasm(), topLevelAwait()],
});
