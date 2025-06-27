import {
  AddonOptions,
  AnteType,
  BlindLevel,
  RebuyOptions,
  ReentryOptions,
  SpeedTypeParams,
  TournamentData,
} from '@declarations/tournament_canister/tournament_canister.did';
import { CurrencyComponent, CurrencyType, CurrencyTypeComponent } from '@zk-game-dao/currency';
import { List, ListItem, Modal } from '@zk-game-dao/ui';
import { format, formatDistance } from 'date-fns';
import { memo, ReactNode, useState } from 'react';

import { nanosecondsToString, secondsToString } from '../../../utils/duration';
import { matchRustEnum } from '../../../utils/rust';
import { BigIntTimestampToDate } from '../../../utils/time';
import { PricePool } from '../../components/prize-pool.component';
import { useTournament } from '../../context/tournament.context';
import { Tooltips } from '../../tooltips';
import { TournamentPageHeader } from './home-page-header.component';
import { TournamentStatusComponent } from '../../components/tournament-status.component';
import { useFormatDateDistance } from '../../../../hooks/countdown';

const dateTimeFormat = `dd.MM.yyyy HH:mm`;
const fmt = (dtn: bigint) => format(BigIntTimestampToDate(dtn), dateTimeFormat);

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

const ItemContainer = memo<{ children: ReactNode; }>(({ children }) => <div className="flex flex-col w-full md:w-[300px]">{children}</div>)
ItemContainer.displayName = 'ItemContainer';

const BlindLevelComponent = memo<BlindLevel & { level?: number; hideDuration?: boolean; }>(({ small_blind, big_blind, ante_type, duration_ns, hideDuration = false, level }) => (
  <List label={`Current level`}>
    <ListItem rightLabel={level?.toString()}>Level</ListItem>
    <ListItem rightLabel={<CurrencyComponent currencyValue={small_blind} currencyType={{ Fake: null }} />}>Small blind</ListItem>
    <ListItem rightLabel={<CurrencyComponent currencyValue={big_blind} currencyType={{ Fake: null }} />}>Big blind</ListItem>
    <ListItem rightLabel={<AnteTypeComponent ante_type={ante_type} />}>Ante </ListItem>
    {!hideDuration && <ListItem rightLabel={nanosecondsToString(duration_ns)}>Duration</ListItem>}
  </List>
));
BlindLevelComponent.displayName = 'BlindLevelComponent';

const Blinds = memo<Pick<TournamentData, 'speed_type'>>(({ speed_type }) => {
  let t: SpeedTypeParams | undefined = undefined;
  const speedType = Object.keys(speed_type).map((key) => key);
  const [showingLevels, setShowingLevels] = useState(false);
  const [showingStructure, setShowingStructure] = useState(false);

  if ('Regular' in speed_type) t = speed_type.Regular;
  if ('Turbo' in speed_type) t = speed_type.Turbo;
  if ('HyperTurbo' in speed_type) t = speed_type.HyperTurbo;
  if ('Custom' in speed_type) t = speed_type.Custom;

  if (!t) return;

  return (
    <ItemContainer>

      <Modal
        title='Levels'
        open={showingLevels}
        onClose={() => setShowingLevels(false)}
      >
        {t.blind_levels[t.current_level] && (
          <BlindLevelComponent {...t.blind_levels[t.current_level]} level={t.current_level + 1} />
        )}
      </Modal>

      <Modal
        title='Structure'
        open={showingStructure}
        onClose={() => setShowingStructure(false)}
      >
        <List>
          <ListItem rightLabel={speedType}>Speed type<span className='w-1' /><Tooltips.speed_type /></ListItem>
          <ListItem rightLabel={nanosecondsToString(t.level_duration_ns)}>Level duration<span className='w-1' /><Tooltips.level_duration /></ListItem>
          <ListItem rightLabel={t.ante_start_level.toString()}>Ante start level<span className='w-1' /><Tooltips.ante_start_level /></ListItem>
          <ListItem rightLabel={`${t.ante_percentage}%`}>Ante percentage<span className='w-1' /><Tooltips.ante_percentage /></ListItem>
          {t.next_level_time.length === 1 && (
            <ListItem rightLabel={'in ' + formatDistance(new Date(), BigIntTimestampToDate(t.next_level_time[0]))}>Next level time</ListItem>
          )}
          <ListItem rightLabel={`${t.blind_multiplier}x`}>Blind multiplier<span className='w-1' /><Tooltips.blind_multiplier /></ListItem>
          <ListItem rightLabel={`${t.initial_blind_percentage}%`}>Initial blind percentage<span className='w-1' /><Tooltips.initial_blind_percentage /></ListItem>
        </List>
      </Modal>

      <List label="Blinds structure" >
        <ListItem rightLabel={speedType} onClick={() => setShowingStructure(true)}>Speed type</ListItem>
        <ListItem onClick={() => setShowingLevels(true)}>
          Levels
        </ListItem>
      </List>
    </ItemContainer>
  )
});
Blinds.displayName = 'BlindsComponent';

const Rebuy = memo<RebuyOptions & { currencyType: CurrencyType }>(({
  max_rebuys,
  rebuy_chips,
  rebuy_end_timestamp,
  currencyType: currency,
  rebuy_price,
  rebuy_window_seconds,
  enabled,
  min_chips_for_rebuy
}) => {

  if (!enabled) return null;

  return (
    <ItemContainer>
      <List label="Rebuy">
        <ListItem rightLabel={<CurrencyComponent currencyValue={rebuy_chips} currencyType={{ Fake: null }} />}>
          Chips<span className='w-1' /><Tooltips.rebuy_chips />
        </ListItem>
        <ListItem rightLabel={<CurrencyComponent currencyValue={rebuy_price} currencyType={currency} />}>
          Price<span className='w-1' /><Tooltips.rebuy_price />
        </ListItem>
        <ListItem rightLabel={max_rebuys.toString()}>
          Max rebuys<span className='w-1' /><Tooltips.max_rebuys />
        </ListItem>
        <ListItem rightLabel={fmt(rebuy_end_timestamp)}>
          End time<span className='w-1' /><Tooltips.rebuy_end_timestamp />
        </ListItem>
        <ListItem rightLabel={secondsToString(rebuy_window_seconds)}>
          Window<span className='w-1' /><Tooltips.rebuy_window_seconds />
        </ListItem>
        <ListItem rightLabel={<CurrencyComponent currencyValue={min_chips_for_rebuy} currencyType={{ Fake: null }} />}>
          Min chips for rebuy<span className='w-1' /><Tooltips.min_chips_for_rebuy />
        </ListItem>
      </List>
    </ItemContainer>
  )
});
Rebuy.displayName = 'RebuyComponent';

const Reentry = memo<ReentryOptions & { currencyType: CurrencyType }>(({
  currencyType: currency,
  max_reentries,
  reentry_chips,
  reentry_end_timestamp,
  reentry_price,
  enabled
}) => {

  if (!enabled) return null;

  return (
    <ItemContainer>
      <List label="Reentry">
        <ListItem rightLabel={<CurrencyComponent currencyValue={reentry_chips} currencyType={{ Fake: null }} />}>
          Chips<span className='w-1' /><Tooltips.reentry_chips />
        </ListItem>
        <ListItem rightLabel={<CurrencyComponent currencyValue={reentry_price} currencyType={currency} />}>
          Price<span className='w-1' /><Tooltips.reentry_price />
        </ListItem>
        <ListItem rightLabel={max_reentries.toString()}>
          Max reentries<span className='w-1' /><Tooltips.max_reentries />
        </ListItem>
        <ListItem rightLabel={fmt(reentry_end_timestamp)}>
          End time<span className='w-1' /><Tooltips.reentry_end_timestamp />
        </ListItem>
      </List>
    </ItemContainer>
  )
});
Reentry.displayName = 'ReentryComponent';

const Addon = memo<AddonOptions & { currencyType: CurrencyType }>(({
  addon_chips,
  addon_start_time,
  max_addons,
  addon_price,
  addon_end_time,
  enabled,
  currencyType: currency,
}) => {
  if (!enabled) return null;

  return (
    <ItemContainer>
      <List label="Addon">
        <ListItem rightLabel={max_addons.toString()}>
          Max addons<span className='w-1' /><Tooltips.max_addons />
        </ListItem>
        <ListItem rightLabel={<CurrencyComponent currencyValue={addon_chips} currencyType={{ Fake: null }} />}>
          Chips<span className='w-1' /><Tooltips.addon_chips />
        </ListItem>
        <ListItem rightLabel={<CurrencyComponent currencyValue={addon_price} currencyType={currency} />}>
          Price<span className='w-1' /><Tooltips.addon_price />
        </ListItem>
        <ListItem rightLabel={fmt(addon_start_time)}>
          Start time<span className='w-1' /><Tooltips.addon_start_time />
        </ListItem>
        <ListItem rightLabel={fmt(addon_end_time)}>
          End time<span className='w-1' /><Tooltips.addon_end_time />
        </ListItem>
      </List>
    </ItemContainer>
  )
});
Addon.displayName = 'AddonComponent';

const General = memo(() => {
  const { data } = useTournament(true);

  const size = matchRustEnum(data.tournament_type)({
    SitAndGo: (size) => size,
    SpinAndGo: ([size]) => size,
    Freeroll: (size) => size,
    BuyIn: (size) => size,
  });
  const type = matchRustEnum(size)({
    SingleTable: (size) => size,
    MultiTable: ([size]) => size,
  });

  return (
    <ItemContainer>
      <List label="General">
        <ListItem rightLabel={<TournamentStatusComponent state={data.state} start_time={data.start_time} />}>
          State
        </ListItem>
        <ListItem rightLabel={<CurrencyTypeComponent currencyType={data.currency} />}>
          Currency
        </ListItem>
        <ListItem rightLabel={matchRustEnum(data.tournament_type)({
          BuyIn: () => 'Buy In',
          Freeroll: () => 'Freeroll',
          SitAndGo: () => 'Sit and Go',
          SpinAndGo: () => 'Spin and Go',
        })}>
          Type<span className='w-1' /><Tooltips.tournament_type />
        </ListItem>
        {!('Freeroll' in data.tournament_type) && !!type.freezout && (
          <ListItem rightLabel="Yes">
            Freezout<span className='w-1' /><Tooltips.freezout />
          </ListItem>
        )}
        {('SpinAndGo' in data.tournament_type) && !!(data.tournament_type.SpinAndGo[1].multiplier) && (
          <ListItem rightLabel={data.tournament_type.SpinAndGo[1].multiplier.toString()}>
            Spin multiplier
          </ListItem>
        )}
      </List>
    </ItemContainer>
  )
});
General.displayName = 'GeneralComponent';

const Participation = memo(() => {
  const { data } = useTournament(true);
  const timeUntilStart = useFormatDateDistance(BigIntTimestampToDate(data.start_time));

  // Placeholder for Participation component logic
  return (
    <ItemContainer>
      <List label="Participation">
        {data.require_proof_of_humanity && <ListItem rightLabel="Required">Proof of humanity</ListItem>}
        {!('SitAndGo' in data.tournament_type) && 'Registration' in data.state && timeUntilStart && (
          <ListItem rightLabel={timeUntilStart.number < 0 ? 'now' : `in ${timeUntilStart.string}`}>
            Start time
          </ListItem>
        )}
        {data.late_registration_duration_ns > 0n && (
          <ListItem rightLabel={nanosecondsToString(data.late_registration_duration_ns)}>
            Late registration
          </ListItem>
        )}
      </List>
    </ItemContainer>
  );
});
Participation.displayName = 'ParticipationComponent';

const PayoutStructure = memo<Pick<TournamentData, 'payout_structure'>>(({ payout_structure }) => {
  return (
    <ItemContainer>
      <List label="Payout structure">
        {payout_structure.map(({ percentage, position }) => (
          <ListItem key={position} rightLabel={`${percentage}%`}>{position + 1}. Place</ListItem>
        ))}
      </List>
    </ItemContainer>
  );
});
PayoutStructure.displayName = 'PayoutStructureComponent';

const Players = memo(() => {
  const { data } = useTournament(true);

  const { tournament_type } = data;
  const size = matchRustEnum(tournament_type)({
    SitAndGo: (size) => size,
    SpinAndGo: ([size]) => size,
    Freeroll: (size) => size,
    BuyIn: (size) => size,
  });

  return (
    <ItemContainer>
      <List label="Players">
        <ListItem rightLabel={data.current_players.length}>
          Current players
        </ListItem>
        <ListItem rightLabel={data.max_players}>
          Max players
        </ListItem>
        <ListItem rightLabel={data.min_players}>
          Min players
        </ListItem>
        {'MultiTable' in size && (
          <>
            <ListItem rightLabel={size.MultiTable[1].max_players_per_table}>
              Max players per table
            </ListItem>
          </>
        )}
      </List>
    </ItemContainer>
  )
});
Players.displayName = 'PlayersComponent';

export const TournamentInfoPage = memo(() => {
  const { data, currencyType } = useTournament(true);

  const size = matchRustEnum(data.tournament_type)({
    SitAndGo: (size) => size,
    SpinAndGo: ([size]) => size,
    Freeroll: (size) => size,
    BuyIn: (size) => size,
  });
  const type = matchRustEnum(size)({
    SingleTable: (size) => size,
    MultiTable: ([size]) => size,
  });

  return (
    <div className="flex flex-col gap-8">
      <TournamentPageHeader />
      <PricePool hideOnDesktop />
      <div className="flex flex-col md:flex-row flex-wrap gap-4 w-full">
        <General />
        <Participation />
        <Blinds speed_type={data.speed_type} />
        {type.addon.enabled && <Addon {...type.addon} currencyType={currencyType} />}
        {type.rebuy.enabled && <Rebuy {...type.rebuy} currencyType={currencyType} />}
        {type.reentry.enabled && <Reentry {...type.reentry} currencyType={currencyType} />}
        <Players />
        <PayoutStructure payout_structure={data.payout_structure} />
      </div>
    </div>
  );
});
TournamentInfoPage.displayName = 'TournamentInfoPage';

export default TournamentInfoPage;