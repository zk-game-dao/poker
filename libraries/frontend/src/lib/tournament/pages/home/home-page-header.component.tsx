import { ButtonComponent, List, ListItem } from '@zk-game-dao/ui';
import { format } from 'date-fns-tz';
import { memo, useState } from 'react';

import { useUser } from '../../../user';
import { matchRustEnum } from '../../../utils/rust';
import { BigIntTimestampToDate } from '../../../utils/time';
import { PricePool } from '../../components/prize-pool.component';
import { QuickJoinModal } from '../../components/quick-join-modal.component';
import { TournamentUserComponent } from '../../components/tournament-user.component';
import { useTournament } from '../../context/tournament.context';


const dateTimeFormat = `dd.MM.yyyy HH:mm`;
const fmt = (dtn: bigint) => format(BigIntTimestampToDate(dtn), dateTimeFormat);

export const TournamentPageHeader = memo(() => {
  const { data, user, currencyType, isRunning } = useTournament(true);
  const { user: zkpUser } = useUser();
  const [quickJoinModalOpen, setQuickJoinModalOpen] = useState(false);

  return (
    <div className='flex flex-row gap-4 justify-start overflow-hidden items-center relative'>
      <img
        src={data.hero_picture}
        className=' w-[128px] h-[128px] rounded-[12px] object-cover grow-0 shrink flex overflow-hidden '
      />
      <div className='flex flex-col relative z-1'>
        <p className=' type-callout text-material-heavy-1 '>
          {matchRustEnum(data.tournament_type)({
            BuyIn: () => 'Buy-in',
            SitAndGo: () => 'Sit and Go',
            Freeroll: () => 'Freeroll',
            SpinAndGo: () => 'Spin and Go',
          })}
          {' Tournament'}
        </p>

        <h1 className="type-top">{data.name}</h1>
        <p className="type-body text-neutral-200/70">{data.description}</p>
        <PricePool hideOnMobile />
      </div>
      {user && <TournamentUserComponent />}
      {!!zkpUser && !user && data.max_players - data.current_players.length > 0 && !isRunning && (
        <div className='flex flex-col gap-4'>
          <QuickJoinModal
            {...data}
            open={quickJoinModalOpen}
            currencyType={currencyType}
            onClose={() => setQuickJoinModalOpen(false)}
          />
          <List>
            <ListItem rightLabel={`${data.max_players - data.current_players.length}/${data.max_players}`}>
              Available spots
            </ListItem>
            <ListItem rightLabel={fmt(data.start_time)}>
              Start Time
            </ListItem>
          </List>
          <ButtonComponent onClick={() => setQuickJoinModalOpen(true)}>
            Join Tournament
          </ButtonComponent>
        </div>
      )
      }
    </div >
  );
});
TournamentPageHeader.displayName = 'TournamentPageHeader';
