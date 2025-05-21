import { matchRustEnum } from "@zk-game-dao/ui";
import { GameType } from "@declarations/table_index/table_index.did";

const CreateSerializer = <T>(
  serialize: (value: T) => string = JSON.stringify,
  deserialize: (value: string) => T = JSON.parse
) => ({
  serialize,
  deserialize,
  validate: (value: T) => deserialize(serialize(value)),
});

export const GameTypeSerializer = CreateSerializer<GameType>(
  (value) =>
    matchRustEnum(value)({
      NoLimit: (pl): string => [0, pl].join(":"),
      FixedLimit: (pl): string => [0, pl.join("|")].join(":"),
      PotLimit: (pl): string => [0, pl].join(":"),
      SpreadLimit: (pl): string => [0, pl.join("|")].join(":"),
    }),
  (value) => {
    const [type, limit] = value.split(":");
    switch (type) {
      case "0":
        return { NoLimit: BigInt(limit) };
      case "1": {
        const FixedLimit = limit.split("|").map(BigInt) as [bigint, bigint];
        return { FixedLimit };
      }
      case "2":
        return { PotLimit: BigInt(limit) };
      case "3": {
        const SpreadLimit = limit.split("|").map(BigInt) as [bigint, bigint];
        return { SpreadLimit };
      }
      default:
        throw new Error(`Invalid game type: ${value}`);
    }
  }
);
