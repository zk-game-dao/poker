
import { mockPrincipal } from '../../../../utils/mock';
import { LeaderboardData, TournamentLeaderboardComponent } from '../tournament-leaderboard.component';

const mockLeaderboardData = (playerCount = 5, offset = 0, baseChips?: bigint, isRefetching = false): LeaderboardData => ({
  isRefetching,
  refetch: () => Promise.resolve(),
  data: Array.from({ length: playerCount }, (_, i) => ({
    user_id: mockPrincipal(i + offset),
    chips: baseChips === undefined ? undefined : baseChips * BigInt(playerCount - i),
    rank: i + offset,
  }))
})

const mockLeaderboards = (kickedPlayers = 5, activePlayers = 5, isRefetching?: boolean): {
  liveLeaderboard: LeaderboardData;
  leaderboard: LeaderboardData;
  isCompleted: boolean;
} => ({
  isCompleted: activePlayers === 0,
  liveLeaderboard: mockLeaderboardData(activePlayers, 0, 10000000000n, isRefetching),
  leaderboard: mockLeaderboardData(kickedPlayers, activePlayers + 1, undefined, isRefetching),
})

import type { Meta, StoryObj } from "@storybook/react";
const meta: Meta<typeof TournamentLeaderboardComponent> = {
  title: "Tournament/Leaderboard",
  component: TournamentLeaderboardComponent,
  args: {
    payoutStructure: [
      { position: 0, percentage: 50 },
      { position: 1, percentage: 30 },
      { position: 2, percentage: 20 },
    ],
    prizepool: 10000000000n,
    currencyType: { Real: { ICP: null } },
    tournamentUserId: mockPrincipal(0),
    ...mockLeaderboards(),
  }
};

export default meta;

type Story = StoryObj<typeof TournamentLeaderboardComponent>;

export const Ongoing: Story = {};
export const OngoingLoading: Story = {
  args: mockLeaderboards(8, 2, true),
};
export const Completed: Story = {
  args: mockLeaderboards(10, 0),
};
