import { CurrencyMeta } from "@zk-game-dao/currency";
import { BigIntToFloat, BigIntToString, FloatToBigInt } from "./bigint";

export const FloatToTokenAmount = <
  Param extends number | undefined = number | undefined,
  Return = Param extends number ? bigint : undefined,
>(
  f: Param,
  meta: Pick<CurrencyMeta, "decimals">
): Return => FloatToBigInt<Param, Return>(f, meta.decimals);

export const TokenAmountToFloat = <
  Param extends bigint | undefined = bigint | undefined,
  Return = Param extends bigint ? number : undefined,
>(
  amount: Param,
  meta: Pick<CurrencyMeta, "decimals">
): Return => BigIntToFloat<Param, Return>(amount, meta.decimals);

export const TokenAmountToString = (
  amount: bigint,
  meta: Pick<CurrencyMeta, "decimals">
) => BigIntToString(amount, meta.decimals).toString();
