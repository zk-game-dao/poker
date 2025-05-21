import { memo } from 'react';

import { TitleTextComponent } from '@zk-game-dao/ui';

import { useTournament } from '../../context/tournament.context';
import { TournamentTablesListComponent } from './tournament-tables-list.component';

export const TournamentTablesPage = memo(() => {
  const { tables } = useTournament(true);

  if (tables.length === 0)
    return <TitleTextComponent title="No tables" text="No tables have been created yet, wait for the tournament to start" />;

  return <TournamentTablesListComponent tables={tables} />;
});
TournamentTablesPage.displayName = 'TournamentTablesPage';

export default TournamentTablesPage;
