import { LoadingAnimationComponent } from '@zk-game-dao/ui';
import { memo } from 'react';

import { TablePageWithPrincipal } from '../../../pages/table/table.page';
import { useUser } from '../../user';
import { useTournament } from '../context/tournament.context';
import { Redirect } from '../../../components/common/redirect.component';

export const TournamentMyTablePage = memo(() => {
  const { data, user } = useTournament(true);
  const { user: zkpUser } = useUser()

  if ('Cancelled' in data.state) return <Redirect to={`/tournaments/${data.id}`} />;
  if ('Completed' in data.state) return <Redirect to={`/tournaments/${data.id}/leaderboard`} />;

  if (!zkpUser || !user?.table) return <LoadingAnimationComponent />;

  return <TablePageWithPrincipal table_principal={user.table.id} />;
});
TournamentMyTablePage.displayName = 'TournamentMyTablePage';

export default TournamentMyTablePage;
