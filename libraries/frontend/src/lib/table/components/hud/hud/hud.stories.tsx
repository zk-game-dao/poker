import type { Meta, StoryObj } from "@storybook/react";

import { ComponentProps } from 'react';

import { RawUserContextProvider } from '@lib/user';
import { Card, DealStage } from '@declarations/table_canister/table_canister.did';
import { mockTable, mockTableContext, mockUserArray } from '@lib/utils/mock';
import { FloatToTokenAmount } from '@lib/utils/token-amount-conversion';

import { ProvideTable } from '../../../context/table.context';
import { HUDComponent } from '../hud.component';

const _meta = { decimals: 8, thousands: 10 ** 8, transactionFee: 10_000n };

type StoryProps = ComponentProps<typeof HUDComponent> & {
  minRequiredBet: number;
  currentBet: number;
  userTurnProgress?: number;
  isDealer: boolean;
  isSittingOut: boolean;
  isJoined: boolean;
  dealStage: DealStage;
  playerCount: number;
  cards: Card[];
};

const meta: Meta<StoryProps> = {
  title: "Table/HUD",
  component: HUDComponent,
  args: {
    currentBet: 4,
    minRequiredBet: 4,
    isSittingOut: false,
    isJoined: true,
    dealStage: { Fresh: null },
    playerCount: 2,
    cards: [
      { value: { Ace: null }, suit: { Club: null } },
      { value: { King: null }, suit: { Heart: null } },
    ],
  },
  decorators: [
    (
      Story,
      {
        args: {
          cards,
          minRequiredBet,
          isSittingOut,
          isDealer,
          userTurnProgress,
          isJoined,
          playerCount,
          dealStage,
        },
      },
    ) => {
      const timer_duration = 30;
      const diff =
        Math.floor((1 - (userTurnProgress ?? 0)) * timer_duration) * 1000000000;
      const last_timer_started_timestamp = BigInt(Date.now() * 1000000 - diff);

      const users = mockUserArray(playerCount, (data) => ({
        user: {
          ...data.user,
          balance: FloatToTokenAmount(100, _meta),
        },
        cumulative: {
          ...data.cumulative,
          cards,
          player_action: isSittingOut ? { SittingOut: null } : { None: null },
          total_bet: FloatToTokenAmount(10, _meta),
          show_card_requests: [],
          current_total_bet: FloatToTokenAmount(0, _meta),
        },
      }));

      return (
        <RawUserContextProvider
          value={{
            user: users[0].user,
            isLoading: false,
            show: () => { },
            showProfile: () => { },
            showSignup: () => { },
          }}
        >
          <ProvideTable
            value={mockTableContext({
              url: "https://example.com",
              table: mockTable({
                small_blind: FloatToTokenAmount(1, _meta),
                big_blind: FloatToTokenAmount(2, _meta),
                current_player_index: userTurnProgress !== undefined ? 0n : 1n,
                config: {
                  game_type: {
                    FixedLimit: [
                      FloatToTokenAmount(2, _meta),
                      FloatToTokenAmount(4, _meta),
                    ],
                  },
                  timer_duration,
                },
                deal_stage: dealStage,
                dealer_position: isDealer ? 0n : 1n,
                last_timer_started_timestamp,
                seats: users.map(({ user }) => ({ Occupied: user.principal_id })),
                users: {
                  users: users.map(({ user }) => [user.principal_id, user]),
                },
                user_table_data: users.map(({ cumulative }) => [
                  cumulative.canister_id!,
                  cumulative.data!,
                ]),
              }),
              userIndex: 0n,
              isJoined,
              currentBet: FloatToTokenAmount(minRequiredBet, _meta),
              isOngoing: userTurnProgress !== undefined,
              user: !isJoined ? undefined : users[0].cumulative,
              users: users.map((u) => u.cumulative),
            })}
          >
            <div style={{ position: "absolute", bottom: 16, left: 16 }}>
              <Story />
            </div>
          </ProvideTable>
        </RawUserContextProvider>
      );
    },
  ],
};

export default meta;

type Story = StoryObj<StoryProps>;

export const HUDFreshNotJoined: Story = { args: { isJoined: false } };
export const HUDFreshWaitingOtherPlayers: Story = { args: { isDealer: false } };
export const HUDFreshDealer: Story = { args: { isDealer: true } };
export const HUDFreshNotDealer: Story = {
  args: { isJoined: true, isDealer: false },
};
export const HUDFreshSittingOut: Story = {
  args: { isSittingOut: true, isDealer: false },
};

export const HUDFlopTurn: Story = {
  args: { userTurnProgress: 0.5, dealStage: { Flop: null } },
};
export const HUDProgress43: Story = {
  args: { userTurnProgress: 0.43, dealStage: { Flop: null } },
};
