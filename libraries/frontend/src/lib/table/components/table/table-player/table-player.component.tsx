import { ProfileModalComponent } from '#ui/profile/profile-modal.component';
import { PlayerTag } from '@lib/ui/player-tag/player-tag.component';
import { useUser } from '@lib/user';
import { memo, useState } from 'react';

import { useTableSeat } from '../../../context/table-seat.context';
import { useTable } from '../../../context/table.context';
import { TakeASeatComponent } from './take-a-seat.component';

export const TablePlayer = memo(() => {
  const { isJoined, currencyType: currency } = useTable();
  const { user: zkpUser } = useUser();
  const { user, data, cards, isSelf, position, userTurnProgress, isDealer, isQueued } = useTableSeat();
  const [isShowingProfile, setIsShowingProfile] = useState(false);

  if (user)
    return (
      <>
        <ProfileModalComponent
          user={user}
          onClose={() => setIsShowingProfile(false)}
          isOpen={isShowingProfile}
        />
        <PlayerTag
          onClick={() => setIsShowingProfile(true)}
          {...data}
          {...user}
          currencyType={currency}
          cards={cards}
          isSelf={isSelf}
          direction={position.vertical === 'top' ? 'down' : 'up'}
          turnProgress={userTurnProgress}
          isQueued={isQueued}
          isDealer={isDealer}
        />
      </>
    );

  if (!!zkpUser && !isJoined)
    return <TakeASeatComponent />;

  return <></>;
});
TablePlayer.displayName = 'TablePlayer';
