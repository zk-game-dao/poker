import type { Meta, StoryObj } from "@storybook/react";

import {
  mockCumulativeUserTableData, mockTable, mockTableContext, PREVIEW_USERS
} from '@lib/utils/mock';
import { FloatToTokenAmount } from '@lib/utils/token-amount-conversion';

import { RawUserContextProvider } from '@lib/user';
import { ProvideTable } from '../../context/table.context';
import { TableBalanceModalComponent } from './table-balance-modal.components';

const _meta = { decimals: 8, thousands: 10 ** 8, transactionFee: 10_000n };

const meta: Meta = {
  title: "Table/TableBalanceModal",
  render: () => <TableBalanceModalComponent isOpen={true} onClose={() => { }} />,
  args: {
    balance: 50,
    tableBalance: 100,
  },
  argTypes: {
    balance: { control: { type: "number" } },
    tableBalance: { control: { type: "number" } },
  },
  decorators: [
    (Story, { args: { balance } }) => (
      <RawUserContextProvider
        value={{
          isLoading: false,
          showProfile: () => { },
          showSignup: () => { },
          show: () => { },
          ...PREVIEW_USERS.karl,
          user: {
            ...PREVIEW_USERS.karl.user,
            balance: FloatToTokenAmount(balance, _meta),
          },
        }}
      >
        <Story />
      </RawUserContextProvider>
    ),
    (Story, { args: { tableBalance } }) => (
      <ProvideTable
        value={mockTableContext({
          users: [
            mockCumulativeUserTableData({
              user: PREVIEW_USERS.karl.user,
              data: {
                total_bet: FloatToTokenAmount(10, _meta),
                current_total_bet: FloatToTokenAmount(tableBalance, _meta),
              },
            }),
          ],
          table: mockTable({
            users: {
              users: [
                [
                  PREVIEW_USERS.karl.user.principal_id,
                  {
                    ...PREVIEW_USERS.karl.user,
                    balance: FloatToTokenAmount(tableBalance, _meta),
                  },
                ],
              ],
            },
          }),
          user: mockCumulativeUserTableData({
            user: PREVIEW_USERS.karl.user,
            data: {
              total_bet: FloatToTokenAmount(10, _meta),
              current_total_bet: FloatToTokenAmount(tableBalance, _meta),
            }
          }),
        })}
      >
        <Story />
      </ProvideTable>
    ),
  ],
};

export default meta;

type Story = StoryObj;

export const TableBalanceModal: Story = {};
