import type { Meta, StoryObj } from "@storybook/react";

import { LobbyFilterModalComponent } from "./lobby-filter-modal.component";

const meta: Meta<typeof LobbyFilterModalComponent> = {
  title: "Pages/Lobby/FilterModal",
  component: LobbyFilterModalComponent,
  args: {
    options: {
      currency_type: [],
      seats: [],
      game_type: [],
      timer_duration: [],
      exclude_timer_duration: [],
      exclude_game_type: [],
      exclude_currency_type: [],
      exclude_seats: []
    },
  },
};

export default meta;

type Story = StoryObj<typeof LobbyFilterModalComponent>;

export const TableCard: Story = {};
