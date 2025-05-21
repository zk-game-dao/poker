import type { Meta, StoryObj } from "@storybook/react";

import { HUDQuickActionsComponent } from '../hud-quick-actions.component';

const meta: Meta<typeof HUDQuickActionsComponent> = {
  title: "Table/HUD/Quick Actions",
  component: HUDQuickActionsComponent,
  args: {
    quickActions: [
      [0n, 'Call'],
      [1n, 'Raise'],
      [2n, 'Fold'],
      [3n, 'Check'],
      [4n, 'All In'],
    ],
    mutate: async () => { }
  },
};

export default meta;

type Story = StoryObj<typeof HUDQuickActionsComponent>;

export const HUDQuickActions: Story = {};
