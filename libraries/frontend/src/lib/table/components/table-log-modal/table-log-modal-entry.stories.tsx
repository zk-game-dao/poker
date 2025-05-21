import type { Meta, StoryObj } from "@storybook/react";

import { ComponentProps } from 'react';

import { mockCumulativeUserTableData, mockTableContext, PREVIEW_USERS } from '@lib/utils/mock';
import { FloatToTokenAmount } from '@lib/utils/token-amount-conversion';

import { ProvideTable } from '../../context/table.context';
import { ActionLogComponent } from '../action-log/action-log.component';
import { TableModalLogEntryComponent } from './table-log-modal-entry.component';

const _meta = { decimals: 8, thousands: 10 ** 8, transactionFee: 10_000n };

declare global {
  interface BigInt {
    toJSON(): string;
  }
}

BigInt.prototype.toJSON = function () {
  return this.toString();
};

type LogArgs = ComponentProps<typeof ActionLogComponent> & {
  timestamp: string;
};

const meta: Meta<LogArgs> = {
  title: "Table/Log Modal/Entry",
  args: {
    timestamp: BigInt(Date.now()).toString(),
    user_principal: [PREVIEW_USERS.karl.user.principal_id],
  },
  render: ({ action_type, timestamp, user_principal }) => {
    const logs: ComponentProps<typeof ActionLogComponent>[] = [
      { action_type, user_principal },
    ];
    return (
      <TableModalLogEntryComponent timestamp={BigInt(timestamp)} logs={logs} />
    );
  },
  decorators: [
    (Story) => (
      <ProvideTable
        value={mockTableContext({
          users: [
            mockCumulativeUserTableData({
              user: PREVIEW_USERS.karl.user,
              data: {
                total_bet: FloatToTokenAmount(10, _meta),
                current_total_bet: FloatToTokenAmount(0, _meta),
              },
            }),
          ],
        })}
      >
        <Story />
      </ProvideTable>
    ),
  ],
};

export default meta;

type Story = StoryObj<LogArgs>;

export const Bet: Story = {
  args: { action_type: { Bet: { amount: FloatToTokenAmount(100, _meta) } } },
};
export const Win: Story = {
  args: { action_type: { Win: { amount: FloatToTokenAmount(200, _meta) } } },
};
export const Leave: Story = { args: { action_type: { Leave: null } } };
export const Call: Story = { args: { action_type: { Call: null } } };
export const Fold: Story = { args: { action_type: { Fold: null } } };
export const Join: Story = { args: { action_type: { Join: null } } };
export const BigBlind: Story = { args: { action_type: { BigBlind: null } } };
export const Raise: Story = {
  args: { action_type: { Raise: { amount: FloatToTokenAmount(150, _meta) } } },
};
export const SmallBlind: Story = {
  args: { action_type: { SmallBlind: null } },
};
export const Check: Story = { args: { action_type: { Check: null } } };
