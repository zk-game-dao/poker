import type { Meta, StoryObj } from "@storybook/react";

import { RawUserContextProvider } from '@lib/user';
import { PREVIEW_USERS } from '@lib/utils/mock';

import { CreateTableModalComponent } from './create-table-modal.component';

type Props = {
  step: number;
  isLoggedIn?: boolean;
};

const meta: Meta<Props> = {
  title: "Common/Modal/Create Table Modal",
  args: {
    step: 0,
    isLoggedIn: true,
  },
  render: ({ isLoggedIn, step }) => (
    <RawUserContextProvider
      value={{
        ...(isLoggedIn ? PREVIEW_USERS.karl : undefined),
        isLoading: false,
        showProfile() { },
        show: () => { },
        showSignup() { },
      }}
    >
      <CreateTableModalComponent open onCancel={() => { }} initialStep={step} />
    </RawUserContextProvider>
  ),
};
export default meta;

type Story = StoryObj<Props>;

export const NotLoggedIn: Story = { args: { isLoggedIn: false } };
export const SetTypeStep: Story = {
  args: { step: 0 },
};
export const SetTimeStep: Story = {
  args: { step: 1 },
};
export const SetAppearanceStep: Story = {
  args: { step: 2 },
};
export const SetPlayerCountStep: Story = {
  args: { step: 3 },
};
export const SetNameStep: Story = { args: { step: 4 } };
export const SetPreviewStep: Story = {
  args: { step: 5 },
};