import { useMutation } from '@tanstack/react-query';
import { CurrencyComponent, useAllowance, useAuth } from '@zk-game-dao/currency';
import {
  ButtonComponent,
  ErrorComponent,
  FauxLoadingBarAnimationComponent,
  Modal,
  ModalFooterPortal,
  UserError,
  useToast,
} from '@zk-game-dao/ui';
import { addMinutes } from 'date-fns';
import { memo, useMemo } from 'react';

import { useRouting } from '../../../hooks/routing';
import { Queries } from '../../data';
import { useUser } from '../../user';
import { callActorMutation } from '../../utils/call-actor-mutation';
import { matchRustEnum } from '../../utils/rust';
import { secondsToLabel } from '../../utils/time';
import { useTournament } from '../context/tournament.context';

type UserEnterTexts = {
  title: string;
  description: string;
  cta: string;
  loading: string;
};

export const useEnterTexts = (): UserEnterTexts | undefined => {
  const tournament = useTournament();

  return useMemo(() => {
    if (!tournament?.joinType) return;
    switch (tournament.joinType.type) {
      case 'join':
        return { title: 'Join the tournament', description: 'You can join the tournament for', cta: 'Join', loading: 'Joining' };
      case 'late':
        return { title: 'Late join the tournament', description: 'You can late join the tournament for', cta: 'Late join', loading: 'Late joining' };
      case 'reentry':
        return { title: 'Reenter the tournament', description: 'You can reenter the tournament for', cta: 'Reenter', loading: 'Reentering' };
      case 'rebuy': {
        const size = matchRustEnum(tournament.data.tournament_type)({
          SitAndGo: (size) => size,
          SpinAndGo: ([size]) => size,
          Freeroll: (size) => size,
          BuyIn: (size) => size,
        });
        const secs = matchRustEnum(size)({
          SingleTable: (size) => size.rebuy.rebuy_window_seconds,
          MultiTable: ([size]) => size.rebuy.rebuy_window_seconds,
        });
        return {
          title: 'Rebuy in the tournament',
          description: `You have ${secondsToLabel(Number(secs))} to rebuy in the tournament for`,
          cta: 'Rebuy',
          loading: 'Rebuying'
        };
      }
    }
  }, [tournament])
}

export const EnterTournamentModalComponent = memo<{ open: boolean; onClose?(): void; }>(({ open, onClose }) => {
  const tournament = useTournament(true);
  const { authData } = useAuth();
  const { user: zkpUser } = useUser();
  const { addToast } = useToast();
  const tournamentAllowance = useAllowance(tournament && { currencyType: tournament.currencyType, receiver: tournament.receiver, name: 'Tournament' });
  const { push } = useRouting();
  const texts = useEnterTexts();

  const joinMutation = useMutation({
    mutationFn: async () => {
      if (!tournament) throw new UserError("Tournament not found");
      if (!zkpUser || !authData) throw "Table or user not found";
      if (!tournament.joinType) throw new UserError("You can't join this tournament");

      const table = tournament.tables[0];
      if (!table) throw new UserError("Table not found");


      // if (!method) throw new UserError("Invalid join type");

      if (tournament.joinType.amount > 0n)
        await tournamentAllowance.require({ amount: tournament.joinType.amount, reason: 'Join tournament' }, addMinutes(new Date(), 2));

      switch (tournament.joinType.type) {
        case 'late':
        case 'join':
          await callActorMutation(
            tournament.actor,
            'user_join_tournament',
            zkpUser.users_canister_id,
            authData.principal
          );
          break;
        case 'reentry':
          await callActorMutation(
            tournament.actor,
            'user_reentry_into_tournament',
            zkpUser.users_canister_id,
            authData.principal,
            table.id
          );
          break;
        case 'rebuy':
          await callActorMutation(
            tournament.actor,
            'user_rebuy_into_tournament',
            zkpUser.users_canister_id,
            authData.principal,
            table.id
          );
          break;
      }
      return [table, tournament] as const;
    },
    onSuccess: ([table, tournament]) => {
      Queries.table.invalidate(table);
      Queries.walletBalance.invalidate(tournament.currencyType, authData);
      Queries.tournament.invalidate(tournament.data.id);
      Queries.tournament_leaderboard.invalidate(tournament.data.id);
      Queries.tournamentIsRunning.invalidate(tournament.data.id);
      Queries.tournamentPrizePool.invalidate(tournament.data.id);

      addToast({ children: "You have joined the tournament" });
      push(`/tournaments/${tournament.data.id}/table/${table.id}`);
    },
  });

  if (!tournament?.joinType || !texts) return null;

  return (
    <Modal open={open} onClose={onClose} title={texts.title}>
      <div className="flex flex-col gap-3">
        <p className="type-top text-center w-full">{texts.title}</p>
        <div className="text-center w-full whitespace-pre-wrap inline">
          {texts.description + ' '}
          {tournament.joinType.amount === 0n ? 'free.' : (
            <CurrencyComponent
              currencyType={tournament.currencyType}
              size="small"
              className="inline-flex mx-1"
              currencyValue={tournament.joinType.amount}
            />
          )}
        </div>
      </div>

      <ErrorComponent error={joinMutation.error} />

      {joinMutation.isPending && (
        <FauxLoadingBarAnimationComponent>
          {texts.loading}
        </FauxLoadingBarAnimationComponent>
      )}

      <ModalFooterPortal>
        {onClose && (
          <ButtonComponent
            onClick={onClose}
            variant="naked"
          >
            Cancel
          </ButtonComponent>
        )}

        <ButtonComponent
          onClick={joinMutation.mutateAsync}
          isLoading={joinMutation.isPending}
        >
          {texts.cta}
        </ButtonComponent>
      </ModalFooterPortal>
    </Modal>
  )
});
EnterTournamentModalComponent.displayName = 'EnterTournamentModalComponent';
