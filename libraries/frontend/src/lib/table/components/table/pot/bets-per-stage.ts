import { useMemo } from "react";

import {
  ActionLog,
  DealStage,
  SeatStatus,
  UserCards,
} from "@declarations/table_index/table_index.did";
import { Principal } from "@dfinity/principal";
import { Optional } from "@zk-game-dao/ui";

// Utility type to extract keys from the DealStage type
type ExtractKeys<T> = T extends { [K in keyof T]: null } ? keyof T : never;

// Extract all keys from the DealStage type
export const DealStageKeys = [
  "Initial",
  "Flop",
  "Turn",
  "River",
  "Showdown",
] as const;
export type DealStageKey = (typeof DealStageKeys)[number];

const ExtractDealStageKey = (stage: DealStage): DealStageKey => {
  const extractedKey = Object.keys(stage)[0] as ExtractKeys<DealStage>;
  switch (Object.keys(stage)[0] as ExtractKeys<DealStage>) {
    case "Blinds":
    case "Fresh":
    case "Opening":
      return "Initial";
    default:
      return extractedKey as DealStageKey;
  }
};

export type DealStageState = {
  players: { [playerSeatIndex: number]: bigint };
  winnings: { [playerSeatIndex: number]: bigint };
  pot: bigint;
  startingPot: bigint;
  rake: bigint;
};

export type BetsPerStage = {
  [key in DealStageKey | "Initial"]?: DealStageState;
};

export const useSeatMap = (seats: SeatStatus[]) =>
  useMemo(() => {
    const map = new Map<string, number>();
    seats.forEach((seat, index) => {
      if ("Occupied" in seat) {
        map.set(seat.Occupied.toText(), index);
      }
    });
    return map;
  }, [seats]);

export const useHasWinners = (sorted_users: Optional<UserCards[]>) =>
  useMemo(() => {
    const winners = Object.values(sorted_users)
      .flat()
      .some((v) => v.amount_won > 0);
    return winners;
  }, [sorted_users]);

export function computeBetsPerStage(
  pot: bigint,
  big_blind: bigint,
  small_blind: bigint,
  action_logs: ActionLog[],
  sorted_users: Optional<UserCards[]>,
  hasWinners: boolean,
  seatMap: Map<string, number>
): BetsPerStage {
  const _betsPerStage: BetsPerStage = {
    Initial: { startingPot: 0n, pot: 0n, players: {}, winnings: {}, rake: 0n },
  };

  const findPlayerIndex = (player: Principal) =>
    seatMap.get(player.toText()) ?? -1;

  let iterationDealStage: DealStageKey = "Initial";
  let currentBet = 0n;

  for (const log of action_logs) {
    if ("Stage" in log.action_type) {
      const newDealStage =
        ExtractDealStageKey(log.action_type.Stage.stage) ?? "Showdown";
      if (iterationDealStage === newDealStage) {
        continue;
      }
      iterationDealStage = newDealStage;
      _betsPerStage[iterationDealStage] = _betsPerStage[iterationDealStage] || {
        startingPot: 0n,
        pot: 0n,
        players: {},
        winnings: {},
        rake: 0n,
      };
      currentBet = 0n;
    }

    if (log.user_principal[0]) {
      const playerIndex = findPlayerIndex(log.user_principal[0]);
      if (playerIndex === -1) continue;

      if (!_betsPerStage[iterationDealStage]!.players[playerIndex])
        _betsPerStage[iterationDealStage]!.players[playerIndex] = 0n;
      if (!_betsPerStage[iterationDealStage]!.winnings[playerIndex])
        _betsPerStage[iterationDealStage]!.winnings[playerIndex] = 0n;

      if ("Call" in log.action_type)
        _betsPerStage[iterationDealStage]!.players[playerIndex] = currentBet;
      else if ("Raise" in log.action_type) {
        currentBet = log.action_type.Raise.amount;
        _betsPerStage[iterationDealStage]!.players[playerIndex] = currentBet;
      } else if ("BigBlind" in log.action_type) {
        if (currentBet < big_blind) currentBet = big_blind;
        _betsPerStage[iterationDealStage]!.players[playerIndex] = big_blind;
      } else if ("SmallBlind" in log.action_type) {
        if (currentBet < small_blind) currentBet = small_blind;
        _betsPerStage[iterationDealStage]!.players[playerIndex] = small_blind;
      } else if ("AllIn" in log.action_type) {
        if (currentBet < log.action_type.AllIn.amount)
          currentBet = log.action_type.AllIn.amount;
        _betsPerStage[iterationDealStage]!.players[playerIndex] =
          log.action_type.AllIn.amount;
      }
    }
  }

  if (hasWinners) {
    _betsPerStage[iterationDealStage]!.pot = pot;
    sorted_users[0]?.forEach(({ id, amount_won }) => {
      const playerIndex = findPlayerIndex(id);
      if (playerIndex === -1) return;
      if (!_betsPerStage[iterationDealStage]!.winnings[playerIndex])
        _betsPerStage[iterationDealStage]!.winnings[playerIndex] = 0n;
      _betsPerStage[iterationDealStage]!.winnings[playerIndex] += amount_won;
    });
  }

  let previousStageBets: BetsPerStage[DealStageKey] | undefined;
  // Calculate pots and starting pots
  for (const [, stageBets] of Object.entries(_betsPerStage)) {
    if (stageBets) {
      stageBets.startingPot = 0n;
      if (previousStageBets) {
        stageBets.pot =
          previousStageBets.startingPot +
          Object.values(previousStageBets?.players).reduce(
            (acc, bet) => acc + bet,
            0n
          );
        stageBets.startingPot += stageBets.pot;
      }
      const totalWinnings = Object.values(stageBets.winnings).reduce(
        (acc, bet) => acc + bet,
        0n
      );
      if (totalWinnings > 0n) {
        stageBets.rake = stageBets.pot - totalWinnings;
        stageBets.pot = 0n;
      }

      previousStageBets = stageBets;
    }
  }

  return _betsPerStage;
}
