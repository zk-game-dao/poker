import { memo, useMemo, useState } from 'react';

import { Principal } from '@dfinity/principal';
import { useUser } from '@lib/user';
import { CurrencyComponent } from '@zk-game-dao/currency';
import { List, ListItem } from '@zk-game-dao/ui';

import { useTournament } from '../context/tournament.context';
import { RefillModal } from './refill-modal.component';

export const TournamentRankingComponent = memo<{ principal: Principal; className?: string }>(({ principal, className }) => {
  const { user: zkpUser } = useUser();
  const tournament = useTournament();
  const ranking = useMemo(() => {
    if (!tournament) return;
    return tournament.data.current_players.find(([p]) => p.compareTo(principal) === 'eq');
  }, [tournament, principal]);

  const isSelf = useMemo(() => zkpUser && ranking && ranking[0].compareTo(zkpUser.principal_id) === 'eq', [ranking, zkpUser]);

  const [refillOpen, setRefillOpen] = useState(false);
  const addon = useMemo(() => tournament?.buyInOptions?.addon, [tournament]);

  if (!ranking) return null;

  return (
    <>
      {refillOpen && <RefillModal onClose={() => setRefillOpen(false)} />}
      <List className={className}>
        <ListItem rightLabel={ranking[1].position + 1}>
          {isSelf ? ' Your rank' : 'Rank'}
        </ListItem>
        <ListItem rightLabel={<CurrencyComponent currencyType={{ Fake: null }} currencyValue={ranking[1].chips} />}>
          {isSelf ? ' Your chips' : 'Chips'}
        </ListItem>
        {isSelf && addon?.enabled && tournament && (
          <ListItem
            rightLabel={<>For <CurrencyComponent currencyValue={addon.addon_price} currencyType={tournament.currencyType} /></>}
            onClick={() => setRefillOpen(true)}
          >
            Refill <CurrencyComponent className='ml-1' currencyType={{ Fake: null }} currencyValue={addon.addon_chips} />
          </ListItem>
        )}
        {isSelf && tournament?.user?.table && (
          <ListItem href={`/tournaments/${tournament.data.id.toText()}/my-table`}>
            Go to your table
          </ListItem>
        )}
      </List>
    </>
  );
});
TournamentRankingComponent.displayName = 'TournamentRankingComponent';
