import { memo } from 'react';

import { TournamentState } from '@declarations/tournament_canister/tournament_canister.did';
import { BigIntTimestampToDate } from '@lib/utils/time';
import { LoadingAnimationComponent } from '@zk-game-dao/ui';

import { useFormatDateDistance } from '../../../hooks/countdown';

export const TournamentStatusComponent = memo<{ state: TournamentState; start_time: bigint }>(({ state, start_time }) => {
  const timeUntilStart = useFormatDateDistance(BigIntTimestampToDate(start_time));
  if ('Registration' in state) {
    if (timeUntilStart && timeUntilStart?.number < 0)
      return <LoadingAnimationComponent variant="shimmer">Starting</LoadingAnimationComponent>
    return <span>Registration</span>;
  }
  if ('FinalTable' in state) return <span>Final table</span>;
  if ('LateRegistration' in state) return <span>Late registration</span>;
  if ('Running' in state) return <span>Running</span>;
  if ('Cancelled' in state) return <span className='text-red-500'>Cancelled</span>;
  if ('Completed' in state) return <span className='text-green-500'>Completed</span>;
  return <span>Unknown</span>;
});
TournamentStatusComponent.displayName = 'TournamentStatusComponent';
