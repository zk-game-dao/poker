import { memo } from 'react';

import { Principal } from '@dfinity/principal';
import { CurrencyComponent, CurrencyType, IsSameCurrencyType, IsSamePrincipal } from '@zk-game-dao/currency';
import { ListItem, LoadingAnimationComponent } from '@zk-game-dao/ui';

import { AvatarComponent } from '../../../../components/common/avatar/avatar.component';
import { useUserFromUserId } from '../../../user/hooks/use-user';

export type TournamentLeaderboardEntryData = {
  user_id: Principal;
  chips?: bigint;
  rank: number;
};

export type TournamentLeaderboardEntryProps = TournamentLeaderboardEntryData & {
  currencyType: CurrencyType;
  isCompleted: boolean;
  winnings?: bigint;
  isSelf?: boolean;
};

export const TournamentLeaderboardEntry = memo<TournamentLeaderboardEntryProps>(({
  rank,
  chips,
  user_id,
  isCompleted,
  currencyType,
  winnings,
  isSelf
}) => {
  const { data: user, isPending } = useUserFromUserId(user_id);
  return (
    <ListItem
      icon={<span className='px-4'>{rank + 1}</span>}
      rightLabel={isCompleted ?
        <CurrencyComponent currencyType={currencyType} currencyValue={winnings} /> :
        <CurrencyComponent currencyType={{ Fake: null }} currencyValue={chips} />}
    >
      <AvatarComponent size='microscopic' {...user} className='mr-2' />
      {isPending || !user ? (
        <LoadingAnimationComponent variant="shimmer">
          {user_id.toText()}
        </LoadingAnimationComponent>
      ) : <>{user.user_name}</>}
      {isSelf && <span className='material rounded-full type-tiny px-2 py-1 mr-auto ml-1'> You</span>}
    </ListItem>
  );
},
  (prevProps, nextProps) =>
    prevProps.rank === nextProps.rank &&
    IsSamePrincipal(prevProps.user_id, nextProps.user_id) &&
    prevProps.isCompleted === nextProps.isCompleted &&
    prevProps.chips === nextProps.chips &&
    IsSameCurrencyType(prevProps.currencyType, nextProps.currencyType) &&
    prevProps.winnings === nextProps.winnings &&
    prevProps.isSelf === nextProps.isSelf
);
TournamentLeaderboardEntry.displayName = 'LeaderboardEntry';
