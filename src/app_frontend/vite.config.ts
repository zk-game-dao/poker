import { config } from "dotenv";
import { resolve } from "path";
import { defineConfig } from "vite";

import { buildConfig } from "../../libraries/frontend/vite-utils";
import { version } from "./package.json";

config({ path: "../../.env" });

export default defineConfig(
  buildConfig({ version, customPublicDir: resolve(__dirname, "public") })
);
