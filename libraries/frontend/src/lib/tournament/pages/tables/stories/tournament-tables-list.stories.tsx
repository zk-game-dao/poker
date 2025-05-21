
import { mockPrincipal, mockTable } from '../../../../utils/mock';
import { TournamentTablesListComponent } from '../tournament-tables-list.component';

import type { Meta, StoryObj } from "@storybook/react";
const meta: Meta<typeof TournamentTablesListComponent> = {
  title: "Tournament/Tables/List",
  component: TournamentTablesListComponent,
  parameters: {
    layout: "fullscreen",
  },
  args: {
    tables: [
      mockTable({ id: mockPrincipal(0) }),
      mockTable({ id: mockPrincipal(1) }),
      mockTable({ id: mockPrincipal(2) }),
      mockTable({ id: mockPrincipal(3) }),
      mockTable({ id: mockPrincipal(4) }),
    ],
  }
};

export default meta;

type Story = StoryObj<typeof TournamentTablesListComponent>;

export const TournamentTablesList: Story = {};
export const TournamentTablesListMobile: Story = {
  globals: {
    viewport: 'mobile1',
  }
};
export const TournamentTablesListTablet: Story = {
  globals: {
    viewport: 'mobile2',
  }
};
