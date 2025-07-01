import { createActor } from "@declarations/tournament_canister";
import { TournamentData } from "@declarations/tournament_canister/tournament_canister.did";
import { tournament_index } from "@declarations/tournament_index";
import { useQueries, useQuery } from "@tanstack/react-query";
import { useMemo } from "react";

import { Queries } from "../../data";
import { callActorMutation } from "../../utils/call-actor-mutation";
import { useIsBTC } from "@zk-game-dao/currency";

export enum TournamentTypeFilter {
  BuyIn = 0,
  SitAndGo = 1,
  Freeroll = 2,
  SpinAndGo = 3,
  Bounty = 4,
}

export type TournamentTypeMeta = {
  label: string;
  value: TournamentTypeFilter;
  locked?: boolean;
};

export const TournamentTypes: TournamentTypeMeta[] = [
  { label: "Buy-in", value: 0 },
  { label: "Sit & Go", value: 1 },
  { label: "Freeroll", value: 2 },
  { label: "Spin & Go", value: 3 },

  { label: "Bounty", value: 4, locked: true },
];

export const fetchTournamentsOfType = async (type: TournamentTypeFilter, isBTC: boolean) => {
  if (TournamentTypes[type].locked) return [];
  const tournaments = (await tournament_index.get_active_tournaments([type])).filter(
    (t) => {
      const isBTCTournament = "Real" in t.currency && "BTC" in t.currency.Real;
      return isBTC ? isBTCTournament : !isBTCTournament;
    }
  );
  return await Promise.all(
    tournaments.map((t) =>
      callActorMutation(createActor(t.id), "get_tournament")
    )
  );
};

export const useGetTournamentsOfType = (type: TournamentTypeFilter, isBTC: boolean) =>
  useQuery({
    queryKey: Queries.tournaments.key(isBTC, type),
    queryFn: async (): Promise<TournamentData[]> =>
      fetchTournamentsOfType(type, isBTC),
    refetchInterval: 10000,
  });

export const useGetAllTournaments = () => {

  const isBTC = useIsBTC();

  return useQueries({
    queries: TournamentTypes.map((meta) => ({
      queryKey: Queries.tournaments.key(isBTC, meta.value),
      queryFn: async (): Promise<{
        meta: TournamentTypeMeta;
        tournaments: TournamentData[];
      }> => ({
        meta,
        tournaments: await fetchTournamentsOfType(meta.value, isBTC),
      }),
      initialData: { meta, tournaments: [] },
      refetchInterval: 10000,
    })),
    combine: (results) =>
      results.map(
        (
          result,
          tournamentTypeFilter
        ): {
          meta: TournamentTypeMeta;
          tournaments: TournamentData[];
          error: Error | null;
          isPending: boolean;
        } => ({
          meta: result.data?.meta ?? TournamentTypes[tournamentTypeFilter],
          tournaments: result.data?.tournaments ?? [],
          error: result.error,
          isPending: result.isPending,
        })
      ),
  })
};

export const isTournamentConsideredActive = ({ state }: TournamentData) =>
  !("Cancelled" in state || "Completed" in state);

export const useAreTournamentsActive = (type?: TournamentTypeFilter) => {
  const tournaments = useGetAllTournaments();
  return useMemo(() => {
    if (type === undefined)
      return tournaments
        .flatMap((v) => v.tournaments)
        .some(isTournamentConsideredActive);
    return !!tournaments
      .find((t) => t.meta.value === type)
      ?.tournaments.some(isTournamentConsideredActive);
  }, [type, tournaments]);
};
