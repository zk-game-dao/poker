import type { Meta, StoryObj } from "@storybook/react";

import { ComponentProps } from "react";

import { Card } from "@declarations/table_canister/table_canister.did";

import { CommunityCardsComponent } from "./community-cards.component";

const CARDS: Card[] = [
  {
    value: { Six: null },
    suit: { Spade: null },
  },
  {
    value: { Seven: null },
    suit: { Spade: null },
  },
  {
    value: { Eight: null },
    suit: { Spade: null },
  },
  {
    value: { Nine: null },
    suit: { Spade: null },
  },
  {
    value: { Ten: null },
    suit: { Spade: null },
  },
];

type CustomArgs = ComponentProps<typeof CommunityCardsComponent> & {
  playedCards: number;
};

const meta: Meta<CustomArgs> = {
  title: "Table/Community Cards",
  // component: CommunityCardsComponent,
  args: {
    playedCards: 0,
  },
  render: ({ playedCards, ...props }) => (
    <CommunityCardsComponent
      {...props}
      community_cards={CARDS.slice(0, playedCards)}
    />
  ),
};

export default meta;

type Story = StoryObj<CustomArgs>;

export const CommunityCards: Story = {
  args: {
    community_cards: [],
  },
};

export const CommunityCards3Cards: Story = {
  args: {
    playedCards: 3,
  },
};

export const CommunityCardsAllCards: Story = {
  args: {
    playedCards: 5,
  },
};
