import { memo } from 'react';

import { LobbyTableCardComponent } from '../../../../pages/lobby/lobby-table-card.component';
import { PublicTable } from '@declarations/table_canister/table_canister.did';

export const TournamentTablesListComponent = memo<{ tables: PublicTable[] }>(({ tables }) => (
  <div className="container mx-auto grid gap-4 grid-cols-1 lg:grid-cols-2">
    {tables.map((table, i) => (
      <LobbyTableCardComponent
        key={table.id.toText()}
        {...table}
        index={i}
      />
    ))}
  </div>
));
TournamentTablesListComponent.displayName = 'TournamentTablesPage';
