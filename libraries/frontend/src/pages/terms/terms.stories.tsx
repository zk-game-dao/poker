import { LayoutProvider } from '@zk-game-dao/ui';

import { TermsPage } from './terms.page';

import type { Meta, StoryObj } from "@storybook/react";

const meta: Meta<typeof TermsPage> = {
  title: "Pages/Terms",
  component: TermsPage,
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

type Story = StoryObj<typeof TermsPage>;

export const Terms: Story = {};
