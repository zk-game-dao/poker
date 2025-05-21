import { lazy } from 'react';
import { RouteObject } from 'react-router-dom';

const TournamentsPage = lazy(() => import("./tournaments.page"));
const TournamentPage = lazy(() => import("./home/home.page"));
const TournamentLeaderboardPage = lazy(() => import("./leaderboard.page"));
const TournamentLayout = lazy(() => import("./layout"));
const TournamentMyTablePage = lazy(() => import("./my-table.page"));
const TournamentTablesPage = lazy(() => import("./tables/tables.page"));
const TournamentTablePage = lazy(() => import("./table.page"));
const TournamentContextLayout = lazy(() => import("./context-layout"));

export const TournamentsRouter: RouteObject = {
  path: "tournaments",
  children: [
    { path: "", element: <TournamentsPage /> },
    {
      path: ":tournamentId",
      element: <TournamentContextLayout />,
      children: [
        { path: "table/:tableId", element: <TournamentTablePage /> },
        { path: "my-table", element: <TournamentMyTablePage /> },
        {
          path: "",
          element: <TournamentLayout />,
          children: [
            { path: "", element: <TournamentPage /> },
            { path: "leaderboard", element: <TournamentLeaderboardPage /> },
            { path: "tables", element: <TournamentTablesPage /> },
          ]
        },
      ]
    },
  ]
};
