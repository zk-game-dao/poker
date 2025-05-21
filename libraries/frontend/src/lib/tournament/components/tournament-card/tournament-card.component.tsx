import { createActor } from '@declarations/tournament_canister';
import { TournamentData } from '@declarations/tournament_index/tournament_index.did';
import { CurrencyComponent } from '@zk-game-dao/currency';
import { List, ListItem, PillComponent, useQuery } from '@zk-game-dao/ui';
import { format, formatDistance, formatDistanceToNow } from 'date-fns';
import { memo, useEffect, useMemo, useState } from 'react';

import { Queries } from '../../../data';
import { useUser } from '../../../user';
import { callActorMutation } from '../../../utils/call-actor-mutation';
import { BigIntTimestampToDate, DateToBigIntTimestamp, DateToLocalDateTimeString } from '../../../utils/time';
import { QuickJoinModal } from '../quick-join-modal.component';
import { TournamentStatusComponent } from '../tournament-status.component';

export type Props = Pick<
  TournamentData,
  'id' |
  'name' |
  'state' |
  'tables' |
  'buy_in' |
  'currency' |
  'start_time' |
  'description' |
  'max_players' |
  'hero_picture' |
  'table_config' |
  'starting_chips' |
  'current_players' |
  'tournament_type' |
  'late_registration_duration_ns'
>;

const dateTimeFormat = `dd.MM.yyyy HH:mm`;
const fmt = (dtn: bigint) => format(BigIntTimestampToDate(dtn), dateTimeFormat);

export const TournamentCardComponent = memo<Props>(({
  id,
  ...props
}) => {
  const { data } = useQuery({
    queryKey: Queries.tournament.key(id),
    queryFn: (): Promise<Props> => callActorMutation(createActor(id), 'get_tournament'),
    initialData: {
      ...props,
      id,
    } as TournamentData | undefined
  });

  const prizePool = useQuery({
    queryKey: Queries.tournamentPrizePool.key(id),
    queryFn: () => createActor(id).get_total_prize_pool(),
    refetchInterval: 10000
  });

  const startTimeString = useMemo(() => data && DateToLocalDateTimeString(data.start_time), [data?.start_time]);
  const { user } = useUser();
  const [currentTime, setCurrentTime] = useState(DateToBigIntTimestamp(new Date()));

  useEffect(() => {
    const interval = setInterval(() => setCurrentTime(DateToBigIntTimestamp(new Date())), 10000);
    return () => clearInterval(interval);
  }, []);

  const canJoin = useMemo(() => {
    if (!user || !data) return false;
    if (data.current_players.find((player) => player[0].compareTo(user.principal_id) === 'eq')) return false;
    if (data.start_time < currentTime) return false;
    if (data.current_players.length >= data.max_players) return false;
    return true;
  }, [data?.current_players, data?.max_players, data?.start_time, currentTime, user]);

  const [quickJoinModalOpen, setQuickJoinModalOpen] = useState(false);

  const timeDifferenceString = useMemo(() => data && formatDistance(BigIntTimestampToDate(data.start_time), BigIntTimestampToDate(currentTime)), [data?.start_time, currentTime]);

  const didStart = useMemo(() => data && data.start_time < currentTime, [data?.start_time, currentTime]);
  const lateRegistrationActive = useMemo(() => didStart && data && (currentTime - data?.start_time) < data?.late_registration_duration_ns, [didStart, currentTime, data?.start_time, data?.late_registration_duration_ns]);

  if (!data) return null;

  return (
    <div className='relative rounded-[16px] material overflow-hidden flex flex-col p-6 gap-4'>
      <QuickJoinModal
        open={quickJoinModalOpen}
        onClose={() => setQuickJoinModalOpen(false)}
        tables={data.tables}
        id={id}
        buy_in={data.buy_in}
        currencyType={data.currency}
        tournament_type={data.tournament_type}
      />
      <div className='rounded-[8px] overflow-hidden h-[275px]'>
        <img
          alt="Tournament"
          src={data.hero_picture}
          width={275}
          height={275}
          className="w-full object-cover size-[275px]"
        />
      </div>

      <div className="flex flex-col gap-1 z-1 mb-1">
        <div className="flex flex-row type-callout text-material-medium-3">
          {data.description}
          <div className="flex flex-1" />
          {`${data.current_players.length} / ${data.max_players} players`}
        </div>
        <p className="type-header">{data.name}</p>
        <div className="flex flex-row type-button-3 text-material-medium-3">
          {didStart ?
            `Started at ${fmt(data.start_time)}` : `Starts in ${timeDifferenceString}`}
        </div>
      </div>

      <List>
        <ListItem rightLabel={startTimeString}>
          Starting time
        </ListItem>
        <ListItem rightLabel={<CurrencyComponent currencyValue={data.buy_in} currencyType={data.currency} />}>
          Buy in
        </ListItem>
        <ListItem rightLabel={<TournamentStatusComponent state={data.state} start_time={data.start_time} />}>
          State
        </ListItem>
        {prizePool.data !== undefined && (
          <ListItem rightLabel={<CurrencyComponent currencyValue={prizePool.data} currencyType={data.currency} />}>
            Prize Pool
          </ListItem>
        )}
        <ListItem rightLabel={<CurrencyComponent currencyValue={data.starting_chips} currencyType={{ Fake: null }} />}>
          Starting chips
        </ListItem>
        {lateRegistrationActive && (
          <ListItem rightLabel={formatDistanceToNow(BigIntTimestampToDate(data.start_time + data.late_registration_duration_ns))}>
            Late registration active
          </ListItem>
        )}
      </List>

      <div className='flex flex-row gap-4 justify-center'>
        {canJoin && (
          <PillComponent onClick={() => setQuickJoinModalOpen(true)}>
            Quick join
          </PillComponent>
        )}
        <PillComponent href={`/tournaments/${id}`}>
          View
        </PillComponent>
      </div>
    </div>
  )
});
TournamentCardComponent.displayName = 'TournamentCardComponent';
