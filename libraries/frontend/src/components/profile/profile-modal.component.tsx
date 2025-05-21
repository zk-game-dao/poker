import { useUserExperiencePoints } from '#hooks/experience-points';
import { memo, useMemo, useState } from 'react';

import { User } from '@declarations/users_canister/users_canister.did';
import { Principal } from '@dfinity/principal';
import {
  TableBalanceModalComponent
} from '@lib/table/components/deposit-table-modal/table-balance-modal.components';
import { useTable } from '@lib/table/context/table.context';
import { useUser } from '@lib/user';
import { CurrencyComponent } from '@zk-game-dao/currency';
import { ExperiencePointsComponent, List, ListItem, Modal, PillComponent } from '@zk-game-dao/ui';

import {
  TournamentRankingComponent
} from '../../lib/tournament/components/tournament-ranking.component';
import { useTournament } from '../../lib/tournament/context/tournament.context';
import { AvatarComponent } from '../common/avatar/avatar.component';

// TODO: Reuse this component for the user profile modal
export const ProfileModalComponent = memo<{
  user: User;
  onClose(): void;
  isOpen?: boolean;
}>(({ isOpen, user, onClose }) => {
  const { user: currentUser } = useUser();
  const [showBalance, setShowBalance] = useState(false);

  const { table, currencyType: currency } = useTable();
  const tournament = useTournament();

  const isSelf = useMemo(
    () => currentUser?.principal_id.compareTo(user.principal_id) === "eq",
    [currentUser?.principal_id, user.principal_id],
  );
  const { data: experience_points = 0n } = useUserExperiencePoints(user);

  const queuedItems = useMemo(
    () =>
      !table || !("queue" in table)
        ? []
        : table.queue.filter(
          (item): item is { Deposit: [Principal, Principal, bigint] } => {
            let user_canister_principal: Principal | undefined;
            if ("Deposit" in item)
              user_canister_principal = item.Deposit[0];
            if ("SittingIn" in item)
              user_canister_principal = item.SittingIn[0];
            return user_canister_principal?.compareTo(user.principal_id) === "eq";
          },
        ),
    [table.queue, user.principal_id],
  );

  return (
    <Modal open={isOpen} onClose={onClose}>
      <AvatarComponent size="big" className="mx-auto" {...user} />

      <div className="text-center flex flex-col justify-center">
        <p className="type-header mb-2">{user.user_name}</p>

        <div className="flex flex-row gap-2 justify-center">
          <ExperiencePointsComponent
            experience_points={[experience_points]}
            size="small"
            className="material px-2 py-1 text-material-heavy-1"
          />
          <CurrencyComponent
            currencyType={currency}
            forceFlex
            size="small"
            className="text-material-heavy-1 material px-2 py-1"
            currencyValue={user.balance}
          />
        </div>


        {isSelf && !tournament && (
          <>
            <PillComponent
              className="mt-4"
              onClick={() => setShowBalance(true)}
            >
              Manage table balance
            </PillComponent>
            <TableBalanceModalComponent
              isOpen={showBalance}
              onClose={() => setShowBalance(false)}
            />
          </>
        )}

        {tournament && <TournamentRankingComponent className='mt-4' principal={user.principal_id} />}

        {queuedItems.length > 0 && (
          <>
            <p className="type-callout text-material-medium-2 mr-auto mb-2 mt-3">
              Queued for next round
            </p>
            <List>
              {queuedItems.map((item, i) => {
                if ("LeaveTable" in item)
                  return <ListItem key={i}>Leave</ListItem>;
                if ("SittingIn" in item)
                  return <ListItem key={i}>Sitting in</ListItem>;
                return (
                  <ListItem
                    key={i}
                    rightLabel={
                      <CurrencyComponent currencyType={currency} currencyValue={item.Deposit[2]} />
                    }
                  >
                    Queued deposit
                  </ListItem>
                );
              })}
            </List>
          </>
        )}
      </div>
    </Modal>
  );
});
ProfileModalComponent.displayName = 'ProfileModalComponent';
