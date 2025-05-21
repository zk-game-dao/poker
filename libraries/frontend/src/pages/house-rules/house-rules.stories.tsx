import { LayoutProvider } from '@zk-game-dao/ui';

import { HouseRulesPage } from './house-rules.page';

import type { Meta, StoryObj } from "@storybook/react";

const meta: Meta<typeof HouseRulesPage> = {
  title: "Pages/HouseRules",
  component: HouseRulesPage,
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

type Story = StoryObj<typeof HouseRulesPage>;

export const HouseRules: Story = {};
