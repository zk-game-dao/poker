
import { mockPrincipal } from '../../../../utils/mock';
import { TournamentLeaderboardEntry } from '../tournament-leaderboard-entry.component';

import type { Meta, StoryObj } from "@storybook/react";
const meta: Meta<typeof TournamentLeaderboardEntry> = {
  title: "Tournament/Leaderboard/Entry",
  component: TournamentLeaderboardEntry,
  args: {
    user_id: mockPrincipal(0),
    rank: 0,
    currencyType: { Real: { ICP: null } }
  }
};

export default meta;

type Story = StoryObj<typeof TournamentLeaderboardEntry>;

export const Ongoing: Story = {
  args: {
    chips: 100000n,
    winnings: undefined,
    isSelf: false,
    isCompleted: false,
  }
};

export const OngoingSelf: Story = {
  args: {
    chips: 100000n,
    winnings: undefined,
    isSelf: true,
    isCompleted: false,
  }
};

export const FinishedSelf: Story = {
  args: {
    winnings: 10000000000n,
    isSelf: true,
    isCompleted: true,
  }
};