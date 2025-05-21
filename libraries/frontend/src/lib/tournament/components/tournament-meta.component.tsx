import { formatDistance } from 'date-fns';
import { format } from 'date-fns-tz';
import { memo, useMemo } from 'react';

import { AnteType, CurrencyType } from '@declarations/table_index/table_index.did';
import {
  BlindLevel, SpeedTypeParams, TournamentData, TournamentType
} from '@declarations/tournament_canister/tournament_canister.did';
import { BigIntTimestampToDate, secondsToLabel } from '@lib/utils/time';
import { CurrencyComponent, CurrencyTypeComponent } from '@zk-game-dao/currency';
import { List, ListItem } from '@zk-game-dao/ui';

import { nanosecondsToString } from '../../utils/duration';
import { matchRustEnum } from '../../utils/rust';
import { useTournament } from '../context/tournament.context';
import { TournamentStatusComponent } from './tournament-status.component';

const BoolLabel = memo(({ value }: { value: boolean }) => (
  <span className={`type-button-3 ${value ? 'text-green-500' : 'text-red-500'}`}>
    {value ? 'Yes' : 'No'}
  </span>
));
BoolLabel.displayName = 'BoolLabel';

const dateTimeFormat = `dd.MM.yyyy HH:mm`;
const fmt = (dtn: bigint) => format(BigIntTimestampToDate(dtn), dateTimeFormat);

const TournamentTypeModalContent = memo<TournamentType & { currencyType: CurrencyType }>(({ currencyType, ...props }) => {
  const size = matchRustEnum(props)({
    SitAndGo: (size) => size,
    SpinAndGo: ([size]) => size,
    Freeroll: (size) => size,
    BuyIn: (size) => size,
  });

  const options = matchRustEnum(size)({
    SingleTable: (size) => size,
    MultiTable: ([size]) => size,
  });

  return (
    <>
      <List>
        <ListItem rightLabel={<BoolLabel value={options.freezout} />}>Freeze out</ListItem>
        <ListItem rightLabel={<BoolLabel value={!!options.addon && options.addon.enabled} />}>Addon</ListItem>
        <ListItem rightLabel={<BoolLabel value={!!options.rebuy && options.rebuy.enabled} />}>Rebuy</ListItem>
      </List>

      {options.addon && options.addon.enabled && (
        <List label="Addon">
          <ListItem rightLabel={<CurrencyComponent currencyValue={options.addon.addon_chips} currencyType={{ Fake: null }} />}>
            Chips
          </ListItem>
          <ListItem rightLabel={<CurrencyComponent currencyValue={options.addon.addon_price} currencyType={currencyType} />}>
            Price
          </ListItem>
          <ListItem rightLabel={fmt(options.addon.addon_start_time)}>
            Start time
          </ListItem>
          <ListItem rightLabel={fmt(options.addon.addon_end_time)}>
            End time
          </ListItem>
        </List>
      )}

      {options.rebuy && options.rebuy.enabled && (
        <List label="Rebuy">
          <ListItem rightLabel={<CurrencyComponent currencyValue={options.rebuy.rebuy_chips} currencyType={{ Fake: null }} />}>
            Chips
          </ListItem>
          <ListItem rightLabel={<CurrencyComponent currencyValue={options.rebuy.rebuy_price} currencyType={currencyType} />}>
            Price
          </ListItem>
          <ListItem rightLabel={options.rebuy.max_rebuys.toString()}>
            Max rebuys
          </ListItem>
          <ListItem rightLabel={fmt(options.rebuy.rebuy_end_timestamp)}>
            End time
          </ListItem>
          <ListItem rightLabel={options.rebuy.min_chips_for_rebuy.toString()}>
            Min chips for rebuy
          </ListItem>
          <ListItem rightLabel={secondsToLabel(Number(options.rebuy.rebuy_window_seconds))}>
            Rebuy window seconds
          </ListItem>
        </List>
      )}

      {options.reentry && options.reentry.enabled && (
        <List label="reentry">
          <ListItem rightLabel={<CurrencyComponent currencyValue={options.reentry.reentry_chips} currencyType={{ Fake: null }} />}>
            Chips
          </ListItem>
          <ListItem rightLabel={<CurrencyComponent currencyValue={options.reentry.reentry_price} currencyType={currencyType} />}>
            Price
          </ListItem>
          <ListItem rightLabel={options.reentry.max_reentries.toString()}>
            Max reentries
          </ListItem>
          <ListItem rightLabel={fmt(options.reentry.reentry_end_timestamp)}>
            End time
          </ListItem>
        </List>
      )}
    </>
  );
});
TournamentTypeModalContent.displayName = 'TournamentTypeModalContent';

const AnteTypeComponent = memo<{ ante_type: AnteType }>(({ ante_type }) => {
  if ('PercentageOfBigBlind' in ante_type)
    return <>{ante_type.PercentageOfBigBlind.toString()}% of big blind</>;
  if ('None' in ante_type)
    return <span>None</span>;
  if ('BigBlindAnte' in ante_type)
    return <span>Big blind</span>;
  if ('Fixed' in ante_type)
    return <span>{ante_type.Fixed.toString()}</span>;
});
AnteTypeComponent.displayName = 'AnteTypeComponent';

const BlindLevelComponent = memo<BlindLevel & { level?: number; hideDuration?: boolean; }>(({ small_blind, big_blind, ante_type, duration_ns, hideDuration = false, level }) => (
  <List label={`Current level`}>
    <ListItem rightLabel={level?.toString()}>Level</ListItem>
    <ListItem rightLabel={<CurrencyComponent currencyValue={small_blind} currencyType={{ Fake: null }} />}>Small blind</ListItem>
    <ListItem rightLabel={<CurrencyComponent currencyValue={big_blind} currencyType={{ Fake: null }} />}>Big blind</ListItem>
    <ListItem rightLabel={<AnteTypeComponent ante_type={ante_type} />}>Ante</ListItem>
    {!hideDuration && <ListItem rightLabel={nanosecondsToString(duration_ns)}>Duration</ListItem>}
  </List>
));
BlindLevelComponent.displayName = 'BlindLevelComponent';

const BlindsStructureModalContent = memo<Pick<TournamentData, 'speed_type'>>(({ speed_type }) => {
  let t: SpeedTypeParams | undefined = undefined;
  const speedType = Object.keys(speed_type).map((key) => key);

  if ('Regular' in speed_type) t = speed_type.Regular;
  if ('Turbo' in speed_type) t = speed_type.Turbo;
  if ('HyperTurbo' in speed_type) t = speed_type.HyperTurbo;
  if ('Custom' in speed_type) t = speed_type.Custom;

  if (!t) return <span>Unknown</span>;

  return (
    <>
      {t.blind_levels[t.current_level] && (
        <BlindLevelComponent {...t.blind_levels[t.current_level]} level={t.current_level + 1} />
      )}

      <List label="Blinds structure">
        <ListItem rightLabel={speedType}>Type</ListItem>
        <ListItem rightLabel={nanosecondsToString(t.level_duration_ns)}>Level duration</ListItem>
        <ListItem rightLabel={t.ante_start_level.toString()}>Ante start level</ListItem>
        <ListItem rightLabel={`${t.ante_percentage}%`}>Ante percentage</ListItem>
        {t.next_level_time.length === 1 && (
          <ListItem rightLabel={'in ' + formatDistance(new Date(), BigIntTimestampToDate(t.next_level_time[0]))}>Next level time</ListItem>
        )}
        <ListItem rightLabel={`${t.blind_multiplier}x`}>Blind multiplier</ListItem>
        <ListItem rightLabel={`${t.initial_blind_percentage}%`}>Initial blind percentage</ListItem>
      </List>
    </>
  )
});
BlindsStructureModalContent.displayName = 'BlindsStructureModalContent';

const PayoutStructureModalContent = memo<Pick<TournamentData, 'payout_structure'>>(({ payout_structure }) => {
  return (
    <List key={JSON.stringify(payout_structure)}>
      {payout_structure.map(({ percentage, position }) => (
        <ListItem key={position} rightLabel={`${percentage}%`}>{position + 1}. Place</ListItem>
      ))}
    </List>
  );
});
PayoutStructureModalContent.displayName = 'PayoutStructureModalContent';

const TournamentInfoContent = memo(() => {
  const { data, currencyType } = useTournament(true);
  return (
    <List>
      <ListItem
        modal={{
          title: 'Tournament type',
          children: <TournamentTypeModalContent {...data.tournament_type} currencyType={currencyType} />
        }}
        rightLabel={matchRustEnum(data.tournament_type)({
          BuyIn: () => 'Buy-in',
          SitAndGo: () => 'Sit and Go',
          Freeroll: () => 'Freeroll',
          SpinAndGo: () => 'Spin and Go',
        })}
      >
        Tournament type
      </ListItem>
      <ListItem
        modal={{
          title: 'Blinds structure',
          children: <BlindsStructureModalContent {...data} />
        }}
      >
        Blinds structure
      </ListItem>
      <ListItem
        modal={{
          title: 'Payout',
          children: <PayoutStructureModalContent {...data} />
        }}
      >
        Payout
      </ListItem>
    </List>
  )
});
TournamentInfoContent.displayName = 'TournamentInfoContent';

export const TournamentMetaComponent = memo(() => {
  const { data } = useTournament(true);

  const currentLevel = useMemo((): (SpeedTypeParams & { isLast: boolean }) | undefined => {
    const level = Object.values(data.speed_type)[0];
    if (!level) return;
    return {
      ...level,
      isLast: Number(level.current_level) === level.blind_levels.length - 1,
    };
  }, [data]);

  return (
    <div className='flex flex-col'>
      <List className='min-w-[300px]'>
        <ListItem rightLabel={fmt(data.start_time)}>
          Start
        </ListItem>
        <ListItem rightLabel={<CurrencyTypeComponent currencyType={data.currency} />}>
          Currency
        </ListItem>
        <ListItem rightLabel={<TournamentStatusComponent state={data.state} start_time={data.start_time} />}>
          State
        </ListItem>
        {currentLevel && (
          <ListItem
            rightLabel={(currentLevel.current_level + 1).toString()}
            modal={{
              children: <BlindsStructureModalContent {...data} />,
            }}
          >
            Current blind level
          </ListItem>
        )}
        <ListItem
          rightLabel={`${data.current_players.length}/${data.max_players}`}
        >
          Current players
        </ListItem>
        <ListItem
          modal={{
            title: 'Information',
            children: <TournamentInfoContent />
          }}
        >
          Information
        </ListItem>
      </List>
    </div>
  );
});
TournamentMetaComponent.displayName = 'TournamentMetaComponent';