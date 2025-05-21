import { BuildConfig } from "@zk-game-dao/ui/tailwind.config";
import { resolve } from "path";

export default BuildConfig({
  contentDirs: [resolve(__dirname, "src")],
});
