import { defineConfig } from "vite";
import { VitePWA } from "vite-plugin-pwa";
import config from "./package.json";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

const htmlPlugin = (version: string) => {
  return {
    name: "html-transform",
    transformIndexHtml(html: string) {
      return html
        .replace("%APP_VERSION%", version)
        .replace("%COMMIT_SHA%", process.env.GITHUB_SHA?.substring(0, 8) ?? "")
        .replace("%GEOCODE_API_KEY%", process.env.GEOAPIFY_API_KEY ?? "");
    },
  };
};

export default defineConfig({
  build: {
    outDir: "../dist",
  },
  plugins: [
    htmlPlugin(config.version),
    VitePWA({
      devOptions: {
        enabled: true,
      },
      manifest: {
        name: "Roman Sunclock Time",
        short_name: "Sunclock",
        description:
          "Displays time following the roman sunclock time calculations (12 hours daytime, 12 hours nighttime).",
        id: "com.solova.rsct",
        theme_color: "#FFFFFF",
      },
      registerType: "autoUpdate",
      workbox: {
        globPatterns: ["**/*.css,html,js,svg,wasm"],
      },
    }),
    wasm(),
    topLevelAwait(),
  ],
});
