import type { Meta, StoryObj } from "@storybook/react";

import { LobbyTableCardComponent } from "./lobby-table-card.component";
import { mockTable } from "@/src/lib/utils/mock";

const meta: Meta<typeof LobbyTableCardComponent> = {
  title: "Pages/Lobby/TableCard",
  component: LobbyTableCardComponent,
  args: {
    ...mockTable({ config: { name: "Table Name", seats: 5 } }),
    index: 0,
  },
  parameters: {
    layout: "fullscreen",
  },
};

export default meta;

type Story = StoryObj<typeof LobbyTableCardComponent>;

export const TableCard: Story = {};
export const TableCardSmall: Story = { args: { variant: "small" } };
export const TableCardTeal: Story = {
  args: {
    ...mockTable({
      config: { name: "Table Name", seats: 5, environment_color: 1n },
    }),
  },
};
export const TableCardRedBGGreenCards: Story = {
  args: {
    ...mockTable({
      config: { name: "Table Name", seats: 5, color: 1n, card_color: 0n },
    }),
  },
};
export const TableCardRedBGRedCards: Story = {
  args: {
    ...mockTable({
      config: { name: "Table Name", seats: 5, color: 1n, card_color: 1n },
    }),
  },
};
export const TableFakeMoney: Story = {
  args: {
    ...mockTable({ config: { currency_type: { Fake: null } } }),
  },
};
export const MultipleTableFakeMoney: Story = {
  render: () => (
    <div className="flex flex-row items-center relative my-4 lg:mb-6 gap-4 flex-wrap">
      {Array.from({ length: 10 }, (_, i) => (
        <LobbyTableCardComponent
          key={i}
          {...mockTable({
            config: {
              currency_type: { Fake: null },
              environment_color: BigInt(i % 6),
            }
          })}
          index={i}
        />
      ))}
    </div>
  )
};
