import { HomeIcon, NumberedListIcon, RectangleStackIcon } from '@heroicons/react/24/solid';
import { LayoutComponent, LoadingAnimationComponent } from '@zk-game-dao/ui';
import { memo } from 'react';
import { Outlet } from 'react-router-dom';

import { SidebarComponent } from '../../../components/common/sidebar/sidebar.component';
import { useTournament } from '../context/tournament.context';

export const TournamentLayout = memo(() => {
  const { tables, data, user } = useTournament(true);

  if (!data || !tables)
    return (
      <LayoutComponent footer>
        <LoadingAnimationComponent />
      </LayoutComponent>
    );

  return (
    <LayoutComponent footer>
      <div className='flex flex-row gap-4 md:gap-16 container w-full mx-auto'>
        <SidebarComponent
          items={[
            {
              type: 'link',
              title: 'Info',
              value: `/tournaments/${data.id.toText()}`,
              icon: <HomeIcon />,
            },
            {
              type: 'link',
              icon: <NumberedListIcon />,
              title: 'Leaderboard',
              value: `/tournaments/${data.id.toText()}/leaderboard`,
            },

            {
              type: 'link',
              title: 'Tables',
              value: `/tournaments/${data.id.toText()}/tables`,
              icon: <RectangleStackIcon />,
              hidden: tables.length < 2 || !!user?.table,
            },

            {
              type: 'link',
              title: 'Table',
              value: `/tournaments/${data.id.toText()}/table/${tables[0]?.id}`,
              icon: <RectangleStackIcon />,
              hidden: tables.length !== 1 || !!user?.table,
            },

            {
              type: 'link',
              title: 'My Table',
              value: `/tournaments/${data.id.toText()}/my-table`,
              icon: <RectangleStackIcon />,
              hidden: !user?.table,
            }
          ]}
        />
        <div className='flex flex-col w-full'>
          <Outlet />
        </div>
      </div >
    </LayoutComponent>
  );
});
TournamentLayout.displayName = 'TournamentLayout';

export default TournamentLayout;
