import { memo, useMemo } from 'react';
import { Outlet, useParams } from 'react-router-dom';

import { Principal } from '@dfinity/principal';
import { ErrorComponent, LayoutComponent } from '@zk-game-dao/ui';

import { ProvideTournamentContext } from '../context/tournament.context';

export const TournamentContextLayout = memo(() => {
  const { tournamentId } = useParams<{ tournamentId: string }>();

  const tournament_principal = useMemo(
    () => tournamentId ? Principal.fromText(tournamentId) : undefined,
    [tournamentId],
  );

  if (!tournament_principal)
    return (
      <LayoutComponent footer>
        <ErrorComponent error="Tournament principal not found" />
      </LayoutComponent>
    );

  return (
    <ProvideTournamentContext id={tournament_principal}>
      <Outlet />
    </ProvideTournamentContext>
  );
});
TournamentContextLayout.displayName = 'TournamentContextLayout';

export default TournamentContextLayout;
