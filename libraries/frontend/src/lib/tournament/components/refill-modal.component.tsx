import { addMinutes, formatDistance } from 'date-fns';
import { memo, useEffect, useMemo, useState } from 'react';

import { useTournament } from '@lib/tournament/context/tournament.context';
import { callActorMutation } from '@lib/utils/call-actor-mutation';
import { BigIntTimestampToDate, DateToBigIntTimestamp } from '@lib/utils/time';
import { CurrencyComponent, useAllowance, useAuth } from '@zk-game-dao/currency';
import {
  ButtonComponent, ErrorComponent, List, ListItem, Modal, ModalFooterPortal,
  TransferLoadingIndicatorComponent, useMutation, UserError, useToast
} from '@zk-game-dao/ui';

export const RefillModal = memo<{ onClose?(): void; }>(({ onClose }) => {
  const tournament = useTournament(true);
  const addon = useMemo(() => tournament.buyInOptions?.addon, [tournament]);
  const { authData } = useAuth();

  const [timestamp, setTimestamp] = useState(DateToBigIntTimestamp(new Date()));
  const allowance = useAllowance({ currencyType: tournament.currencyType, receiver: { principal: tournament.data.id }, name: 'Tournament' });
  const { addToast } = useToast();

  useEffect(() => {
    const interval = setInterval(() => setTimestamp(DateToBigIntTimestamp(new Date())), 1000);
    return () => clearInterval(interval);
  }, []);

  const purchaseAddonMutation = useMutation({
    mutationFn: async () => {
      if (!authData) throw new UserError("User is not authenticated");
      if (!tournament.user) throw new UserError("User is not in the tournament");
      if (!addon?.enabled) throw new UserError("Addon is not enabled");
      if (!tournament.user.table) throw new UserError("User is not in a table");

      await allowance.require({ amount: addon.addon_price, reason: 'Purchase addon' }, addMinutes(new Date(), 2));

      return await callActorMutation(tournament.actor, 'user_refill_chips', tournament.user.principal, tournament.user.table.id, authData.principal);
    },
    onSuccess: () => {
      addToast({
        children: "Addon purchased successfully",
      })
    }
  })

  if (!addon?.enabled) return null;

  return (
    <Modal open title="Addon Pause" onClose={onClose}>
      <p>The tournament is paused for addon purchases. The tournament will resume in {formatDistance(BigIntTimestampToDate(addon.addon_end_time), BigIntTimestampToDate(timestamp))}</p>

      <ErrorComponent error={purchaseAddonMutation.error} />
      <TransferLoadingIndicatorComponent
        isProcessing={purchaseAddonMutation.isPending}
        isTransferring={false}
      >
        Purchasing addon
      </TransferLoadingIndicatorComponent>

      {tournament.user && (
        <>
          <List>
            <ListItem rightLabel={<CurrencyComponent currencyValue={tournament.user.ranking.chips} currencyType={{ Fake: null }} />}>Your current chips</ListItem>
            <ListItem rightLabel={<CurrencyComponent currencyValue={addon.addon_price} currencyType={tournament.currencyType} />}>Addon price</ListItem>
            <ListItem rightLabel={<CurrencyComponent currencyValue={addon.addon_chips} currencyType={{ Fake: null }} />}>Addon chips</ListItem>
            <ListItem rightLabel={addon.max_addons - tournament.user.ranking.addons}>Remaining addons</ListItem>
          </List>

          <ModalFooterPortal>
            {onClose && (
              <ButtonComponent variant="naked" onClick={onClose}>
                Cancel
              </ButtonComponent>
            )}
            <ButtonComponent className='w-full' onClick={purchaseAddonMutation.mutate} isLoading={purchaseAddonMutation.isPending}>
              Purchase Addon
            </ButtonComponent>
          </ModalFooterPortal>
        </>
      )}
    </Modal>
  );
});
RefillModal.displayName = 'RefillModal';
