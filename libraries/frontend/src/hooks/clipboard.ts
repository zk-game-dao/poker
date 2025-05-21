import { useToast } from "@zk-game-dao/ui";
import { useCallback } from "react";

export const useCopyToClipboard = (url?: string) => {
  const { addToast } = useToast();

  return useCallback(() => {
    if (!url) return;
    navigator.clipboard.writeText(url);
    addToast({ children: "Link copied" });
  }, [url]);
};
