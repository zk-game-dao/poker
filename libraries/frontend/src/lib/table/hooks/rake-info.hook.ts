import { table_index } from "@declarations/table_index";
import { GameType } from "@declarations/table_index/table_index.did";
import { Queries } from "@lib/data";
import { useQuery } from "@tanstack/react-query";
import { Currency } from "@zk-game-dao/currency";

export const useRakeInfo = (
  small_blind: bigint = 0n,
  currency: Currency,
  game_type: GameType = { NoLimit: 0n }
) =>
  useQuery({
    queryKey: Queries.rakeInfo.key(small_blind, currency, game_type),
    queryFn: async () => table_index.get_rake(small_blind, currency, game_type),
  });
