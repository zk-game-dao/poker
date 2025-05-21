import classNames from 'classnames';
import { memo, useCallback } from 'react';

import { PayoutPercentage } from '@declarations/tournament_canister/tournament_canister.did';
import { List, LoadingSpinnerComponent } from '@zk-game-dao/ui';

import {
  TournamentLeaderboardEntry, TournamentLeaderboardEntryData
} from './tournament-leaderboard-entry.component';
import { Principal } from '@dfinity/principal';
import { CurrencyType, IsSameCurrencyType, IsSamePrincipal } from '@zk-game-dao/currency';
import { IsSamePayoutStructure } from '../../../utils/compare';

export type LeaderboardData = {
  isRefetching: boolean;
  data: TournamentLeaderboardEntryData[];
  refetch(): void;
}

export const TournamentLeaderboardComponent = memo<{
  isCompleted: boolean;
  className?: string;
  payoutStructure: PayoutPercentage[];
  prizepool: bigint;
  currencyType: CurrencyType;
  tournamentUserId?: Principal;
  liveLeaderboard: LeaderboardData;
  /** Eliminated players */
  leaderboard: LeaderboardData;
}>(({
  leaderboard,
  liveLeaderboard,
  className,
  isCompleted,
  payoutStructure,
  prizepool,
  currencyType,
  tournamentUserId
}) => {

  const isSelf = useCallback((user_id: Principal) => tournamentUserId && IsSamePrincipal(user_id, tournamentUserId), [tournamentUserId?.toText()]);
  const getWinnings = useCallback((rank: number): bigint => {
    const winningsPercentage = payoutStructure[rank];
    if (!winningsPercentage) return 0n;
    return prizepool / 100n * BigInt(winningsPercentage.percentage);
  }, [payoutStructure, prizepool]);

  return (
    <div className='flex flex-col gap-6 relative min-h-32'>
      {(leaderboard.isRefetching || liveLeaderboard.isRefetching) && <LoadingSpinnerComponent className='absolute inset-0' />}
      {!isCompleted && (
        <List
          label="Live leaderboard"
          className={classNames(className, 'mx-auto')}
          ctas={[{ label: 'Refresh', onClick: () => liveLeaderboard.refetch() }]}
        >
          {liveLeaderboard.data
            .filter(v => !leaderboard.data.find(c => c.user_id.compareTo(v.user_id) === 'eq'))
            .map(rank => (
              <TournamentLeaderboardEntry
                key={rank.rank}
                {...rank}
                currencyType={currencyType}
                isCompleted={isCompleted}
                winnings={getWinnings(rank.rank)}
                isSelf={isSelf(rank.user_id)}
              />
            ))}
        </List>
      )}

      {!isCompleted && liveLeaderboard.data.length === 0 && leaderboard.data.length === 0 && (
        <p className='text-center type-body text-material-heavy-1'>There are no rankings yet</p>
      )}

      {leaderboard.data.length > 0 && (
        <List
          label={isCompleted ? 'Rankings' : 'Eliminated players'}
          className={classNames(className, 'mx-auto')}
          ctas={isCompleted ? undefined : [{ label: 'Refresh', onClick: () => leaderboard.refetch() }]}
        >
          {leaderboard.data.map(rank => (
            <TournamentLeaderboardEntry
              key={rank.rank}
              {...rank}
              rank={rank.rank - 1}
              isCompleted={isCompleted}
              currencyType={currencyType}
              winnings={isCompleted ? getWinnings(rank.rank) : undefined}
              isSelf={isSelf(rank.user_id)}
            />
          ))}
        </List>
      )}

    </div>
  );
},
  (prevProps, nextProps) =>
    prevProps.className === nextProps.className &&
    prevProps.isCompleted === nextProps.isCompleted &&
    prevProps.leaderboard.isRefetching === nextProps.leaderboard.isRefetching &&
    prevProps.leaderboard.data.length === nextProps.leaderboard.data.length &&
    prevProps.leaderboard.data.every((v, i) => v.user_id.compareTo(nextProps.leaderboard.data[i].user_id) === 'eq') &&
    prevProps.liveLeaderboard.isRefetching === nextProps.liveLeaderboard.isRefetching &&
    prevProps.liveLeaderboard.data.length === nextProps.liveLeaderboard.data.length &&
    prevProps.liveLeaderboard.data.every((v, i) => v.user_id.compareTo(nextProps.liveLeaderboard.data[i].user_id) === 'eq') &&

    IsSamePrincipal(prevProps.tournamentUserId, nextProps.tournamentUserId) &&
    IsSamePayoutStructure(prevProps.payoutStructure, nextProps.payoutStructure) &&
    prevProps.prizepool === nextProps.prizepool &&
    IsSameCurrencyType(prevProps.currencyType, nextProps.currencyType) &&
    prevProps.isCompleted === nextProps.isCompleted
);
TournamentLeaderboardComponent.displayName = 'TournamentLeaderboardComponent';