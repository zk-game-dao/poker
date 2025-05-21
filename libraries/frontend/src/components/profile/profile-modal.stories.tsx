import type { Meta, StoryObj } from "@storybook/react";

import { mockTableContext, PREVIEW_USERS } from '@/src/lib/utils/mock';
import { ProvideTable } from '@lib/table/context/table.context';

import { ProfileModalComponent } from './profile-modal.component';

const meta: Meta<typeof ProfileModalComponent> = {
  title: "Profile/ProfileModal",
  component: ProfileModalComponent,
  args: {
    isOpen: true,
    onClose: () => console.log("close"),
  },
  decorators: [
    (Story) => (
      <ProvideTable value={mockTableContext()}>
        <Story />
      </ProvideTable>
    ),
  ],
};
export default meta;

type Story = StoryObj<typeof ProfileModalComponent>;

export const ProfileModal: Story = {
  args: {
    user: PREVIEW_USERS.karl.user,
  },
};
