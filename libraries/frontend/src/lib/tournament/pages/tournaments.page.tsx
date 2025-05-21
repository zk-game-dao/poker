import { TournamentData } from '@declarations/tournament_canister/tournament_canister.did';
import { NotificationIndicatorComponent } from '@lib/ui/notification-indicator.component';
import {
  DropdownComponent,
  ErrorComponent,
  Image,
  LayoutComponent,
  LoadingAnimationComponent,
  PillComponent,
  SmallHeroComponent,
  TabsComponent,
} from '@zk-game-dao/ui';
import { AnimatePresence } from 'framer-motion';
import { memo, useMemo, useState } from 'react';

import { CreateTournamentModalComponent } from '../components/create-tournament-modal/create-tournament-modal.component';
import { TournamentCardComponent } from '../components/tournament-card/tournament-card.component';
import {
  isTournamentConsideredActive,
  TournamentTypeFilter,
  TournamentTypes,
  useGetAllTournaments,
} from '../hooks/get-tournaments';

const FilterToType = (filter: TournamentTypeFilter): "BuyIn" | "Freeroll" | "SitAndGo" | "SpinAndGo" | undefined => {
  switch (filter) {
    case TournamentTypeFilter.BuyIn:
      return 'BuyIn';
    case TournamentTypeFilter.Freeroll:
      return 'Freeroll';
    case TournamentTypeFilter.SitAndGo:
      return 'SitAndGo';
    case TournamentTypeFilter.SpinAndGo:
      return 'SpinAndGo';
  }
}

const TournamentLabel = memo<{ label: string, locked?: boolean; tournaments: TournamentData[] }>(({
  label,
  locked,
  tournaments
}) => {

  const hasActiveTournaments = useMemo(() => tournaments.some(isTournamentConsideredActive), [tournaments]);

  return (
    <div className='relative flex flex-row items-center'>
      {locked && <Image alt="locked" src="/icons/🔒.png" type="png" width={17} height={26} className='mr-1 -mt-1' />}
      {label}
      <AnimatePresence>
        {hasActiveTournaments && <NotificationIndicatorComponent />}
      </AnimatePresence>
    </div>
  )
});
TournamentLabel.displayName = "TournamentLabel";

export const TournamentsPage = memo(() => {
  const [isCreateTableModalOpen, setIsCreateTableModalOpen] = useState(false);
  const [selectedTournamentType, setSelectedTournamentType] = useState(TournamentTypes[0].value);
  const selectedTournamentLabel = useMemo(() => {
    switch (selectedTournamentType) {
      case TournamentTypeFilter.BuyIn:
        return 'Buy-in';
      case TournamentTypeFilter.Freeroll:
        return 'Freeroll';
      case TournamentTypeFilter.SitAndGo:
        return 'Sit & Go';
      case TournamentTypeFilter.SpinAndGo:
        return 'Spin & Go';
    }
  }, [selectedTournamentType]);

  const allTournaments = useGetAllTournaments();

  const { tournaments, error, isPending } = useMemo(() => allTournaments[selectedTournamentType], [allTournaments, selectedTournamentType]);

  const resultMeta = useMemo(() => {
    if (error)
      return (
        <ErrorComponent
          error={error}
          className="m-auto"
          title="Failed to fetch tournaments"
        />
      );
    if (isPending)
      return (
        <LoadingAnimationComponent className="m-auto">
          Fetching tables
        </LoadingAnimationComponent>
      );
    if (!tournaments.length)
      return (
        <SmallHeroComponent
          icon={{
            type: "png",
            src: "/icons/table.png",
            width: 64,
            height: 64,
            alt: "Table",
          }}
          title="There are no active tournaments."
          subtitle="Create a table or wait for a table to spawn."
        />
      );
  }, [tournaments, isPending, error]);

  return (
    <LayoutComponent footer>
      <div className="hidden lg:flex container mx-auto w-full mb-4 lg:mb-6">
        <TabsComponent
          className='w-full'
          onChange={(value) => setSelectedTournamentType(value)}
          tabs={allTournaments.map(({
            meta: {
              label,
              locked,
              value
            },
            tournaments
          }) => ({
            label: (
              <TournamentLabel
                label={label}
                locked={locked}
                tournaments={tournaments}
              />
            ),
            value,
            disabled: locked,
          }))}
          value={selectedTournamentType}
        />
      </div>

      <div className="container mx-auto flex flex-row justify-start gap-4 items-center relative mb-4 lg:mb-6">
        <DropdownComponent
          className='lg:hidden'
          options={TournamentTypes.filter(v => !v.locked).map(({ label, value }) => ({ label: <>{label}<span className='hidden sm:flex'>{' tournaments'}</span></>, value }))}
          value={selectedTournamentType}
          onChange={(value) => setSelectedTournamentType(typeof value === 'number' ? value : 0)}
        />
        <div className="flex flex-1" />
        {selectedTournamentType !== TournamentTypeFilter.SpinAndGo && (
          <PillComponent
            size="large"
            className="relative z-1"
            onClick={() => setIsCreateTableModalOpen(true)}
          >
            Create {selectedTournamentLabel} tournament
          </PillComponent>
        )}
      </div>
      {resultMeta}

      <div className="grid grid-cols-1 gap-4 lg:grid-cols-2 xl:grid-cols-3 container mx-auto">
        {tournaments?.map((tournament) => (
          <TournamentCardComponent key={tournament.id.toText()} {...tournament} />
        ))}
      </div>

      {selectedTournamentType !== TournamentTypeFilter.SpinAndGo && (
        <CreateTournamentModalComponent
          open={isCreateTableModalOpen}
          key={selectedTournamentType}
          initialType={FilterToType(selectedTournamentType) ?? 'BuyIn'}
          onCancel={() => setIsCreateTableModalOpen(false)}
        />
      )}
    </LayoutComponent >
  );
});
TournamentsPage.displayName = "TournamentsPage";

export default TournamentsPage;
