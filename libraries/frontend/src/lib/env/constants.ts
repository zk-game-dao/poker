import { IsDev, SelectEnv } from "@zk-game-dao/ui";

export const SOLANA_RPC_URL = SelectEnv({
  development: "http://127.0.0.1:8899",
  production: "https://api.devnet.solana.com",
  staging: "https://api.devnet.solana.com",
});

export const IC_HOST = IsDev ? "http://127.0.0.1:4943/" : "https://ic0.app";

export const APIUrl = SelectEnv({
  development: "http://0.0.0.0:3000",
  production: "https://app-uoh-lw-green-shadow-1884.fly.dev",
  staging: "https://app-uoh-lw.fly.dev",
});
