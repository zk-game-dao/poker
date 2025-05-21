import { memo, useCallback, useState } from 'react';

import { GameType, TableConfig } from '@declarations/table_index/table_index.did';
import {
  Currency, CurrencyInputComponent, CurrencyType, CurrencyTypeInputComponent, IsSameCurrencyType,
  RealCurrencyInputComponent
} from '@zk-game-dao/currency';
import {
  List, matchRustEnum, StepComponentProps, SteppedModalStep, TabsInputComponent
} from '@zk-game-dao/ui';

import { Tooltips } from '../../../../tournament/tooltips';
import { IsSameGameType } from '../../../../utils/compare';

type Value = Pick<TableConfig, "currency_type" | "game_type">;


const BlindsInput = memo<{
  gameType: GameType;
  currencyType: CurrencyType;
  onChange(gameType: GameType): void;
}>(({ currencyType, gameType, onChange }) => {

  return matchRustEnum(gameType)({
    PotLimit: (small_blind) =>
      <List>
        <CurrencyInputComponent
          label="Small Blind"
          value={small_blind}
          min={0n}
          onChange={(v) => onChange({ PotLimit: v })}
          currencyType={currencyType}
        />
        <CurrencyInputComponent
          label="Big Blind"
          value={small_blind * 2n}
          onChange={(v) => onChange({ PotLimit: v })}
          currencyType={currencyType}
          disabled
        />
      </List>,
    NoLimit: (small_blind) =>
      <List>
        <CurrencyInputComponent
          label="Small Blind"
          value={small_blind}
          min={0n}
          onChange={(v) => onChange({ NoLimit: v })}
          currencyType={currencyType}
        />
        <CurrencyInputComponent
          label="Big Blind"
          value={small_blind * 2n}
          min={0n}
          onChange={(v) => onChange({ NoLimit: v })}
          currencyType={currencyType}
          disabled
        />
      </List>,
    SpreadLimit: ([min_bet, max_bet]) =>
      <List>
        <CurrencyInputComponent
          label="Min Bet"
          value={min_bet}
          min={0n}
          onChange={(v) => onChange({ SpreadLimit: [v, max_bet] })}
          currencyType={currencyType}
        />
        <CurrencyInputComponent
          label="Max Bet"
          value={max_bet}
          min={0n}
          onChange={(v) => onChange({ SpreadLimit: [min_bet, v] })}
          currencyType={currencyType}
        />
      </List>,
    FixedLimit: ([small_bet, big_bet]) =>
      <List>
        <CurrencyInputComponent
          label="Small Bet"
          value={small_bet}
          min={0n}
          onChange={(v) => onChange({ FixedLimit: [v, big_bet] })}
          currencyType={currencyType}
        />
        <CurrencyInputComponent
          label="Big Bet"
          value={big_bet}
          min={0n}
          onChange={(v) => onChange({ FixedLimit: [small_bet, v] })}
          currencyType={currencyType}
        />

      </List>,
  });
},
  (prevProps, nextProps) =>
    IsSameCurrencyType(prevProps.currencyType, nextProps.currencyType) &&
    IsSameGameType(prevProps.gameType, nextProps.gameType) &&
    prevProps.onChange === nextProps.onChange
);
BlindsInput.displayName = "BlindsInput";

export const GameTypeStepComponent = memo<StepComponentProps<Value> & { hideFake?: boolean; }>(({ data, patch, hideFake = false }) => {
  const [selectedGameType, setSelectedGameType] = useState(
    Object.keys(data.game_type || { NoLimit: null })[0],
  );

  const [limits, setLimits] = useState({
    NoLimit: 0n,
    FixedLimit: (data.game_type && "FixedLimit" in data.game_type
      ? [data.game_type.FixedLimit[0], data.game_type.FixedLimit[1]]
      : []) as [bigint | undefined, bigint | undefined],
    SpreadLimit: (data.game_type && "SpreadLimit" in data.game_type
      ? [data.game_type.SpreadLimit[0], data.game_type.SpreadLimit[1]]
      : []) as [bigint | undefined, bigint | undefined],
  });

  const propagate = useCallback(
    (gameType: string, _limits: Partial<typeof limits>) => {
      const nLimits = { ...limits, ..._limits };
      setLimits(nLimits);
      let game_type: GameType | undefined;
      switch (gameType) {
        case "NoLimit":
          game_type = { NoLimit: nLimits.NoLimit as bigint };
          break;
        case "SpreadLimit":
          if (
            nLimits.SpreadLimit?.[0] === undefined ||
            nLimits.SpreadLimit?.[1] === undefined
          )
            break;
          game_type = { SpreadLimit: nLimits.SpreadLimit as [bigint, bigint] };
          break;
        case "FixedLimit":
          if (
            nLimits.FixedLimit?.[0] === undefined ||
            nLimits.FixedLimit?.[1] === undefined
          )
            break;
          game_type = { FixedLimit: nLimits.FixedLimit as [bigint, bigint] };
          break;
      }
      patch({
        game_type,
      });
    },
    [limits, patch],
  );

  return (
    <>
      {hideFake ? (
        <RealCurrencyInputComponent
          label={<>Token <Tooltips.token /></>}
          value={!data.currency_type || 'Fake' in data.currency_type ? undefined : data.currency_type.Real}
          onChange={(currency: Currency) => patch({ currency_type: currency && { Real: currency } })}
        />
      ) : (
        <CurrencyTypeInputComponent
          label={<>Token <Tooltips.token /></>}
          value={data.currency_type ?? { Real: { ICP: null } }}
          onChange={(currency_type: CurrencyType) => patch({ currency_type })}
        />
      )}
      <TabsInputComponent
        label="Game type"
        value={selectedGameType}
        onChange={(newGameType) => {
          setSelectedGameType(newGameType);
          propagate(newGameType, {});
        }}
        tabs={[
          { value: "NoLimit", label: "No limit" },
          { value: "SpreadLimit", label: "Spread limit" },
          { value: "FixedLimit", label: "Fixed limit" },
          { value: "PotLimit", label: "Pot limit" },
        ]}
      />
      <BlindsInput
        gameType={data.game_type ?? { NoLimit: 0n }}
        currencyType={data.currency_type ?? { Real: { ICP: null } }}
        onChange={(game_type) => patch({ game_type })}
      />
    </>
  );
});
GameTypeStepComponent.displayName = "GameTypeStepComponent";

export const Config: SteppedModalStep<Value> = {
  title: "Set the type of game",
  Component: GameTypeStepComponent,
  isValid: ({ game_type }) => {
    if (!game_type) return ["Game type is required"];
    if ("NoLimit" in game_type) {
      if (game_type.NoLimit <= 0) return ["Bets must be greater than 0"];
      return true;
    }
    if ("SpreadLimit" in game_type) {
      const [min, max] = game_type.SpreadLimit;
      if (min < 0 || max < 0) return ["Bets must be greater than 0"];
      if (min >= max) return ["Max bet must be greater than min bet"];
      return true;
    }
    if ("FixedLimit" in game_type) {
      const [small, big] = game_type.FixedLimit;
      if (small < 0 || big < 0) return ["Bets must be greater than 0"];
      if (small >= big) return ["Big bet must be greater than small bet"];
      return true;
    }
    return ["Invalid game type"];
  },
  defaultValues: {
    currency_type: { Real: { ICP: null } },
    game_type: { NoLimit: 0n },
  }
};
