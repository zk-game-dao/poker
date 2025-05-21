/// <reference types="vite/client" />
import { config } from "dotenv";
import { resolve } from "node:path";
import { defineConfig } from "vite";

import { buildConfig } from "./vite-utils";

config({ path: "../../.env" });

type App = "pure-poker" | "zk-poker";

const activeApp: App = (process.env.DEV_APP as App) ?? "pure-poker";

export default defineConfig({
  root: resolve(__dirname, "apps", activeApp),
  ...buildConfig({
    version: "0.0.0",
  }),
});
