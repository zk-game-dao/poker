import { FloatToTokenAmount } from '@/src/lib/utils/token-amount-conversion';
import { Meta, StoryObj } from '@storybook/react';

import { ChipsStackComponent } from './chips-stack.component';
import { CurrencyMeta } from '@zk-game-dao/currency';

const _meta: CurrencyMeta = { decimals: 8, thousands: 10 ** 8, transactionFee: 10_000n, symbol: "ICP", isFetched: true };

const meta: Meta<typeof ChipsStackComponent> = {
  title: "Table/Chips Stack",
  component: ChipsStackComponent,
  args: {
    value: FloatToTokenAmount(1234.3453, _meta),
    currencyType: { Real: { ICP: null } },
  },
  argTypes: {
    value: { control: { type: "number" } },
    // currencyType: {
    //   options: Currencies,
    //   control: {
    //     type: "select",
    //     defaultValue: "ICP",
    //   },
    // },
    name: { control: { type: "text" } },
    openable: { control: { type: "boolean" } },
  },
};

export default meta;

type Story = StoryObj<typeof ChipsStackComponent>;

export const ChipStack: Story = {};
// export const ChipStackckEth: Story = { args: { currency: "ckETH" } };
export const ChipStackckUSDC: Story = { args: { currencyType: { Real: { CKETHToken: { USDC: null } } } } };
export const ChipStackckUSDT: Story = { args: { currencyType: { Real: { CKETHToken: { USDT: null } } } } };
export const ChipStackNumber: Story = {
  args: { value: FloatToTokenAmount(1234.3453, _meta) },
};
export const ChipStack1MEuro: Story = {
  args: { value: FloatToTokenAmount(138026.22498274673, _meta) },
};
export const ChipStackReallyBigAmount: Story = {
  args: { value: FloatToTokenAmount(Number("1380026.22498274673"), _meta) },
};
export const ChipStackBuggyAmount: Story = {
  args: { value: FloatToTokenAmount(Number("13800926.22498274673"), _meta) },
};
export const ChipStackSmall: Story = {
  args: { value: FloatToTokenAmount(0.1, _meta) },
};