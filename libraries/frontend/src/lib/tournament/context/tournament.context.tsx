import {
  createContext, memo, PropsWithChildren, useContext, useEffect, useMemo, useState
} from 'react';

import { createActor as createTableActor } from '@declarations/table_canister';
import { CurrencyType, PublicTable } from '@declarations/table_canister/table_canister.did';
import { createActor, tournament_canister } from '@declarations/tournament_canister';
import {
  BuyInOptions, TournamentData, UserTournamentData
} from '@declarations/tournament_canister/tournament_canister.did';
import { Principal } from '@dfinity/principal';
import { Queries } from '@lib/data';
import { useUser } from '@lib/user';
import { callActorMutation } from '@lib/utils/call-actor-mutation';
import { useQueries, useQuery } from '@tanstack/react-query';
import {
  CurrencyReceiver, CurrencyTypeIconComponent, useAuth, useCurrencyManagerMeta
} from '@zk-game-dao/currency';
import { ErrorComponent } from '@zk-game-dao/ui';

import { SpinnerComponent } from '../../../components/common/spinner/spinner.component';
import { usePersistentState } from '../../../hooks/persitent-state';
import { matchRustEnum } from '../../utils/rust';
import { DateToBigIntTimestamp } from '../../utils/time';
import { TokenAmountToFloat } from '../../utils/token-amount-conversion';

type UserTournamentRanking = {
  table?: PublicTable;
  ranking: UserTournamentData;
  principal: Principal;
};

export type JoinType = { type: 'reentry' | 'join' | 'late' | 'rebuy', amount: bigint };

type TournamentContext = {
  data: TournamentData;
  prizepool: bigint;
  actor: typeof tournament_canister;
  tables: PublicTable[];

  user?: UserTournamentRanking;

  joinType?: JoinType;

  buyInOptions?: BuyInOptions;

  isRunning: boolean;
  currencyType: CurrencyType;
  receiver: CurrencyReceiver;
};

const Context = createContext<TournamentContext | undefined>(undefined);

export const ProvideTournamentContext = memo<PropsWithChildren<{ id: Principal }>>(({ id, children }) => {
  const { authData } = useAuth();
  const { user: zkpUser } = useUser();
  const actor = useMemo(() => createActor(id, { agent: authData?.agent }), [id, authData]);
  const { data, isPending, error } = useQuery({
    queryKey: Queries.tournament.key(id),
    queryFn: async () => await callActorMutation(actor, "get_tournament"),
    refetchInterval: 10000,
  });
  const prizePool = useQuery({
    queryKey: Queries.tournamentPrizePool.key(id),
    queryFn: () => actor.get_total_prize_pool(),
    refetchInterval: 10000,
    initialData: 0n
  });
  const buyInOptions = useMemo(() => {
    if (!data) return;
    const size = matchRustEnum(data.tournament_type)({
      SitAndGo: (size) => size,
      SpinAndGo: ([size]) => size,
      Freeroll: (size) => size,
      BuyIn: (size) => size,
    });

    return matchRustEnum(size)({
      SingleTable: (size) => size,
      MultiTable: ([size]) => size,
    });
  }, [data]);

  const leaderboard = useQuery({
    queryKey: ['leaderboard', id.toText()],
    queryFn: async () => callActorMutation(actor, 'get_leaderboard'),
    select: (data) => data.map(([canister_id]) => canister_id),
    refetchInterval: 10000,
    initialData: []
  });

  const tables = useQueries({
    queries: data?.tables?.map(([id]) => ({
      queryKey: Queries.table.key({ id }),
      queryFn: async (): Promise<PublicTable> => await callActorMutation(createTableActor(id, authData), "get_table"),
      refetchInterval: 10000,
    })) || [],
    combine: (result) =>
      result.map((r) => r.data).filter((r): r is PublicTable => !!r),
  });

  const currencyType = useMemo(() => (data?.currency ?? { Fake: null }), [data]);
  const user = useMemo((): UserTournamentRanking | undefined => {
    if (!zkpUser || !data) return;

    const ranking = data.current_players.find(([u]) => u.compareTo(zkpUser.principal_id) === 'eq')?.[1];

    if (!ranking)
      return;

    return {
      principal: zkpUser.principal_id,
      ranking,
      table: tables.find((t) => t.users.users.some(([pri]) => pri.compareTo(zkpUser.principal_id) === 'eq'))
    }
  }, [zkpUser, data, tables]);

  const receiver = useMemo((): CurrencyReceiver => ({ principal: id }), [id]);

  const [currentTime, setCurrentTime] = useState(DateToBigIntTimestamp(new Date()));

  useEffect(() => {
    const interval = setInterval(() => setCurrentTime(DateToBigIntTimestamp(new Date())), 10000);
    return () => clearInterval(interval);
  }, []);

  const isRunning = useMemo(() => {
    if (!data || 'Registration' in data.state) return false;
    return currentTime > data.start_time;
  }, [currentTime, data]);

  const joinType = useMemo((): JoinType | undefined => {
    if (!buyInOptions || !data || !zkpUser)
      return;

    if (!isRunning) {
      if (user)
        return undefined;
      if ('Freeroll' in data.tournament_type)
        return { type: 'join', amount: 0n };
      return { type: 'join', amount: data.buy_in };
    }

    if (user) {
      if (
        !user.table &&
        buyInOptions.rebuy.enabled &&
        currentTime < buyInOptions.rebuy.rebuy_end_timestamp
      )
        return { type: 'rebuy', amount: buyInOptions.rebuy.rebuy_price };
      return undefined;
    }

    // Was a user but got kicked out
    if (leaderboard.data.find(user_id => zkpUser.principal_id.compareTo(user_id) === 'eq')) {
      if (buyInOptions.reentry.enabled && currentTime < buyInOptions.reentry.reentry_end_timestamp)
        return { type: 'reentry', amount: data.buy_in };
      return undefined;
    }

    if (currentTime < (data.start_time + data.late_registration_duration_ns))
      return { type: 'late', amount: data.buy_in };

    return undefined;
  }, [isRunning, buyInOptions, data, user, currentTime, zkpUser, leaderboard.data]);


  const [didAnimateBefore, setDidAnimateBefore] = usePersistentState<boolean>(`${data?.id.toText()}-did-spin`, false);
  const meta = useCurrencyManagerMeta(currencyType);

  const [spinAnimation, setSpinAnimation] = useState<number>();
  const spinMultiplierValue = useMemo(() => {
    if (!data) return;
    if (!('SpinAndGo' in data.tournament_type) || data.tournament_type.SpinAndGo[1].multiplier === 0n) return;
    return TokenAmountToFloat(data.tournament_type.SpinAndGo[1].multiplier * data.buy_in, meta);
  }, [data]);

  useEffect(() => {
    if (spinMultiplierValue === undefined || didAnimateBefore || false) return;
    setDidAnimateBefore(true);
    setSpinAnimation(spinMultiplierValue);
  }, [spinMultiplierValue]);

  if (!data)
    return isPending ?
      null : (
        <div className='container mx-auto'>
          <ErrorComponent error={error ?? new Error("Unknown")} title="Failed to fetch tournament" />
        </div>
      );

  return (
    <Context.Provider
      value={{
        prizepool: prizePool.data,
        data,
        actor,
        currencyType,
        tables,
        receiver,
        user,
        joinType,
        isRunning,
        buyInOptions,
      }}
    >
      {typeof spinAnimation === 'number' && (
        <SpinnerComponent
          symbol={<CurrencyTypeIconComponent className='scale-200' currencyType={currencyType} />}
          onFinish={() => setSpinAnimation(undefined)}
          result={spinAnimation}
        />
      )}
      {children}
    </Context.Provider >
  );
});
ProvideTournamentContext.displayName = 'ProvideTournamentContext';

export const ProvideRawTournamentContext = Context.Provider;

export function useTournament<T extends boolean>(required?: T): T extends true ? TournamentContext : (TournamentContext | undefined) {
  const context = useContext(Context);
  if (required && !context) throw new Error("Tournament context not provided");
  return context as any;
};

export function ConsumeTournamentContext<T extends boolean>({ children, required }: {
  required?: T;
  children: (context: T extends true ? TournamentContext : TournamentContext | undefined) => React.ReactNode;
}) {
  const context = useTournament(required);
  return <>{children(context)}</>;
}
