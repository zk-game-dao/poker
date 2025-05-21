import type { Meta, StoryObj } from "@storybook/react";

import { LobbyPage } from "./lobby.page";

const meta: Meta<typeof LobbyPage> = {
  title: "Pages/Lobby",
  component: LobbyPage,
  parameters: {
    layout: "fullscreen",
  },
};

export default meta;

type Story = StoryObj<typeof LobbyPage>;

export const Lobby: Story = {};
