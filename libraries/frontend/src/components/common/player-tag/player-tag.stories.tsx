import type { Meta, StoryObj } from "@storybook/react";
import { PREVIEW_USERS } from "@/src/lib/utils/mock";

import { PlayerTag, PlayerTagProps } from "./player-tag.component";
import { FloatToTokenAmount } from "@/src/lib/utils/token-amount-conversion";
import { TableUIContext } from "../../../lib/table/context/table-ui.context";

const _meta = { decimals: 8, thousands: 10 ** 8, transactionFee: 10_000n };
type Props = PlayerTagProps & { isMobile?: boolean };

const meta: Meta<Props> = {
  title: "Common/Player Tag",
  render: (args) => (
    <TableUIContext.Provider
      value={{
        setSeatPosition: () => { },
        getSeatPosition: () => ({ x: 0, y: 0 }),
        tableRef: { current: null as any },
        visualSize: 1,
        orientation: "portrait",
        animatePots: !args.isMobile,
      }}
    >
      <PlayerTag {...args} />
    </TableUIContext.Provider>
  ),
  args: {
    user_name: PREVIEW_USERS.karl.user.user_name,
    avatar: PREVIEW_USERS.karl.user.avatar,
    balance: PREVIEW_USERS.karl.user.balance,
    currencyType: { Real: { ICP: null } }
  },
  argTypes: {
    player_action: {
      options: ["Folded", "Checked", "Called", "Raised", "AllIn"],
      mapping: {
        Folded: { Folded: null },
        Checked: { Checked: null },
        Called: { Called: null },
        Raised: { Raised: 123.456 },
        AllIn: { AllIn: null },
      },
      control: { type: "select" },
    },
    direction: { control: { type: "select" }, options: ["up", "down"] },
    turnProgress: {
      control: {
        type: "number",
        min: 0,
        max: 1,
      },
    },
  }
};

export default meta;

type Story = StoryObj<Props>;

export const Default: Story = {};
export const DefaultSelf: Story = { args: { isSelf: true } };
export const DefaultMobile: Story = {
  args: { isMobile: true },
  globals: {
    viewport: 'mobile1',
  },
};
export const DefaultSelfMobile: Story = {
  args: {
    isSelf: true,
    isMobile: true
  },
  globals: {
    viewport: 'mobile1',
  },
};
export const IsTurn: Story = { args: { turnProgress: 0.2 } };
export const IsTurnSelf: Story = { args: { turnProgress: 0.2, isSelf: true } };
export const IsTurnSelfMobile: Story = { args: { turnProgress: 0.2, isSelf: true, isMobile: true } };
export const Folded: Story = { args: { player_action: { Folded: null } } };
export const Queued: Story = { args: { isQueued: true } };

export const FoldedDown: Story = {
  args: { player_action: { Folded: null }, direction: "down" },
};
export const Raised: Story = {
  args: { player_action: { Raised: FloatToTokenAmount(1234.1234, _meta) } },
};
export const Called: Story = { args: { player_action: { Called: null } } };
export const CalledAndCards: Story = {
  args: { player_action: { Called: null }, cards: [undefined, undefined] },
};
export const Checked: Story = { args: { player_action: { Checked: null } } };
export const AllIn: Story = { args: { player_action: { AllIn: null } } };
export const SittingOut: Story = {
  args: { player_action: { SittingOut: null } },
};