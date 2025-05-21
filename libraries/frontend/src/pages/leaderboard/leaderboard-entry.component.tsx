import { memo } from 'react';

import { Principal } from '@dfinity/principal';
import { IsSamePrincipal } from '@zk-game-dao/currency';
import { ExperiencePointsComponent, ListItem, LoadingAnimationComponent } from '@zk-game-dao/ui';

import AvatarComponent from '../../components/common/avatar/avatar.component';
import { useUser } from '../../lib/user';
import { useUserFromUserId } from '../../lib/user/hooks/use-user';

export const LeaderboardEntry = memo<{
  experience_points: bigint;
  user_id: Principal;
  rank: number;
}>(({ experience_points, user_id, rank }) => {
  const { user: self } = useUser();

  const user = useUserFromUserId(user_id);

  return (
    <ListItem
      icon={<span className='px-4'>{rank + 1}</span>}
      rightLabel={<ExperiencePointsComponent experience_points={[experience_points]} />}
    >
      <AvatarComponent size='microscopic' {...user.data} className='mr-2' />
      {!user.data ?
        <LoadingAnimationComponent variant="shimmer">{user_id.toText()}</LoadingAnimationComponent> :
        user.data.user_name}
      {IsSamePrincipal(self?.principal_id, user_id) && <span className='material rounded-full type-tiny px-2 py-1 mr-auto ml-1'> You</span>}
    </ListItem>
  );
},
  (prevProps, nextProps) =>
    IsSamePrincipal(prevProps.user_id, nextProps.user_id) &&
    prevProps.experience_points === nextProps.experience_points &&
    // prevProps.isFirst === nextProps.isFirst &&
    prevProps.rank === nextProps.rank
);
LeaderboardEntry.displayName = 'LeaderboardEntry';
