import { useIsBTC } from "@zk-game-dao/currency";
import { useMemo } from "react";

type WordingType = {
  product: string;
};

const Wording: Record<"pp" | "zkp", WordingType> = {
  pp: {
    product: "Pure Poker",
  },
  zkp: {
    product: "zkPoker",
  },
};

export const useWording = () => {
  const isBTC = useIsBTC();
  return useMemo(() => Wording[isBTC ? "pp" : "zkp"], [isBTC]);
};
