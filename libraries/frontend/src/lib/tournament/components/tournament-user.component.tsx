import { memo } from 'react';

import { useTournament } from '../context/tournament.context';
import { TournamentRankingComponent } from './tournament-ranking.component';

export const TournamentUserComponent = memo(() => {
  const tournament = useTournament();

  if (!tournament?.user) return null;

  return (
    <div className='flex flex-col'>
      <TournamentRankingComponent className='min-w-[300px]' principal={tournament.user.principal} />
    </div>
  );
});
TournamentUserComponent.displayName = 'TournamentUserComponent';
