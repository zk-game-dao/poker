import { addMinutes } from 'date-fns';
import { memo } from 'react';

import { createActor } from '@declarations/tournament_canister';
import { TournamentData } from '@declarations/tournament_index/tournament_index.did';
import { useMutation } from '@tanstack/react-query';
import { CurrencyComponent, CurrencyType, useAllowance, useAuth } from '@zk-game-dao/currency';
import {
  ButtonComponent, ErrorComponent, Modal, ModalFooterPortal, TransferLoadingIndicatorComponent,
  useToast
} from '@zk-game-dao/ui';

import { useRouting } from '../../../hooks/routing';
import { Queries } from '../../data';
import { useUser } from '../../user';
import { callActorMutation } from '../../utils/call-actor-mutation';

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

export const QuickJoinModal = memo<Pick<Props, 'tournament_type' | 'id' | 'buy_in' | 'tables'> & { open: boolean, onClose: () => void; currencyType: CurrencyType }>(({
  open, onClose, currencyType: currencyType, buy_in, id, tables, tournament_type }) => {
  const { user } = useUser();
  const { authData } = useAuth();
  const { addToast } = useToast();

  const allowance = useAllowance({ currencyType, receiver: { principal: id }, name: 'Tournament' });

  const { push } = useRouting();

  const quickJoinMutation = useMutation({
    mutationFn: async () => {
      if (!authData) throw new Error('You are not logged in');
      if (!user) throw new Error('Cannot join tournament');
      if (!('Freeroll' in tournament_type))
        await allowance.require({ amount: buy_in, reason: 'Join tournament' }, addMinutes(new Date(), 2));
      const actor = createActor(id, authData);
      return await callActorMutation(actor, 'user_join_tournament', user.users_canister_id, authData.principal);
    },
    onSuccess: () => {
      if (tables.length > 0) {
        const table = tables[0];
        push(`/tournaments/${id}/tables/${table[0].toText()}`);
      } else {
        push(`/tournaments/${id}`);
      }
      addToast({ children: 'You have joined the tournament' });
      Queries.tournament.invalidate(id);
    },
  });

  return (
    <Modal open={open} onClose={onClose}>
      <div className="flex flex-col gap-3">
        <p className="type-top text-center w-full">
          Quick join tournament
        </p>
        <div className="text-center w-full whitespace-pre-wrap inline">
          {'Freeroll' in tournament_type ?
            'This is a freeroll tournament. You can join for free.' : <>
              {'You need to buy in for '}
              <CurrencyComponent
                currencyType={currencyType}
                size="small"
                className="inline-flex mx-1"
                currencyValue={buy_in}
              />
            </>}
        </div>
      </div>
      <TransferLoadingIndicatorComponent
        isTransferring={quickJoinMutation.isPending}
        isProcessing={quickJoinMutation.isPending}
      />
      <ErrorComponent
        error={quickJoinMutation.error}
      />
      <ModalFooterPortal>
        <ButtonComponent onClick={onClose} variant="naked">Cancel</ButtonComponent>
        <ButtonComponent onClick={quickJoinMutation.mutateAsync} isLoading={quickJoinMutation.isPending}>Join</ButtonComponent>
      </ModalFooterPortal>
    </Modal>
  );
});
QuickJoinModal.displayName = 'QuickJoinModal';
