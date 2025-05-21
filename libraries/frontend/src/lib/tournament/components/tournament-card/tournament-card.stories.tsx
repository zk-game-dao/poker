import type { Meta, StoryObj } from "@storybook/react";

import { Principal } from '@dfinity/principal';

import { UserTournamentData } from '@declarations/tournament_index/tournament_index.did';
import { mockUserArray } from '../../../utils/mock';
import { TournamentCardComponent } from './tournament-card.component';

const meta: Meta<typeof TournamentCardComponent> = {
  title: "Tournament/TournamentCard",
  component: TournamentCardComponent,
  args: {
    name: "Tournament Name",
    buy_in: 10000000000n,
    currency: { Real: { ICP: null } },
    start_time: 1736692189067000000n,
    current_players: mockUserArray(10).map(({ user }) => [user.principal_id, {
      'chips': 0n,
      'position': 0,
      'rebuys': 0,
    }] as [Principal, UserTournamentData]),
    max_players: 100,
    starting_chips: 100000n,
    id: Principal.anonymous(),
  },
};

export default meta;

type Story = StoryObj<typeof TournamentCardComponent>;

export const TournamentCard: Story = {};
