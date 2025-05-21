import type { Meta, StoryObj } from "@storybook/react";

import { CardColor, TableColor } from '@/src/models/table-color.model';
import { FloatToTokenAmount } from '@lib/utils/token-amount-conversion';

import { ChipsStackComponent } from '../../chips-stack/chips-stack.component';
import { TableBackgroundComponent } from './table-background.component';

const meta: Meta<typeof TableBackgroundComponent> = {
  title: "Table/Background",
  component: TableBackgroundComponent,
};

const _meta = { decimals: 8, thousands: 10 ** 8, transactionFee: 10_000n };

export default meta;

type Story = StoryObj<typeof TableBackgroundComponent>;

export const Background: Story = {
  args: {
    children: (
      <ChipsStackComponent
        currencyType={{ Real: { ICP: null } }}
        value={FloatToTokenAmount(1234.1234, _meta)}
      />
    ),
    className: "w-[1400px] aspect-1040/600",
    visuals: { color: TableColor.Green, cardColor: CardColor.Red },
    community_cards: [],
  },
};

export const BackgroundSmall: Story = {
  args: {
    children: (
      <ChipsStackComponent
        currencyType={{ Real: { ICP: null } }}
        value={FloatToTokenAmount(1234.1234, _meta)}
      />
    ),
    community_cards: [],
    className: "w-64 aspect-1040/600",
  },
};

export const BackgroundBlueMedium: Story = {
  args: {
    children: (
      <ChipsStackComponent
        currencyType={{ Real: { ICP: null } }}
        value={FloatToTokenAmount(1234.1234, _meta)}
      />
    ),
    community_cards: [],
    className: "w-96 aspect-1040/600",
    visuals: { color: TableColor.Blue, cardColor: CardColor.Red },
  },
};

export const BackgroundBlackLarge: Story = {
  args: {
    children: (
      <ChipsStackComponent
        currencyType={{ Real: { ICP: null } }}
        value={FloatToTokenAmount(1234.1234, _meta)}
      />
    ),
    community_cards: [],
    className: "w-[1024px] aspect-1040/600",
    visuals: { color: TableColor.Black, cardColor: CardColor.Red },
  },
};

export const BackgroundPurple: Story = {
  args: {
    children: (
      <ChipsStackComponent
        currencyType={{ Real: { ICP: null } }}
        value={FloatToTokenAmount(1234.1234, _meta)}
      />
    ),
    community_cards: [],
    className: "w-[420px] aspect-1040/600",
    visuals: { color: TableColor.Purple, cardColor: CardColor.Red },
  },
};

export const BackgroundBlueScreen: Story = {
  args: {
    children: (
      <ChipsStackComponent
        currencyType={{ Real: { ICP: null } }}
        value={FloatToTokenAmount(1234.1234, _meta)}
      />
    ),
    community_cards: [],
    className: "absolute left-0 top-0 w-screen h-screen",
    visuals: { color: TableColor.Blue, cardColor: CardColor.Red },
  },
};

export const GreenTable: Story = {
  args: {
    className: "w-[420px] aspect-1040/600",
    visuals: { color: TableColor.Green, cardColor: CardColor.Red },
  },
};
export const RedTable: Story = {
  args: {
    className: "w-[420px] aspect-1040/600",
    visuals: { color: TableColor.Red, cardColor: CardColor.Red },
  },
};
export const BlueTable: Story = {
  args: {
    className: "w-[420px] aspect-1040/600",
    visuals: { color: TableColor.Blue, cardColor: CardColor.Red },
  },
};
export const PurpleTable: Story = {
  args: {
    className: "w-[420px] aspect-1040/600",
    visuals: { color: TableColor.Purple, cardColor: CardColor.Red },
  },
};
export const YellowTable: Story = {
  args: {
    className: "w-[420px] aspect-1040/600",
    visuals: { color: TableColor.Yellow, cardColor: CardColor.Red },
  },
};
export const BlackTable: Story = {
  args: {
    className: "w-[420px] aspect-1040/600",
    visuals: { color: TableColor.Black, cardColor: CardColor.Red },
  },
};
export const BlackTableVerySmall: Story = {
  decorators: [
    (Story) => (
      <div className="w-[200px] aspect-1040/600 flex">
        <Story />
      </div>
    ),
  ],
  args: {
    visuals: { color: TableColor.Black, cardColor: CardColor.Red },
  },
};
export const BlackTableVeryMidSmall: Story = {
  decorators: [
    (Story) => (
      <div className="w-[300px] aspect-1040/600 flex">
        <Story />
      </div>
    ),
  ],
  args: {
    visuals: { color: TableColor.Black, cardColor: CardColor.Red },
  },
};
