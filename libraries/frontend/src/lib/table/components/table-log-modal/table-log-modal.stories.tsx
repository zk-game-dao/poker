import type { Meta, StoryObj } from "@storybook/react";

import {
  mockCumulativeUserTableData, mockTable, mockTableContext, PREVIEW_USERS
} from '@/src/lib/utils/mock';
import { FloatToTokenAmount } from '@lib/utils/token-amount-conversion';

import { ProvideTable } from '../../context/table.context';
import { TableLogModalComponent } from './table-log-modal.component';

const _meta = { decimals: 8, thousands: 10 ** 8, transactionFee: 10_000n };

const meta: Meta<typeof TableLogModalComponent> = {
  title: "Table/Log Modal",
  component: TableLogModalComponent,
  args: {
    isOpen: true,
    onClose: () => { },
  },
};

const StartDate = BigInt(new Date(1995, 4, 6).getTime());

export default meta;

type Story = StoryObj<typeof TableLogModalComponent>;

export const Default: Story = {
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
              }
            }),
            mockCumulativeUserTableData({
              user: PREVIEW_USERS.aaron.user,
              data: {
                total_bet: FloatToTokenAmount(10, _meta),
                current_total_bet: FloatToTokenAmount(0, _meta),
              }
            }),
          ],
          table: mockTable({
            action_logs: [
              {
                action_type: { Join: null },
                user_principal: [PREVIEW_USERS.aaron.user.principal_id],
                timestamp: StartDate,
              },
              {
                action_type: { Join: null },
                user_principal: [PREVIEW_USERS.karl.user.principal_id],
                timestamp: StartDate + 10000000000n,
              },
              {
                action_type: { Bet: { amount: FloatToTokenAmount(50, _meta) } },
                user_principal: [PREVIEW_USERS.aaron.user.principal_id],
                timestamp: StartDate + 10000000000n,
              },
              {
                action_type: { Call: null },
                user_principal: [PREVIEW_USERS.karl.user.principal_id],
                timestamp: StartDate + 30000000000n,
              },
              {
                action_type: {
                  Raise: { amount: FloatToTokenAmount(100, _meta) },
                },
                user_principal: [PREVIEW_USERS.aaron.user.principal_id],
                timestamp: StartDate + 40000000000n,
              },
              {
                action_type: { Fold: null },
                user_principal: [PREVIEW_USERS.karl.user.principal_id],
                timestamp: StartDate + 50000000000n,
              },
              {
                action_type: { Leave: null },
                user_principal: [PREVIEW_USERS.aaron.user.principal_id],
                timestamp: StartDate + 60000000000n,
              },
              {
                action_type: { Join: null },
                user_principal: [PREVIEW_USERS.aaron.user.principal_id],
                timestamp: StartDate + 70000000000n,
              },
              {
                action_type: { BigBlind: null },
                user_principal: [PREVIEW_USERS.karl.user.principal_id],
                timestamp: StartDate + 80000000000n,
              },
              {
                action_type: { SmallBlind: null },
                user_principal: [PREVIEW_USERS.aaron.user.principal_id],
                timestamp: StartDate + 90000000000n,
              },
              {
                action_type: { Check: null },
                user_principal: [PREVIEW_USERS.karl.user.principal_id],
                timestamp: StartDate + 100000000000n,
              },
              {
                action_type: {
                  Win: { amount: FloatToTokenAmount(200, _meta) },
                },
                user_principal: [PREVIEW_USERS.karl.user.principal_id],
                timestamp: StartDate + 110000000000n,
              },
            ],
          }),
        })}
      >
        <Story />
      </ProvideTable>
    ),
  ],
};

export const Empty: Story = {
  args: {
    isOpen: true,
    onClose: () => { },
  },
  decorators: [
    (Story) => (
      <ProvideTable
        value={mockTableContext({
          users: [],
          table: mockTable({
            action_logs: [],
          }),
        })}
      >
        <Story />
      </ProvideTable>
    ),
  ],
};
