import { resolve } from "path";
import environment from "vite-plugin-environment";
import { Mode, plugin as mdPlugin } from "vite-plugin-markdown";
import { nodePolyfills } from "vite-plugin-node-polyfills";
import { VitePWA } from "vite-plugin-pwa";
import tsconfigPaths from "vite-tsconfig-paths";

import { NodeGlobalsPolyfillPlugin } from "@esbuild-plugins/node-globals-polyfill";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { DynamicPublicDirectory } from "vite-multiple-assets";
import { UserConfig } from "vite";

export const pokerFrontendDir = resolve(__dirname);

export const manualChunks = {
  ui: [
    "react-markdown",
    "framer-motion",
    "@heroicons/react",
    "@tanstack/react-query",
  ],
  zkpUI: ["@zk-game-dao/ui"],
  zkpCurrency: ["@zk-game-dao/currency"],
  // dfinity: [
  //   "@dfinity/agent",
  //   "@dfinity/auth-client",
  //   "@dfinity/candid",
  //   "@dfinity/cketh",
  //   "@dfinity/ledger-icp",
  //   "@dfinity/ledger-icrc",
  //   "@dfinity/nns",
  //   "@dfinity/principal",
  //   "@dfinity/utils",
  //   "@dfinity/verifiable-credentials",
  //   "ic-stoic-identity",
  // ],
  omnisat: ["@omnisat/lasereyes", "@omnisat/lasereyes-core"],
  siwb: ["ic-siwb-lasereyes-connector"],
  web3auth: [
    "@web3auth/auth-adapter",
    "@web3auth/base",
    "@web3auth/base-provider",
    "@web3auth/default-evm-adapter",
    "@web3auth/ethereum-provider",
    "@web3auth/no-modal",
    "@web3auth/passkeys-sfa-plugin",
    "@web3auth/solana-provider",
  ],
  // crypto: ["crypto", "bs58", "buffer", "@noble/hashes"],
  ipfs: ["helia", "ipfs-core", "pinata-web3"],
  ether: ["ethers", "web3"],
};

export const buildConfig = ({
  version,
  customPublicDir,
}: {
  version: string;
  customPublicDir?: string;
}): UserConfig => ({
  build: {
    emptyOutDir: true,
    target: "esnext",
    rollupOptions: {
      output: {
        manualChunks,
      },
    },
  },
  optimizeDeps: {
    esbuildOptions: {
      plugins: [
        NodeGlobalsPolyfillPlugin({
          buffer: true,
          process: true,
        }),
      ],
    },
    include: ["react", "react-dom", "react-router-dom"],
    exclude: [
      "@storybook/builder-vite",
      "vite-plugin-node-polyfills/shims/buffer",
      "vite-plugin-node-polyfills/shims/global",
      "vite-plugin-node-polyfills/shims/process",
      "chromatic",
      "buffer",
    ],
  },
  server: {
    proxy: {
      "/api": {
        target: "http://127.0.0.1:4943",
        changeOrigin: true,
      },
    },
  },
  plugins: [
    tailwindcss(),
    react(),

    environment("all", { defineOn: "process.env", prefix: "CANISTER_" }),
    environment("all", { defineOn: "process.env", prefix: "II_" }),
    environment("all", { defineOn: "process.env", prefix: "DFX_" }),

    environment("all", { defineOn: "import.meta.env", prefix: "CANISTER_" }),
    environment("all", { defineOn: "import.meta.env", prefix: "II_" }),
    environment("all", { defineOn: "import.meta.env", prefix: "DFX_" }),

    VitePWA({
      srcDir: resolve(pokerFrontendDir, "src"),
      filename: "sw.ts",
      strategies: "injectManifest",
      injectRegister: false,
      manifest: false,
      injectManifest: {
        injectionPoint: undefined,
      },
      devOptions: {
        enabled: process.env.NODE_ENV === "development",
      },
    }),

    tsconfigPaths(),
    nodePolyfills(),
    mdPlugin({
      mode: [Mode.MARKDOWN],
    }),
    customPublicDir
      ? DynamicPublicDirectory(
          [resolve(pokerFrontendDir, "public"), customPublicDir].map(
            (v) => `${v}/**/*`
          ),
          {
            ssr: true,
          }
        )
      : undefined,
  ].filter(Boolean),
  resolve: {
    alias: [
      {
        find: "crypto",
        replacement: "empty-module",
      },
      {
        find: "react-router-dom",
        replacement: "react-router-dom",
      },
    ],
  },
  define: {
    "process.env.VERSION": JSON.stringify(version),
    "process.env.NODE_ENV": JSON.stringify(
      process.env.NODE_ENV || "development"
    ),
  },

  publicDir: customPublicDir ? false : resolve(pokerFrontendDir, "public"),
  // publicDir: resolve(pokerFrontendDir, "public"),
});
