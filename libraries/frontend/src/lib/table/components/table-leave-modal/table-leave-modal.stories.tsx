import type { Meta, StoryObj } from "@storybook/react";

import { mockTableContext } from '@lib/utils/mock';

import { ProvideTable } from '../../context/table.context';
import { TableLeaveModalComponent } from './table-leave-modal.component';

type Props = {
  isGameOngoing: boolean;
};

const meta: Meta<Props> = {
  title: "Table/Leave Modal",
  args: {
    isGameOngoing: false,
  },
  render: ({ isGameOngoing }) => {
    return (
      <ProvideTable
        value={mockTableContext({
          isJoined: true,
          isOngoing: isGameOngoing,
        })}
      >
        <TableLeaveModalComponent
          blocker={{
            state: "blocked",
            location: {} as any,
            reset: () => { },
            proceed: () => { },
          }}
        />
      </ProvideTable>
    );
  },
};

export default meta;

type Story = StoryObj<Props>;

export const LeaveModal: Story = {};
export const LeaveModalOngoing: Story = { args: { isGameOngoing: true } };
