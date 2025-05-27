import classNames from 'classnames';
import { memo, useMemo, useState } from 'react';

import { users_index } from '@declarations/users_index';
import { Queries } from '@lib/data';
import {
  CreateTableModalComponent
} from '@lib/table/components/create-table-modal/create-table-modal.component';
import { useQuery } from '@tanstack/react-query';
import {
  CurrencyToString, TokenAmountToString, useCurrencyManagerMeta, useIsBTC
} from '@zk-game-dao/currency';
import {
  ErrorComponent, Interactable, LayoutComponent, List, LoadingAnimationComponent, PaginationComponent,
  TabsComponent,
  useRouting
} from '@zk-game-dao/ui';

import { useWording } from '../../hooks/wording';
import { callActorMutation } from '../../lib/utils/call-actor-mutation';
import { useJackpot } from './data';
import { LeaderboardEntry } from './leaderboard-entry.component';
import { HowItWorksModal } from './leaderboard-how-it-works-modal.component';
import { useNavigate, useNavigation, useParams, useSearchParams } from 'react-router-dom';
import { useUser } from '@/src/lib/user';

const ENDPOINTS = {
  verified: {
    get_length: 'get_verified_experience_points_leaderboard_length' as const,
    zkp: 'get_verified_experience_points_leaderboard' as const,
    pp: 'get_verified_pure_poker_experience_points' as const,
  },
  all: {
    get_length: 'get_leaderboard_length' as const,
    zkp: 'get_experience_points_leaderboard' as const,
    pp: 'get_pure_poker_experience_points' as const,
  },
};

type LeaderboardType = keyof typeof ENDPOINTS;

export const LeaderboardPage = memo(() => {
  // const [page, setPage] = useState(0n);
  const pageSize = useMemo(() => 50n, []);
  const { show, user } = useUser();
  const isBTC = useIsBTC();

  const [params, setParams] = useSearchParams();

  const page = useMemo(() => {
    const pageParam = params.get('page');
    return pageParam ? BigInt(pageParam) : 0n;
  }, [params]);
  const setPage = (newPage: bigint) => setParams({ page: newPage.toString() });

  const { type = 'verified' } = useParams<{ type?: LeaderboardType }>()

  const leaderboardSize = useQuery({
    queryKey: Queries.leaderboardSize.key(type),
    queryFn: () => callActorMutation(users_index, ENDPOINTS[type].get_length),
    retry: false,
    initialData: 0n,
  });

  const leaderboardPrincipals = useQuery({
    queryKey: Queries.leaderboard.key(type, page, pageSize),
    queryFn: () =>
      callActorMutation(users_index,
        ENDPOINTS[type][isBTC ? 'pp' : 'zkp'],
        page,
        pageSize
      ),
    retry: false,
  });

  const totalPages = useMemo(() => Math.ceil(Number(leaderboardSize.data) / Number(pageSize)), [leaderboardSize.data, pageSize]);

  const [isModalOpen, setModalOpen] = useState(false);
  const [isHowItWorksOpen, setHowItWorksOpen] = useState(false);
  const wording = useWording();
  const { currency, jackpots } = useJackpot();
  const jackpot = useMemo(() => Object.values(jackpots).flatMap(v => v).reduce((a, b) => a + b, 0n), [Object.values(jackpots)]);
  const meta = useCurrencyManagerMeta({ Real: currency });


  return (
    <LayoutComponent
      footer
      hero={{
        title: 'Leaderboards',
        subTitle: (
          <>
            Track your progress and see how you stack up against other players in {wording.product}'s XP Rewards System.
            <br />
            {user && <>
            <Interactable
              className='inline underline hover:no-underline text-material-heavy-2 '
              onClick={show}
            >Verify your account</Interactable>{' and compete '}</>}
            Compete for a spot in the Top 5 to claim your share of the {TokenAmountToString(jackpot, meta)} {CurrencyToString(currency)} weekly prize pool!
            <div className='type-body mt-4'>
              You are currently {type === 'all' ? 'seeing all users' : 'only seeing verified users'}.
              <Interactable className='underline hover:no-underline ml-1' href={`/leaderboard/${type === 'all' ? 'verified' : 'all'}`}>
                {type === 'all' ? 'Show verified users' : 'Show all users'}
              </Interactable>
            </div>
          </>
        ),
        ctas: [
          { children: 'How it works', onClick: () => setHowItWorksOpen(true) },
          { children: 'Go to lobby', href: '/', filled: true },
        ],
      }}
    >
      <div className="container mx-auto flex flex-col gap-4">
        {leaderboardPrincipals.isPending && <LoadingAnimationComponent variant="spinner" />}
        <ErrorComponent error={leaderboardSize.error || leaderboardPrincipals.error} />
        {typeof leaderboardPrincipals.data !== 'undefined' && Array.isArray(leaderboardPrincipals.data) && (
          <div className='flex flex-col justify-center items-center mx-auto w-full max-w-[650px]'>
            <List
              label="Leaderboard"
              className={classNames('mx-auto')}
              ctas={[{
                label: 'Refresh', onClick: () => {
                  leaderboardPrincipals.refetch();
                  leaderboardSize.refetch();
                }
              }]}
            >
              {leaderboardPrincipals.data.map(([canister_id, experience_points], i) => (
                <LeaderboardEntry
                  key={canister_id.toText()}
                  experience_points={experience_points}
                  user_id={canister_id}
                  rank={Number(page * pageSize) + i}
                />
              ))}
            </List>
          </div>
        )}

        {totalPages > 1 && (
          <PaginationComponent
            currentPage={Number(page)}
            totalPages={Number(totalPages)}
            onPageChange={(page) => setPage(BigInt(page))}
          />
        )}

        <HowItWorksModal
          isOpen={isHowItWorksOpen}
          onClose={() => setHowItWorksOpen(false)}
        />

        <CreateTableModalComponent
          open={isModalOpen}
          onCancel={() => setModalOpen(false)}
        />
      </div>
    </LayoutComponent>
  );
});
LeaderboardPage.displayName = 'LeaderboardPage';

export default LeaderboardPage;
