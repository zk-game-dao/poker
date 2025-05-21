import {
  _SERVICE,
  TableBalancer,
  TournamentData,
  TournamentSizeType,
} from "@declarations/tournament_canister/tournament_canister.did";
import { ActorSubclass } from "@dfinity/agent";
import { useQuery } from "@tanstack/react-query";
import { differenceInMilliseconds } from "date-fns";
import { useMemo } from "react";

import { matchRustEnum } from "../../utils/rust";
import { BigIntTimestampToDate } from "../../utils/time";

export const useNextMultitableRebalanceDateTime = (
  data?: TournamentData,
  actor?: ActorSubclass<_SERVICE>
) => {
  const tableBalancer = useMemo(() => {
    if (!data) return;
    return matchRustEnum(
      matchRustEnum(data.tournament_type)({
        BuyIn: (t): TournamentSizeType => t,
        SpinAndGo: ([t]): TournamentSizeType => t,
        SitAndGo: (t): TournamentSizeType => t,
        Freeroll: (t): TournamentSizeType => t,
      })
    )({
      MultiTable: ([, t]): TableBalancer | undefined => t,
      SingleTable: (): TableBalancer | undefined => undefined,
    });
  }, [JSON.stringify(data)]);

  const { data: lastBalanceTimestampNS } = useQuery({
    queryKey: ["get-multitable-rebalance-time", data?.id.toText() ?? "-"],
    queryFn: async () => {
      if (!actor || !tableBalancer) return null;
      const time = await actor.get_last_balance_timestamp();
      if (!time) return null;
      return time + tableBalancer.balance_interval_ns;
    },
    refetchInterval: ({ state: { data } }) => {
      if (!data) {
        if (!tableBalancer?.balance_interval_ns) return 0;
        return Number(tableBalancer?.balance_interval_ns ?? 0) / 1_000_000;
      }
      return Math.abs(
        differenceInMilliseconds(BigIntTimestampToDate(data), new Date())
      );
    },
  });

  return useMemo(() => {
    if (!lastBalanceTimestampNS) return;
    return BigIntTimestampToDate(lastBalanceTimestampNS);
  }, [lastBalanceTimestampNS]);
};
