import type { Meta, StoryObj } from "@storybook/react";

import { RawUserContextProvider } from '@lib/user';
import { PREVIEW_USERS } from '@lib/utils/mock';

import { CreateTournamentModalComponent } from './create-tournament-modal.component';

type Props = {
  step: number;
  isLoggedIn?: boolean;
};

const meta: Meta<Props> = {
  title: "Tournament/CreateTournamentModal",
  args: {
    step: 0,
    isLoggedIn: true,
  },
  render: ({ isLoggedIn, step }) => (
    <RawUserContextProvider
      value={{
        ...(isLoggedIn ? PREVIEW_USERS.karl : undefined),
        isLoading: false,
        show: () => { },
        showProfile() { },
        showSignup() { },
      }}
    >
      <CreateTournamentModalComponent open onCancel={() => { }} initialStep={step} initialType={"BuyIn"} />
    </RawUserContextProvider>
  ),
};
export default meta;

type Story = StoryObj<Props>;

export const NotLoggedIn: Story = { args: { isLoggedIn: false } };
export const SetBasicsStep: Story = { args: { step: 0 } }
export const SetTypeStep: Story = { args: { step: 1 } }
export const SetBuyInStep: Story = { args: { step: 2 } }
export const SetBlindsStep: Story = { args: { step: 3 } }
export const SetPayoutsStep: Story = { args: { step: 4 } }
export const SetAppeareanceStep: Story = { args: { step: 5 } }
export const SetTimeLimitStep: Story = { args: { step: 6 } }