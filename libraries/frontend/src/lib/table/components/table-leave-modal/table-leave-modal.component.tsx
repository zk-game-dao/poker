import { memo, useCallback, useMemo } from 'react';
import { Blocker } from 'react-router-dom';

import { Queries } from '@lib/data';
import { useTournament } from '@lib/tournament/context/tournament.context';
import { useUser } from '@lib/user';
import { callActorMutation } from '@lib/utils/call-actor-mutation';
import { useMutation } from '@tanstack/react-query';
import { useAuth } from '@zk-game-dao/currency';
import { ButtonComponent, ErrorComponent, Modal, ModalFooterPortal } from '@zk-game-dao/ui';

import { useTable } from '../../context/table.context';

export const TableLeaveModalComponent = memo<{
  blocker: Blocker;
  onClose?: () => void;
}>(({ blocker, onClose }) => {
  const {
    isJoined,
    isOngoing,
    actor: service,
    currencyType: currency,
    user: { status } = {},
  } = useTable();
  const { user } = useUser();
  const { authData } = useAuth();
  const tournament = useTournament();

  const isQueued = useMemo(() => status && "QueuedForNextRound" in status, [status]);

  const needsToFold = useMemo(
    () => isOngoing && isJoined && !isQueued || true,
    [isOngoing, isJoined, isQueued],
  );

  const cancel = useCallback(() => {
    if (blocker.reset) blocker.reset();
    if (onClose) onClose();
  }, [onClose, blocker]);

  const { mutate, isPending, error } = useMutation({
    mutationFn: async () => {
      if (!isJoined) return;
      if (!user) throw new Error("User not found");

      if (tournament) {
        if (!tournament.user || !authData) throw new Error("You are not in this tournament");
        if (!tournament.user.table) throw new Error("You are not in a table");
        return callActorMutation(
          tournament.actor,
          'user_leave_tournament',
          tournament.user.principal,
          authData.principal
        );
      }

      return callActorMutation(
        service,
        "leave_table",
        user.users_canister_id,
        user.principal_id,
      );
    },
    onSuccess: () => {
      Queries.walletBalance.invalidate(currency, authData);
      if (blocker.proceed) blocker.proceed();
    },
  });

  return (
    <Modal
      open={blocker.state === "blocked"}
      onClose={cancel}
      title={tournament ? 'Leave tournament' : 'Leave table'}
    >
      <div className="flex flex-col text-center gap-3">
        <p className="type-header">Do you want to {tournament ? 'leave the tournament' : 'return to the lobby'}?</p>
        <p className="type-body text-material-heavy-1">
          {tournament ?
            'You will be removed from the tournament and your buy-in will be returned to your account.' :
            'Your table balance will be returned to your account after the round is finished which may take a few minutes.'}
        </p>
      </div>
      <ErrorComponent error={error} />
      <ModalFooterPortal>
        <ButtonComponent variant="naked" onClick={cancel}>
          No, stay seated
        </ButtonComponent>
        <ButtonComponent
          onClick={mutate}
          color={tournament || needsToFold ? "red" : undefined}
          isLoading={isPending}
        >
          {tournament ? 'Yes, leave tournament' : `Yes, ${needsToFold ? "fold and " : ""}leave table`}
        </ButtonComponent>
      </ModalFooterPortal>
    </Modal>
  );
});
TableLeaveModalComponent.displayName = "TableLeaveModalComponent";
