import type { Meta, StoryObj } from "@storybook/react";

import { mockTable, mockTableContext } from '@/src/lib/utils/mock';
import { FloatToTokenAmount } from '@/src/lib/utils/token-amount-conversion';

import { ProvideTable } from '../../context/table.context';
import { TableSettingsModalComponent } from './table-settings-modal.component';

const _meta = { decimals: 8, thousands: 10 ** 8, transactionFee: 10_000n };

const meta: Meta<typeof TableSettingsModalComponent> = {
  title: "Table/SettingsModal",
  component: TableSettingsModalComponent,
  args: {
    show: true,
    onClose: () => { },
  },
  decorators: [
    (Story) => (
      <ProvideTable
        value={mockTableContext({
          url: "https://example.com",
          table: mockTable({
            small_blind: FloatToTokenAmount(1, _meta),
            big_blind: FloatToTokenAmount(2, _meta),
            config: {
              game_type: {
                FixedLimit: [
                  FloatToTokenAmount(2, _meta),
                  FloatToTokenAmount(4, _meta),
                ],
              },
            },
          }),
        })}
      >
        <Story />
      </ProvideTable>
    ),
  ],
};

export default meta;

type Story = StoryObj<typeof TableSettingsModalComponent>;

export const SettingsModal: Story = {};
