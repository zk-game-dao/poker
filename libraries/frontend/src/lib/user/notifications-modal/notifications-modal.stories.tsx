import type { Meta, StoryObj } from "@storybook/react";

import { NotificationsModalComponent } from './notifications-modal.component';

const meta: Meta<typeof NotificationsModalComponent> = {
  title: "Profile/NotificationsModal",
  component: NotificationsModalComponent,
  args: {
    isOpen: true,
    onClose: () => console.log("close"),
  },
};
export default meta;

type Story = StoryObj<typeof NotificationsModalComponent>;

export const ProfileModal: Story = {
  args: {},
};
