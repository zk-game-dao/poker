import { CustomTournamentSpeedType, NewTournament } from '@declarations/tournament_index/tournament_index.did';
import { CurrencyInputComponent } from '@zk-game-dao/currency';
import {
  DropdownInputComponent,
  List,
  NumberInputComponent,
  StepComponentProps,
  SteppedModalStep,
  TimeInputComponent,
} from '@zk-game-dao/ui';
import { memo, useMemo } from 'react';

import { Tooltips } from '../../../tooltips';

type BuyInStepValues = Pick<NewTournament, "starting_chips" | 'buy_in' | 'currency' | 'speed_type' | 'tournament_type'>;

const CustomDefaults: CustomTournamentSpeedType = {
  level_duration_ns: 0n,
  ante_start_level: 0,
  ante_percentage: 0,
  blind_multiplier: 0,
  max_levels: 0,
  initial_blind_percentage: 0,
};

const BuyStepComponent = memo<StepComponentProps<BuyInStepValues>>(({ data, patch }) => {

  // const currency = CurrencyTypeToCurrency(data.currency ?? { Real: { ICP: null } });

  const [levels, type] = useMemo((): [number, 'Regular' | 'Turbo' | 'HyperTurbo' | 'Custom'] => {
    if (!data.speed_type) return [0, 'Regular'];
    if ('Regular' in data.speed_type) return [data.speed_type.Regular, 'Regular'];
    if ('Turbo' in data.speed_type) return [data.speed_type.Turbo, 'Turbo'];
    if ('HyperTurbo' in data.speed_type) return [data.speed_type.HyperTurbo, 'HyperTurbo'];
    if ('Custom' in data.speed_type) return [0, 'Custom'];
    return [0, 'Regular'];
  }, [data.speed_type]);

  const customParams = useMemo((): CustomTournamentSpeedType => {
    if (!data.speed_type || !('Custom' in data.speed_type)) return CustomDefaults;
    return data.speed_type.Custom;
  }, [data.speed_type]);

  const patchCustom = (p?: Partial<CustomTournamentSpeedType>) => patch({
    speed_type: {
      Custom: {
        ...customParams,
        ...p,
      }
    }
  });
  const setType = (type: 'Turbo' | 'Normal' | 'HyperTurbo' | 'Custom') => {
    if (type === 'Custom') return patchCustom();
    patch({ speed_type: { [type]: levels } as any });
  };
  const setLevels = (levels: number) => patch({ speed_type: { [type]: levels } as any });

  return (
    <>
      <List label="Buy-in Details">

        <CurrencyInputComponent
          label={
            <>
              Starting chips{" "}
              <Tooltips.buy_in_starting_chips />
            </>
          }
          value={data.starting_chips}
          currencyType={{ Fake: null }}
          onChange={(starting_chips) => patch({ starting_chips })}
        />

        {!(data.tournament_type && 'Freeroll' in data.tournament_type) && (
          <CurrencyInputComponent
            label={
              <>
                Buy-in{" "}
                <Tooltips.buy_in />
              </>
            }
            currencyType={data.currency || { Real: { ICP: null } }}
            value={data.buy_in}
            onChange={(buy_in) => patch({ buy_in: BigInt(buy_in) })}
          />
        )}
      </List>

      <List label="Blind structure" key={`${type === 'Custom' ? 'custom' : 'preset'}`}>
        <DropdownInputComponent
          label={(
            <>
              {'Speed type '}
              <Tooltips.speed_type />
            </>

          )}
          value={type}
          options={[
            { label: "Regular", value: "Regular" },
            { label: "Turbo", value: "Turbo" },
            { label: "Hyper turbo", value: "HyperTurbo" },
            { label: "Custom", value: "Custom" },
          ]}
          onChange={v => setType(v as any)}
        />
        {type === 'Custom' ? (
          <>
            <TimeInputComponent
              label={
                <>
                  Level duration{" "}
                  <Tooltips.level_duration />
                </>
              }
              nanoseconds={customParams.level_duration_ns}
              onChangeNanoseconds={level_duration_ns => patchCustom({ level_duration_ns })}
            />
            <NumberInputComponent
              label={
                <>
                  Ante start level{" "}
                  <Tooltips.ante_start_level />
                </>
              }
              value={customParams.ante_start_level}
              step={1}
              min={0}
              onChange={ante_start_level => patchCustom({ ante_start_level })}
            />
            <NumberInputComponent
              label={
                <>
                  Ante percentage{" "}
                  <Tooltips.ante_percentage />
                </>
              }
              value={customParams.ante_percentage}
              step={1}
              min={0}
              onChange={ante_percentage => patchCustom({ ante_percentage })}
            />
            <NumberInputComponent
              label={
                <>
                  Blind multiplier{" "}
                  <Tooltips.blind_multiplier />
                </>
              }
              value={customParams.blind_multiplier}
              onChange={blind_multiplier => patchCustom({ blind_multiplier })}
            />
            <NumberInputComponent
              label={
                <>
                  Max levels{" "}
                  <Tooltips.blind_max_levels />
                </>
              }
              value={customParams.max_levels}
              step={1}
              min={0}
              onChange={max_levels => patchCustom({ max_levels })}
            />
            <NumberInputComponent
              label={
                <>
                  Initial blind percentage{" "}
                  <Tooltips.initial_blind_percentage />
                </>
              }
              value={customParams.initial_blind_percentage}
              step={1}
              min={0}
              max={99}
              onChange={initial_blind_percentage => patchCustom({ initial_blind_percentage })}
            />
          </>
        ) : (
          <NumberInputComponent
            label={
              <>
                Levels{" "}
                <Tooltips.blind_levels />
              </>
            }
            value={levels}
            onChange={(levels) => setLevels(levels)}
          />
        )}


      </List>
    </>
  );
});
BuyStepComponent.displayName = "BuyStepComponent";

export const Config: SteppedModalStep<BuyInStepValues> = {
  title: "Buy in and chips",
  defaultValues: {
    starting_chips: BigInt(0),
    buy_in: BigInt(0),
    currency: { Real: { ICP: null } },
    speed_type: {
      Regular: 0,
    }
  },
  Component: BuyStepComponent,
  isValid: ({ currency, starting_chips, buy_in, speed_type, tournament_type }) => {
    if (!currency) return ["Currency is required"];
    if (starting_chips === undefined) return ["Starting chips are required"];
    if (starting_chips <= 0n) return ["Starting chips must be greater than 0"];
    if (tournament_type && !(('Freeroll' in tournament_type))) {
      if (buy_in === undefined) return ["Buy in is required"];
      if (buy_in <= 0n) return ["Buy in must be greater than 0"];
    }
    if (speed_type === undefined) return ["Speed type is required"];

    if ('Custom' in speed_type) {
      const custom = speed_type.Custom;
      if (custom.level_duration_ns <= 0n) return ["Level duration must be greater than 0"];
      if (custom.ante_start_level <= 0) return ["Ante start level must be greater than 0"];
      if (custom.ante_percentage <= 0) return ["Ante percentage must be greater than 0"];
      if (custom.blind_multiplier <= 0) return ["Blind multiplier must be greater than 0"];
      if (custom.max_levels <= 0) return ["Max levels must be greater than 0"];
      if (custom.initial_blind_percentage <= 0) return ["Initial blind percentage must be greater than 0"];
      if (custom.initial_blind_percentage >= 100) return ["Initial blind percentage must be less than 100"];
    } else if (Object.values(speed_type).some(v => v <= 0)) return ["Levels must be greater than 0"];

    return true;
  },
};
