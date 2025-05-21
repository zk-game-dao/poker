import { addMinutes, format } from 'date-fns';
import { memo, useMemo } from 'react';

import { tournament_index } from '@declarations/tournament_index';
import {
  BlindLevel, BuyInOptions, NewTournament, TournamentSizeType, TournamentType
} from '@declarations/tournament_index/tournament_index.did';
import { useQuery } from '@tanstack/react-query';
import { CurrencyComponent, CurrencyInputComponent } from '@zk-game-dao/currency';
import {
  DateInputComponent, List, NumberInputComponent, StepComponentProps, SteppedModalStep,
  SwitchInputComponent, TimeInputComponent
} from '@zk-game-dao/ui';

import { Max } from '../../../../utils/bigint';
import { matchRustEnum } from '../../../../utils/rust';
import { BigIntTimestampToDate, DateToBigIntTimestamp } from '../../../../utils/time';
import { Tooltips } from '../../../tooltips';

type TypeStepValues = Pick<NewTournament, 'starting_chips' | "speed_type" | "tournament_type" | 'start_time' | 'currency'>;

const dateTimeFormat = `dd.MM.yyyy HH:mm`;
const fmt = (dtn: bigint) => format(BigIntTimestampToDate(dtn), dateTimeFormat);

export const defaultBuyInOptions: BuyInOptions = {
  'addon': {
    'enabled': false,
    max_addons: 1,
    'addon_chips': 0n,
    'addon_start_time': DateToBigIntTimestamp(new Date()),
    'addon_price': 0n,
    'addon_end_time': DateToBigIntTimestamp(new Date()),
  },
  'freezout': false,
  'reentry': {
    'enabled': false,
    'max_reentries': 0,
    'reentry_chips': 0n,
    'reentry_price': 0n,
    'reentry_end_timestamp': DateToBigIntTimestamp(new Date()),
  },
  'rebuy': {
    'enabled': false,
    'max_rebuys': 0,
    'rebuy_chips': 0n,
    'rebuy_price': 0n,
    'rebuy_end_timestamp': DateToBigIntTimestamp(new Date()),
    'rebuy_window_seconds': 0n,
    'min_chips_for_rebuy': 0n,

    // 'max_rebuys' : number,
    // 'rebuy_chips' : bigint,
    // 'rebuy_end_timestamp' : bigint,
    // 'min_chips_for_rebuy' : bigint,
    // 'enabled' : boolean,
    // 'rebuy_price' : bigint,
    // 'rebuy_window_seconds' : bigint,
  },
}

export const default_tournament_type: TournamentType = {
  BuyIn: {
    SingleTable: defaultBuyInOptions
  }
};

const BigBlindWarning = memo<{
  blinds?: BlindLevel;
  timestamp?: bigint;
}>(({ blinds, timestamp }) => {
  if (!blinds || !timestamp) return null;
  return (
    <div className='px-4 type-callout text-material-medium-2'>
      The big blind at {fmt(timestamp)} is <CurrencyComponent size="small" className='inline' currencyValue={blinds.big_blind} currencyType={{ Fake: null }} /> so the rebuy chips need to be equal or above that.
    </div>
  );
}, (prevProps, nextProps) =>
  prevProps.blinds?.big_blind === nextProps.blinds?.big_blind &&
  prevProps.timestamp === nextProps.timestamp);
BigBlindWarning.displayName = "BigBlindWarning";

const TypeStepComponent = memo<StepComponentProps<TypeStepValues>>(({ data, patch }) => {

  // const type = useMemo(() => {
  //   if (!data.tournament_type) return 'BuyIn';
  //   if ('BuyIn' in data.tournament_type) return 'BuyIn';
  //   if ('SitAndGo' in data.tournament_type) return 'SitAndGo';
  //   if ('Freeroll' in data.tournament_type) return 'Freeroll';
  //   if ('SpinAndGo' in data.tournament_type) return 'SpinAndGo';
  //   return 'BuyIn';
  // }, [data.tournament_type]);

  const size = useMemo((): TournamentSizeType => {
    if (!data.tournament_type) return { SingleTable: defaultBuyInOptions };
    return matchRustEnum(data.tournament_type)({
      SpinAndGo: ([t]) => t,
      BuyIn: t => t,
      SitAndGo: t => t,
      Freeroll: t => t
    });
  }, [data.tournament_type]);

  const currentBuyInOptions = useMemo(() => matchRustEnum(size)({
    SingleTable: (t) => t,
    MultiTable: ([t]) => t
  }), [size]);

  const patchBuyInOptions = (p: (current: BuyInOptions) => BuyInOptions) => {
    if (!data.tournament_type) return;

    const nBuyInOptions = p(currentBuyInOptions);

    const nSize: TournamentSizeType = matchRustEnum(size)({
      SingleTable: (): TournamentSizeType => ({ SingleTable: nBuyInOptions }),
      MultiTable: ([, b]): TournamentSizeType => ({ MultiTable: [nBuyInOptions, b] })
    });

    const nType = matchRustEnum(data.tournament_type)({
      SpinAndGo: ([, m]): TournamentType => ({ SpinAndGo: [nSize, m] }),
      BuyIn: (): TournamentType => ({ BuyIn: nSize }),
      SitAndGo: (): TournamentType => ({ SitAndGo: nSize }),
      Freeroll: (): TournamentType => ({ Freeroll: nSize }),
    });

    patch({ tournament_type: nType });
  };

  const blindsAtRebuyEnd = useQuery({
    queryKey: [
      "blindsAtRebuyEnd",
      JSON.stringify(data.speed_type),
      JSON.stringify(currentBuyInOptions.rebuy.rebuy_end_timestamp),
      JSON.stringify(data.start_time),
      JSON.stringify(data.starting_chips)
    ],
    queryFn: async () => {
      if (
        data.speed_type === undefined ||
        currentBuyInOptions.rebuy.rebuy_end_timestamp === undefined ||
        data.start_time === undefined ||
        data.starting_chips === undefined
      ) {
        throw new Error("Required data is undefined");
      }

      const blind_levels = await tournament_index.get_blind_level_at_timestamp(
        data.speed_type,
        currentBuyInOptions.rebuy.rebuy_end_timestamp,
        data.start_time,
        data.starting_chips
      );

      const bb = blind_levels[0]?.big_blind ?? 0n;

      patchBuyInOptions(opt => ({
        ...opt,
        rebuy: {
          ...opt.rebuy,
          rebuy_chips: Max(opt.rebuy.rebuy_chips, bb)
        }
      }));
      return blind_levels;
    },
  })

  const blindsAtReentryEnd = useQuery({
    queryKey: [
      "blindsAtReentry",
      JSON.stringify(currentBuyInOptions.reentry.reentry_end_timestamp),
      JSON.stringify(data.speed_type),
      JSON.stringify(data.start_time),
      JSON.stringify(data.starting_chips),
    ],
    queryFn: async () => {
      if (
        data.speed_type === undefined ||
        currentBuyInOptions.reentry.reentry_end_timestamp === undefined ||
        data.start_time === undefined ||
        data.starting_chips === undefined
      ) {
        throw new Error("Required data is undefined");
      }

      const blind_levels = await tournament_index.get_blind_level_at_timestamp(
        data.speed_type,
        currentBuyInOptions.reentry.reentry_end_timestamp,
        data.start_time,
        data.starting_chips
      );

      const bb = blind_levels[0]?.big_blind ?? 0n;

      patchBuyInOptions(opt => ({
        ...opt,
        reentry: {
          ...opt.reentry,
          reentry_chips: Max(opt.reentry.reentry_chips, bb)
        }
      }));
      return blind_levels;
    },
  });

  const currency = useMemo(() => data.currency ?? { Real: { ICP: null } }, [data.currency]);

  return (
    <>
      <SwitchInputComponent
        label={(
          <>
            Freezout
            <Tooltips.freezout />
          </>
        )}
        checked={currentBuyInOptions.freezout}
        onChange={(freezout) => patchBuyInOptions(opt => ({
          ...opt,
          freezout,
          addon: freezout ? { ...opt.addon, enabled: false } : opt.addon,
          rebuy: freezout ? { ...opt.rebuy, enabled: false } : opt.rebuy
        }))}
      />

      <List>
        <SwitchInputComponent
          label={(
            <>
              Addon
              <Tooltips.addon />
            </>
          )}
          checked={currentBuyInOptions.addon.enabled}
          onChange={(enabled) => patchBuyInOptions(opt => ({
            ...opt,
            addon: { ...opt.addon, enabled },
            freezout: enabled ? false : opt.freezout,
          }))}
        />
        {currentBuyInOptions.addon.enabled && (
          <>
            <CurrencyInputComponent
              label={
                <>
                  Chips{" "}
                  <Tooltips.addon_chips />
                </>
              }
              value={currentBuyInOptions.addon.addon_chips}
              currencyType={{ Fake: null }}
              onChange={(addon_chips) => patchBuyInOptions(opt => ({ ...opt, addon: { ...opt.addon, addon_chips } }))}
            />
            <CurrencyInputComponent
              label={
                <>
                  Price{" "}
                  <Tooltips.addon_price />
                </>
              }
              currencyType={currency}
              value={currentBuyInOptions.addon.addon_price}
              min={0n}
              onChange={(addon_price) => patchBuyInOptions(opt => ({ ...opt, addon: { ...opt.addon, addon_price } }))}
            />
            <NumberInputComponent
              label={
                <>
                  Max Addons{" "}
                  <Tooltips.max_addons />
                </>
              }
              min={1}
              step={1}
              value={currentBuyInOptions.addon.max_addons}
              onChange={(max_addons) => patchBuyInOptions(opt => ({ ...opt, addon: { ...opt.addon, max_addons } }))}
            />
            <DateInputComponent
              label={
                <>
                  Start{" "}
                  <Tooltips.addon_start_time />
                </>
              }
              datetime_ns={currentBuyInOptions.addon.addon_start_time}
              min_ns={data.start_time ? DateToBigIntTimestamp(addMinutes(BigIntTimestampToDate(data.start_time), 5)) : undefined}
              onChange={(addon_start_time) => patchBuyInOptions(opt => ({ ...opt, addon: { ...opt.addon, addon_start_time } }))}
            />
            <DateInputComponent
              label={
                <>
                  End{" "}
                  <Tooltips.addon_end_time />
                </>
              }
              datetime_ns={currentBuyInOptions.addon.addon_end_time}
              min_ns={data.start_time ? DateToBigIntTimestamp(addMinutes(BigIntTimestampToDate(data.start_time), 5)) : undefined}
              onChange={(addon_end_time) => patchBuyInOptions(opt => ({ ...opt, addon: { ...opt.addon, addon_end_time } }))}
            />
          </>
        )}
      </List>

      <div className='flex flex-col gap-3'>
        <List>
          <SwitchInputComponent
            label={(
              <>
                Rebuy
                <Tooltips.rebuy />
              </>
            )}
            checked={currentBuyInOptions.rebuy.enabled}
            onChange={(enabled) => patchBuyInOptions(opt => ({
              ...opt, rebuy: { ...opt.rebuy, enabled },
              freezout: enabled ? false : opt.freezout,
            }))}
          />

          {currentBuyInOptions.rebuy.enabled && (
            <>
              <NumberInputComponent
                label={
                  <>
                    Max Rebuys{" "}
                    <Tooltips.max_rebuys />
                  </>
                }
                value={currentBuyInOptions.rebuy.max_rebuys}
                onChange={(max_rebuys) => patchBuyInOptions(opt => ({ ...opt, rebuy: { ...opt.rebuy, max_rebuys } }))}
              />
              <CurrencyInputComponent
                label={
                  <>
                    Chips{" "}
                    <Tooltips.rebuy_chips />
                  </>
                }
                min={blindsAtRebuyEnd.data?.[0]?.big_blind ?? 0n}
                value={currentBuyInOptions.rebuy.rebuy_chips}
                currencyType={{ Fake: null }}
                onChange={(rebuy_chips) => patchBuyInOptions(opt => ({ ...opt, rebuy: { ...opt.rebuy, rebuy_chips } }))}
              />
              <CurrencyInputComponent
                label={
                  <>
                    Price{" "}
                    <Tooltips.rebuy_price />
                  </>
                }
                currencyType={currency}
                value={currentBuyInOptions.rebuy.rebuy_price}
                min={0n}
                onChange={(rebuy_price) => patchBuyInOptions(opt => ({ ...opt, rebuy: { ...opt.rebuy, rebuy_price } }))}
              />
              <DateInputComponent
                label={
                  <>
                    End{" "}
                    <Tooltips.rebuy_end_timestamp />
                  </>
                }
                datetime_ns={currentBuyInOptions.rebuy.rebuy_end_timestamp}
                min_ns={data.start_time}
                onChange={(rebuy_end_timestamp) => patchBuyInOptions(opt => ({ ...opt, rebuy: { ...opt.rebuy, rebuy_end_timestamp } }))}
              />
              <CurrencyInputComponent
                label={(
                  <>
                    Min Chips{" "}
                    <Tooltips.min_chips_for_rebuy />
                  </>
                )}
                value={currentBuyInOptions.rebuy.min_chips_for_rebuy}
                onChange={(min_chips_for_rebuy) => patchBuyInOptions(opt => ({ ...opt, rebuy: { ...opt.rebuy, min_chips_for_rebuy } }))}
                currencyType={{ Fake: null }}
              />
              <TimeInputComponent
                label={
                  <>
                    Rebuy Window{" "}
                    <Tooltips.rebuy_window_seconds />
                  </>
                }
                seconds={currentBuyInOptions.rebuy.rebuy_window_seconds}
                showSeconds
                hideHours
                onChangeSeconds={(rebuy_window_seconds) => patchBuyInOptions(opt => ({ ...opt, rebuy: { ...opt.rebuy, rebuy_window_seconds } }))}
              />

            </>
          )}
        </List>

        {currentBuyInOptions.rebuy.enabled && (
          <BigBlindWarning
            blinds={blindsAtRebuyEnd.data?.[0]}
            timestamp={currentBuyInOptions.reentry.reentry_end_timestamp}
          />
        )}
      </div>


      <div className='flex flex-col gap-3'>
        <List>
          <SwitchInputComponent
            label={(
              <>
                Reentry
                <Tooltips.reentry />
              </>
            )}
            checked={currentBuyInOptions.reentry.enabled}
            onChange={(enabled) => patchBuyInOptions(opt => ({
              ...opt,
              reentry: { ...opt.reentry, enabled },
              freezout: enabled ? false : opt.freezout,
            }))}
          />

          {currentBuyInOptions.reentry.enabled && (
            <>
              <NumberInputComponent
                label={
                  <>
                    Max Reentries{" "}
                    <Tooltips.max_reentries />
                  </>
                }
                value={currentBuyInOptions.reentry.max_reentries}
                onChange={(max_reentries) => patchBuyInOptions(opt => ({ ...opt, reentry: { ...opt.reentry, max_reentries } }))}
              />
              <CurrencyInputComponent
                label={
                  <>
                    Chips{" "}
                    <Tooltips.reentry_chips />
                  </>
                }
                min={blindsAtReentryEnd.data?.[0]?.big_blind ?? 0n}
                value={currentBuyInOptions.reentry.reentry_chips}
                currencyType={{ Fake: null }}
                onChange={(reentry_chips) => patchBuyInOptions(opt => ({ ...opt, reentry: { ...opt.reentry, reentry_chips } }))}
              />
              <CurrencyInputComponent
                label={
                  <>
                    Price{" "}
                    <Tooltips.reentry_price />
                  </>
                }
                currencyType={currency}
                value={currentBuyInOptions.reentry.reentry_price}
                min={0n}
                onChange={(reentry_price) => patchBuyInOptions(opt => ({ ...opt, reentry: { ...opt.reentry, reentry_price } }))}
              />
              <DateInputComponent
                label={
                  <>
                    End{" "}
                    <Tooltips.reentry_end_timestamp />
                  </>
                }
                datetime_ns={currentBuyInOptions.reentry.reentry_end_timestamp}
                min_ns={data.start_time}
                onChange={(reentry_end_timestamp) => patchBuyInOptions(opt => ({ ...opt, reentry: { ...opt.reentry, reentry_end_timestamp } }))}
              />
            </>
          )}
        </List>

        {currentBuyInOptions.reentry.enabled && (
          <BigBlindWarning
            blinds={blindsAtReentryEnd.data?.[0]}
            timestamp={currentBuyInOptions.reentry.reentry_end_timestamp}
          />
        )}
      </div>
    </>
  );
});
TypeStepComponent.displayName = "TypeStepComponent";

export const Config: SteppedModalStep<TypeStepValues> = {
  title: "Type options",
  defaultValues: { tournament_type: default_tournament_type },
  Component: TypeStepComponent,
  enabled: ({ tournament_type }) => !!tournament_type && !('SitAndGo' in tournament_type),
  isValid: ({ start_time, tournament_type }) => {
    if (!tournament_type) return ["Tournament type is required"];

    const size = matchRustEnum(tournament_type)({
      SitAndGo: (size) => size,
      SpinAndGo: ([size]) => size,
      Freeroll: (size) => size,
      BuyIn: (size) => size,
    });
    const opt = matchRustEnum(size)({
      SingleTable: (size) => size,
      MultiTable: ([size]) => size,
    });

    if (!opt) return ["BuyIn options are required"];

    if (opt.freezout && opt.rebuy.enabled) return ["Freezout and rebuy can't be enabled at the same time"];

    if (!start_time) return ["Define a tournament start time in the first step"];

    if (opt.addon.enabled) {
      if (opt.addon.addon_chips === 0n) return ["Addon chips is required"];
      if (opt.addon.addon_price === 0n) return ["Addon price is required"];
      if (opt.addon.max_addons === 0) return ["Max addons is required"];
      if (opt.addon.addon_start_time === 0n) return ["Addon start time is required"];
      const minStart = DateToBigIntTimestamp(addMinutes(BigIntTimestampToDate(start_time), 5))
      if (opt.addon.addon_start_time < minStart)
        return [`The minimum addon start time is ${fmt(minStart)} (5 minutes after tournament start time)`];
      if (opt.addon.addon_end_time === 0n) return ["Addon end time is required"];
      if (opt.addon.addon_start_time >= opt.addon.addon_end_time) return ["Addon start time must be before addon end time"];
    }

    if (opt.reentry.enabled) {
      if (opt.reentry.max_reentries === 0) return ["Max reentries is required"];
      if (opt.reentry.reentry_chips === 0n) return ["Reentry chips is required"];
      if (opt.reentry.reentry_price === 0n) return ["Reentry price is required"];
      if (opt.reentry.reentry_end_timestamp === 0n) return ["Reentry end time is required"];
      if (start_time && opt.reentry.reentry_end_timestamp <= start_time)
        return [`Reentry end time must be after tournament start time ${fmt(start_time)}`];
    }

    if (opt.rebuy.enabled) {
      if (opt.rebuy.max_rebuys === 0) return ["Max rebuys is required"];
      if (opt.rebuy.rebuy_chips === 0n) return ["Rebuy chips is required"];
      if (opt.rebuy.rebuy_price === 0n) return ["Rebuy price is required"];
      if (opt.rebuy.rebuy_end_timestamp === 0n) return ["Rebuy end time is required"];
      if (start_time && opt.rebuy.rebuy_end_timestamp <= start_time)
        return [`Rebuy end time must be after tournament start time ${fmt(start_time)}`];
      if (opt.rebuy.min_chips_for_rebuy === 0n) return ["Min chips for rebuy is required"];
      if (opt.rebuy.rebuy_window_seconds === 0n) return ["Rebuy window seconds is required"];
    }

    return true;
  },
};
