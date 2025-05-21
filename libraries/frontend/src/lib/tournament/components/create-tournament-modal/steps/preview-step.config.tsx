import { TableConfig } from '@declarations/table_index/table_index.did';
import { NewTournament, TournamentSizeType } from '@declarations/tournament_index/tournament_index.did';
import { CurrencyComponent, CurrencyTypeSymbolComponent } from '@zk-game-dao/currency';
import { List, ListItem, StepComponentProps, SteppedModalStep } from '@zk-game-dao/ui';
import { format } from 'date-fns-tz';
import { memo, useMemo } from 'react';

import { nanosecondsToString } from '../../../../utils/duration';
import { matchRustEnum } from '../../../../utils/rust';
import { BigIntTimestampToDate } from '../../../../utils/time';
import { defaultBuyInOptions } from './type-step.config';

const dateTimeFormat = `dd.MM.yyyy HH:mm`;
const fmt = (dtn: bigint) => format(BigIntTimestampToDate(dtn), dateTimeFormat);

type TypeStepValues = NewTournament & Pick<TableConfig, 'seats'>;

const PreviewStepComponent = memo<StepComponentProps<TypeStepValues>>(({ data }) => {

  const size = useMemo((): TournamentSizeType => {
    if (!data.tournament_type) return { SingleTable: defaultBuyInOptions };
    return matchRustEnum(data.tournament_type)({
      SpinAndGo: ([t]) => t,
      BuyIn: t => t,
      SitAndGo: t => t,
      Freeroll: t => t
    });
  }, [data.tournament_type]);

  const buyInOptions = useMemo(() => matchRustEnum(size)({
    SingleTable: (t) => t,
    MultiTable: ([t]) => t,
  }), [size]);

  // const currency = useMemo(() => CurrencyTypeToCurrency(data.currency ?? { Real: { ICP: null } }) as Currency, [data.currency]);

  return (
    <>

      <List label="General">
        <ListItem rightLabel={data.name}>Name</ListItem>
        <ListItem rightLabel={data.description}>Description</ListItem>
        <ListItem rightLabel={<img src={data.hero_picture} width={24} height={24} className='rounded-[4px]' alt="hero" />}>Image</ListItem>
        {data.require_proof_of_humanity && <ListItem rightLabel="Required">Proof of humanity</ListItem>}
        <ListItem rightLabel={<CurrencyTypeSymbolComponent currencyType={data.currency ?? { Real: { ICP: null } }} />}>Currency</ListItem>
        {data.tournament_type && (
          <>
            <ListItem rightLabel={matchRustEnum(data.tournament_type)({
              BuyIn: () => 'Buy-In',
              SitAndGo: () => 'Sit & Go',
              Freeroll: () => 'Freeroll',
              SpinAndGo: () => 'Spin & Go',
            })}>
              Type
            </ListItem>
            {!('SitAndGo' in data.tournament_type) && (
              <>
                <ListItem rightLabel={fmt(data.start_time ?? 0n)}>Start time</ListItem>
                <ListItem rightLabel={nanosecondsToString(data.late_registration_duration_ns ?? 0n)}>Late registration duration</ListItem>
              </>
            )}
          </>
        )}
        <ListItem rightLabel={<CurrencyComponent currencyValue={data.starting_chips} currencyType={{ Fake: null }} />}>Starting chips</ListItem>
        {data.tournament_type && !('Freeroll' in data.tournament_type) && <ListItem rightLabel={<CurrencyComponent currencyValue={data.buy_in} currencyType={data.currency ?? { Real: { ICP: null } }} />}>Buy in</ListItem>}
        {buyInOptions?.freezout && <ListItem>Freezout</ListItem>}
      </List>

      {data.tournament_type && !('SpinAndGo' in data.tournament_type) && ('MultiTable' in size) && (
        <List label="Multi table">
          <ListItem rightLabel={data.seats}>Max players per table</ListItem>
          <ListItem rightLabel={nanosecondsToString(size.MultiTable[1].balance_interval_ns)}>Balance interval</ListItem>
        </List>
      )}

      {data.tournament_type && !('SpinAndGo' in data.tournament_type) && (
        <List label="Players">
          <ListItem rightLabel={data.min_players}>Min players</ListItem>
          <ListItem rightLabel={data.max_players}>Max players</ListItem>

        </List>
      )}

      {buyInOptions?.addon.enabled && (
        <List label="Addon">
          <ListItem rightLabel={<CurrencyComponent currencyValue={buyInOptions.addon.addon_chips} currencyType={{ Fake: null }} />}>Chips</ListItem>
          <ListItem rightLabel={<CurrencyComponent currencyValue={buyInOptions.addon.addon_price} currencyType={data.currency ?? { Real: { ICP: null } }} />}>Cost</ListItem>
          <ListItem rightLabel={buyInOptions.addon.max_addons}>Max addons</ListItem>
          <ListItem rightLabel={fmt(buyInOptions.addon.addon_start_time)}>Start time</ListItem>
          <ListItem rightLabel={fmt(buyInOptions.addon.addon_end_time)}>End time</ListItem>
        </List>
      )}

      {buyInOptions?.rebuy.enabled && (
        <List label="Rebuy">
          <ListItem rightLabel={<CurrencyComponent currencyValue={buyInOptions.rebuy.rebuy_chips} currencyType={{ Fake: null }} />}>Chips</ListItem>
          <ListItem rightLabel={<CurrencyComponent currencyValue={buyInOptions.rebuy.rebuy_price} currencyType={data.currency ?? { Real: { ICP: null } }} />}>Cost</ListItem>
          <ListItem rightLabel={buyInOptions.rebuy.max_rebuys}>Max rebuys</ListItem>
          <ListItem rightLabel={fmt(buyInOptions.rebuy.rebuy_end_timestamp)}>End time</ListItem>
        </List>
      )}

      {buyInOptions?.reentry.enabled && (
        <List label="Reentry">
          <ListItem rightLabel={<CurrencyComponent currencyValue={buyInOptions.reentry.reentry_chips} currencyType={{ Fake: null }} />}>Chips</ListItem>
          <ListItem rightLabel={<CurrencyComponent currencyValue={buyInOptions.reentry.reentry_price} currencyType={data.currency ?? { Real: { ICP: null } }} />}>Cost</ListItem>
          <ListItem rightLabel={buyInOptions.reentry.max_reentries}>Max rebuys</ListItem>
          <ListItem rightLabel={fmt(buyInOptions.reentry.reentry_end_timestamp)}>End time</ListItem>
        </List>
      )}

      {data.speed_type && matchRustEnum(data.speed_type)({
        Custom: ({
          level_duration_ns,
          ante_start_level,
          ante_percentage,
          blind_multiplier,
          max_levels,
          initial_blind_percentage
        }) => (
          <List label="Speed">
            <ListItem rightLabel="Custom">Type</ListItem>
            <ListItem rightLabel={nanosecondsToString(level_duration_ns)}>Level duration</ListItem>
            <ListItem rightLabel={ante_start_level}>Ante start level</ListItem>
            <ListItem rightLabel={`${ante_percentage}%`}>Ante percentage</ListItem>
            <ListItem rightLabel={`${blind_multiplier}x`}>Blind multiplier</ListItem>
            <ListItem rightLabel={max_levels}>Max levels</ListItem>
            <ListItem rightLabel={`${initial_blind_percentage}%`}>Initial blind percentage</ListItem>
          </List>
        ),
        Turbo: (levels) => (
          <List label="Speed">
            <ListItem rightLabel="Turbo">Type</ListItem>
            <ListItem rightLabel={levels}>Levels</ListItem>
          </List>
        ),
        HyperTurbo: (levels) => (
          <List label="Speed">
            <ListItem rightLabel="Hyper Turbo">Type</ListItem>
            <ListItem rightLabel={levels}>Levels</ListItem>
          </List>
        ),
        Regular: (levels) => (
          <List label="Speed">
            <ListItem rightLabel="Regular">Type</ListItem>
            <ListItem rightLabel={levels}>Levels</ListItem>
          </List>
        ),
      })}

      {data.payout_structure && (
        <List label="Payout structure">
          {data.payout_structure.map(({ percentage, position }) => (
            <ListItem key={position} rightLabel={`${percentage}%`}>#{position + 1}</ListItem>
          ))}
        </List>
      )}

    </>
  );
});
PreviewStepComponent.displayName = 'PreviewStepComponent';

export const Config: SteppedModalStep<TypeStepValues> = {
  title: "Preview",
  defaultValues: {},
  Component: PreviewStepComponent,
  isValid: () => true,
};
