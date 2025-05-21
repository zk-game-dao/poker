import type { Meta, StoryObj } from "@storybook/react";

import { JoinTableModalComponent } from '../join-table-modal.component';

const meta: Meta<typeof JoinTableModalComponent> = {
  title: "Table/JoinTableModal",
  component: JoinTableModalComponent,
  args: {
    show: true,
    onClose: () => console.log("Modal closed"),
    error: undefined,
    minBuyIn: BigInt(100),
    maxBuyIn: BigInt(1000),
    buyIn: BigInt(500),
    setBuyIn: (buyIn: bigint) => console.log(`Buy-in set to: ${buyIn}`),
    transactionFee: BigInt(10),
    currencyType: { Real: { BTC: null } },
    hasTable: true,
    isPending: false,
    mutate: () => console.log("Mutation triggered"),
  }
};

export default meta;

type Story = StoryObj<typeof JoinTableModalComponent>;

export const Default: Story = {};
