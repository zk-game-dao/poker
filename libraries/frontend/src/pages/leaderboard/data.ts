import { useMemo } from "react";

import { Currency, useIsBTC } from "@zk-game-dao/currency";

export type LeaderboardType = "weekdays" | "weekends";

type JackpotWrap<T> = Record<LeaderboardType, T>;

type JackpotMultiplier = {
  fromBet?: bigint;
  toBet?: bigint;
  multiplier: number;
};

type JackpotMeta = {
  currency: Currency;
  jackpots: JackpotWrap<bigint[]>;
  multipliers: JackpotMultiplier[];
};

// Experience points multipliers
const XP_MULTIPLIERS: JackpotMultiplier[] = [
  {
    multiplier: 0.5,
    toBet: 100_000n,
  },
  {
    multiplier: 1.0,
    fromBet: 100_000n,
    toBet: 1_000_000n,
  },
  {
    multiplier: 1.5,
    fromBet: 1_000_000n,
    toBet: 10_000_000n,
  },
  {
    multiplier: 2.0,
    fromBet: 10_000_000n,
    toBet: 100_000_000n,
  },
  {
    multiplier: 3.0,
    fromBet: 100_000_000n,
  },
];

const PERCENTAGE_PAYOUT = [45, 25, 15, 10, 5];

const TOTAL_ICP_PRIZE_POOL_WEEKDAY = 25n * 10n ** 8n;
const TOTAL_ICP_PRIZE_POOL_WEEKEND = 15n * 10n ** 8n;

const TOTAL_BTC_PRIZE_POOL_WEEKDAY_SATS = 150000n;
const TOTAL_BTC_PRIZE_POOL_WEEKEND_SATS = 50000n;

export const useJackpot = (): JackpotMeta => {
  const isBTC = useIsBTC();
  const currency = useMemo(
    (): JackpotMeta["currency"] => (isBTC ? { BTC: null } : { ICP: null }),
    [isBTC]
  );

  const prizePools = useMemo(
    (): JackpotWrap<bigint> =>
      isBTC
        ? {
            weekdays: TOTAL_BTC_PRIZE_POOL_WEEKDAY_SATS,
            weekends: TOTAL_BTC_PRIZE_POOL_WEEKEND_SATS,
          }
        : {
            weekdays: TOTAL_ICP_PRIZE_POOL_WEEKDAY,
            weekends: TOTAL_ICP_PRIZE_POOL_WEEKEND,
          },
    [isBTC]
  );

  const jackpots = useMemo(
    (): JackpotMeta["jackpots"] => ({
      weekdays: PERCENTAGE_PAYOUT.map(
        (percentage) => (prizePools.weekdays * BigInt(percentage)) / BigInt(100)
      ),
      weekends: PERCENTAGE_PAYOUT.map(
        (percentage) => (prizePools.weekends * BigInt(percentage)) / BigInt(100)
      ),
    }),
    [prizePools.weekdays, prizePools.weekends]
  );

  return useMemo(
    (): JackpotMeta => ({
      currency,
      jackpots,
      multipliers: XP_MULTIPLIERS,
    }),
    [currency, jackpots]
  );
};
