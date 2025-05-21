import type { Meta, StoryObj } from "@storybook/react";

import { CardColor, TableColor } from '@/src/models/table-color.model';

import { TableVisualsContext } from '../../context/table-visuals.context';
import { CardComponent } from './card.component';

const meta: Meta<typeof CardComponent> = {
  title: "Table/Card",
  component: CardComponent,
  args: {
    card: {
      value: { Six: null },
      suit: { Spade: null },
    },
  },
  argTypes: {
    card: {
      control: {
        type: "object",
      },
    },
    size: {
      options: [
        "microscopic",
        "between-microscopic-and-small",
        "small",
        "medium",
        "large",
      ],
      control: {
        type: "select",
      },
    },
  },
};

export default meta;

type Story = StoryObj<typeof CardComponent>;

export const CardMicroscopic: Story = { args: { size: "microscopic" } };
export const CardBetweenMicroscopicAndSmall: Story = {
  args: { size: "between-microscopic-and-small" },
};
export const CardSmall: Story = { args: { size: "small" } };
export const CardMedium: Story = { args: { size: "medium" } };
export const CardLarge: Story = { args: { size: "large" } };

export const GreenTableCardBack: Story = {
  args: {
    card: undefined,
  },
  decorators: [
    (Story) => (
      <TableVisualsContext.Provider
        value={{ color: TableColor.Green, cardColor: CardColor.Red }}
      >
        <Story />
      </TableVisualsContext.Provider>
    ),
  ],
};

export const RedTableCardBack: Story = {
  args: {
    card: undefined,
  },
  decorators: [
    (Story) => (
      <TableVisualsContext.Provider
        value={{ color: TableColor.Red, cardColor: CardColor.Green }}
      >
        <Story />
      </TableVisualsContext.Provider>
    ),
  ],
};
