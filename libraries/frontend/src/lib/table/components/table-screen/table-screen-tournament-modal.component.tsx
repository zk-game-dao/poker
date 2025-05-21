import { memo, useState } from 'react';

import { ButtonComponent, Modal, ModalFooterPortal, PillComponent } from '@zk-game-dao/ui';

import { useFormatDateDistance } from '../../../../hooks/countdown';
import { TournamentMetaComponent } from '../../../tournament/components/tournament-meta.component';
import { TournamentUserComponent } from '../../../tournament/components/tournament-user.component';
import { useTournament } from '../../../tournament/context/tournament.context';
import {
  useNextMultitableRebalanceDateTime
} from '../../../tournament/hooks/get-multitable-rebalance-time';

const RebalanceSpan = memo<{ nextRebalanceTime: Date }>(({ nextRebalanceTime }) => {
  const timeUntil = useFormatDateDistance(nextRebalanceTime);

  if (!timeUntil) return null;
  if (timeUntil.number < 0) return <span className='type-tiny text-material-medium-2'>Rebalancing now</span>;

  return <span className='type-tiny text-material-medium-2'>Rebalancing in {timeUntil?.string}</span>;
}, (prev, next) => prev.nextRebalanceTime.getTime() === next.nextRebalanceTime.getTime());
RebalanceSpan.displayName = 'RebalanceSpan';

const TournamentModal = memo<{ open: boolean; onClose(): void; }>(({ open, onClose }) => {
  const tournament = useTournament();

  if (!tournament) return null;

  return (
    <Modal title="Tournament" open={open} onClose={onClose}>
      <TournamentUserComponent />
      <TournamentMetaComponent />

      <ModalFooterPortal>
        <ButtonComponent variant="naked" onClick={onClose}>Close</ButtonComponent>
        <ButtonComponent href={`/tournaments/${tournament.data.id.toText()}`}>
          View
        </ButtonComponent>
      </ModalFooterPortal>

    </Modal>
  );
});
TournamentModal.displayName = 'TournamentModal';

export const TableScreenTournamentModalButton = memo(() => {
  const tournament = useTournament();

  const [showTournamentModal, setShowTournamentModal] = useState(false);
  const nextRebalanceTime = useNextMultitableRebalanceDateTime(tournament?.data, tournament?.actor);

  if (!tournament) return null;

  return (
    <>
      <TournamentModal open={showTournamentModal} onClose={() => setShowTournamentModal(false)} />
      {nextRebalanceTime && <RebalanceSpan nextRebalanceTime={nextRebalanceTime} />}
      <PillComponent onClick={() => setShowTournamentModal(true)}>
        Tournament
      </PillComponent>
    </>
  );
});
TableScreenTournamentModalButton.displayName = 'TableScreenTournamentModalButton';
