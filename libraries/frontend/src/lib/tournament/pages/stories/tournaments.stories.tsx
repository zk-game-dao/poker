import { LayoutProvider } from '@zk-game-dao/ui';

import { TournamentsPage } from '../tournaments.page';

import type { Meta, StoryObj } from "@storybook/react";

const meta: Meta<typeof TournamentsPage> = {
  title: "Pages/Tournaments",
  component: TournamentsPage,
  parameters: {
    layout: "fullscreen",
  },
  decorators: [
    (Story) => (
      <LayoutProvider>
        <Story />
      </LayoutProvider>
    ),
  ],
};

export default meta;

type Story = StoryObj<typeof TournamentsPage>;

export const Tournaments: Story = {};
