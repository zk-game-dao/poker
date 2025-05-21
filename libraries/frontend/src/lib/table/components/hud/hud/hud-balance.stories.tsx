import type { Meta, StoryObj } from "@storybook/react";

import { HudBalanceComponent } from '../hud-balance.component';

const meta: Meta<typeof HudBalanceComponent> = {
  title: "Table/HUD/Balance",
  component: HudBalanceComponent,
  args: {
    balance: 100_000_000n,
    currencyType: {
      Real: { ICP: null },
    }
  },
  globals: {
    viewport: 'mobile1',
  },
};

export default meta;

type Story = StoryObj<typeof HudBalanceComponent>;

export const HUDQuickActions: Story = {};
