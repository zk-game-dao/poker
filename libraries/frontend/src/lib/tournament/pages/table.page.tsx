import { Principal } from '@dfinity/principal';
import { memo } from 'react';
import { useParams } from 'react-router-dom';

import { Redirect } from '../../../components/common/redirect.component';
import { TablePageWithPrincipal } from '../../../pages/table/table.page';
import { useTournament } from '../context/tournament.context';

export const TournamentTablePage = memo(() => {
  const { user, data } = useTournament(true);
  const { tableId } = useParams<{ tableId: string }>();

  if ('Completed' in data.state)
    return <Redirect to={`/tournaments/${data.id}/leaderboard`} />;
  if ('Cancelled' in data.state)
    return <Redirect to={`/tournaments/${data.id}`} />;

  if (user?.table) return <Redirect to={`/tournaments/${data.id}/my-table`} />;
  if (!tableId) return <Redirect to={`/tournaments/${data.id}/tables`} />;

  return <TablePageWithPrincipal table_principal={Principal.fromText(tableId)} />;
});
TournamentTablePage.displayName = 'TournamentTablePage';

export default TournamentTablePage;
