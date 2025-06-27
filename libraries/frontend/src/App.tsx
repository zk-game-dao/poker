import { ProvideQuery } from '@lib/data';
import { ErrorBoundaryComponent, LoadingSpinnerComponent } from '@zk-game-dao/ui';
import { MotionConfig } from 'framer-motion';
import { lazy, memo, Suspense, useMemo } from 'react';
import { createBrowserRouter, LoaderFunction, redirect, RouteObject, RouterProvider } from 'react-router-dom';

import { ThemeContextType } from './context/platform-theme.context';
import { TournamentsRouter } from './lib/tournament/pages/tournaments.router';
import { RootComponent } from './root.component';

BigInt.prototype.toJSON = function () {
  return this.toString();
};

const ContactPage = lazy(() => import("./pages/contact/contact.page"));
const HouseRulesPage = lazy(
  () => import("./pages/house-rules/house-rules.page"),
);
const LobbyPage = lazy(() => import("./pages/lobby/lobby.page"));
const StorePage = lazy(() => import("./pages/store.page"));
const TablePage = lazy(() => import("./pages/table/table.page"));
const ChangelogPage = lazy(() => import("./pages/changelog.page"));
const RoadmapPage = lazy(() => import("./pages/roadmap.page"));
const LeaderboardPage = lazy(() => import("./pages/leaderboard/leaderboard.page"));
const RakePage = lazy(() => import("./pages/rake/rake.page"));
const BecomeHostPage = lazy(() => import("./pages/become-host/become-host.page"));

const redirectLoader =
  (to: string): LoaderFunction =>
    () =>
      redirect(to);

const BuildRouter = (theme: Omit<ThemeContextType, 'setShownCurrencyType'>) => {
  const children: RouteObject[] = [
    { path: "", element: theme.homePage },
    { path: "rules", element: <HouseRulesPage /> },
    { path: "contact", element: <ContactPage /> },
    { path: "store", element: <StorePage /> },
    { path: "cash-games", element: <LobbyPage /> },
    { path: "changelog", element: <ChangelogPage markdown={theme.changelogMarkdown} /> },
    { path: "roadmap", element: <RoadmapPage markdown={theme.roadmapMarkdown} /> },
    { path: 'become-host', element: <BecomeHostPage markdown={theme.becomeAHostMarkdown} /> },
    { path: "rake", element: <RakePage /> },
    { path: "terms", loader: redirectLoader("/rules") },
    { path: "lobby", loader: redirectLoader("/cash-games") },
    {
      path: "tables",
      children: [{ path: ":tableId", element: <TablePage /> }],
    },
    TournamentsRouter,
  ];

  if (!theme.isBTC)
    children.push({
      path: "leaderboard",
      children: [
        {
          path: "",
          loader: redirectLoader("/leaderboard/verified"),
        },
        {
          path: ":type",
          element: <LeaderboardPage />,
        }
      ]
    });

  return createBrowserRouter([
    {
      path: "/",
      errorElement: <ErrorBoundaryComponent />,
      element: <RootComponent {...theme} />,
      children: [
        ...children,
        { path: "*", loader: redirectLoader("/") },
      ],
    },
  ])
};

export const App = memo<Omit<ThemeContextType, 'setShownCurrencyType'>>(({
  isBTC,
  ...remainder
}) => {
  const theme = useMemo((): Omit<ThemeContextType, 'setShownCurrencyType'> => ({
    ...remainder,
    banner: {
      children: "We're launching zkGame DAO on SNS! Get involved.",
      href: "https://forum.dfinity.org/t/were-preparing-to-launch-zkgame-dao-on-the-sns/48128",
    },
    isBTC,
    shownCurrencyType: { Real: isBTC ? { BTC: null } : { ICP: null } },
  }), [isBTC, remainder]);

  const router = useMemo(() => BuildRouter(theme), [theme]);

  return (
    <Suspense fallback={<LoadingSpinnerComponent className='absolute inset-0' />}>
      <ProvideQuery>
        <MotionConfig
          transition={{
            type: "spring",
            stiffness: 200,
            damping: 22,
          }}
        >
          <RouterProvider router={router} />
        </MotionConfig>
      </ProvideQuery>
    </Suspense>
  )
});
App.displayName = 'App';

export default App;
