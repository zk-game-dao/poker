import { LayoutProvider } from '@zk-game-dao/ui';

import { AboutUsPage } from './about-us.page';

import type { Meta, StoryObj } from "@storybook/react";

const meta: Meta<typeof AboutUsPage> = {
  title: "Pages/AboutUs",
  component: AboutUsPage,
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

type Story = StoryObj<typeof AboutUsPage>;

export const Home: Story = {};
