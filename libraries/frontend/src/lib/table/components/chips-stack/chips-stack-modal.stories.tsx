import type { Meta, StoryObj } from "@storybook/react";

import { FloatToTokenAmount } from "@/src/lib/utils/token-amount-conversion";

import { ChipsStackModalComponent } from "./chips-stack.component";

const _meta = { decimals: 8, thousands: 10 ** 8, transactionFee: 10_000n };

const meta: Meta<typeof ChipsStackModalComponent> = {
  title: "Table/Chips Stack/Modal",
  component: ChipsStackModalComponent,
  args: {
    value: FloatToTokenAmount(1234.3453, _meta),
    name: "Hello world",
    open: true,
  },
};

export default meta;

type Story = StoryObj<typeof ChipsStackModalComponent>;

export const Modal: Story = {};
