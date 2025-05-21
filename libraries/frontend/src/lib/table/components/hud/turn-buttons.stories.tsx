import type { Meta, StoryObj } from "@storybook/react";

import { mockTable, mockTableContext, mockUserArray } from '@lib/utils/mock';
import { FloatToTokenAmount } from '@lib/utils/token-amount-conversion';

import { RawUserContextProvider } from '@lib/user';
import { ProvideTable } from '../../context/table.context';
import { ProvideHUDBettingContext } from './hud-betting.context';
import { TurnButtonsComponent } from './turn-buttons.component';

const _meta = { decimals: 8, thousands: 10 ** 8, transactionFee: 10_000n };

type StoryProps = {
  bigBlind: number;
  userBet: number;
  lastRaise: number;
  pot: number;
  highestBet: number;
  userTableBalance: number;
};

const meta: Meta<StoryProps> = {
  title: "Table/HUD/TurnButtons",
  component: TurnButtonsComponent,
  args: {
    bigBlind: 1,
    userBet: 0,
    highestBet: 0,
    lastRaise: 0,
    pot: 0,
    userTableBalance: 100,
  },
  decorators: [
    (
      Story,
      {
        args: {
          bigBlind,
          lastRaise,
          userBet,
          highestBet,
          userTableBalance,
          pot,
        },
      },
    ) => {
      const users = mockUserArray(8, (data) => ({
        ...data,
        user: {
          ...data.user,
          balance: FloatToTokenAmount(userTableBalance, _meta),
        },
        cumulative: {
          ...data.cumulative,
          current_total_bet: FloatToTokenAmount(userBet, _meta),
          total_bet: FloatToTokenAmount(userBet, _meta),
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
                small_blind: FloatToTokenAmount(bigBlind / 2, _meta),
                big_blind: FloatToTokenAmount(bigBlind, _meta),
                last_raise: FloatToTokenAmount(lastRaise, _meta),
                users: {
                  users: users.map(({ user }) => [user.principal_id, user]),
                },
                seats: users.map(({ user }) => ({ Occupied: user.principal_id })),
                user_table_data: users.map(({ cumulative }) => [
                  cumulative.canister_id!,
                  cumulative.data!,
                ]),
                config: {
                  seats: 8,
                  game_type: {
                    FixedLimit: [
                      FloatToTokenAmount(2, _meta),
                      FloatToTokenAmount(4, _meta),
                    ],
                  },
                },
                pot: FloatToTokenAmount(pot, _meta),
                highest_bet: FloatToTokenAmount(highestBet, _meta),
              }),
              currentBet: FloatToTokenAmount(lastRaise, _meta),
              isJoined: true,
              getSeat: (index: number) => users[index]?.cumulative,
              users: users.map((u) => u.cumulative),
              user: users[0].cumulative,
            })}
          >
            <div className="flex flex-row gap-[2px]">
              <ProvideHUDBettingContext>
                <Story />
              </ProvideHUDBettingContext>
            </div>
          </ProvideTable>
        </RawUserContextProvider>
      );
    },
  ],
};

export default meta;

type Story = StoryObj<StoryProps>;

export const TurnButtons: Story = {};
export const TurnButtonsOnRaise: Story = { args: { lastRaise: 2 } };
export const TurnButtonsOnRaiseWithPot: Story = {
  args: { lastRaise: 4, pot: 20 },
};
export const TurnButtonsOnlyAllIn: Story = {
  args: { lastRaise: 400, pot: 20 },
};

export const BigHighestBet: Story = {
  args: {
    bigBlind: 1,
    userBet: 4,
    highestBet: 42,
    lastRaise: 4,
    pot: 20,
    userTableBalance: 50,
  },
};

export const CheckOrAllIn: Story = {
  args: {
    bigBlind: 1,
    userBet: 4,
    highestBet: 46,
    lastRaise: 4,
    pot: 20,
    userTableBalance: 50,
  },
};
