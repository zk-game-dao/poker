import type { Meta, StoryObj } from "@storybook/react";

import { RakeInfoModalComponent } from './rake-info-modal.component';

const meta: Meta<typeof RakeInfoModalComponent> = {
  title: "Table/RakeInfoModal",
  component: RakeInfoModalComponent,
  args: {
    isOpen: true,
    onClose: () => console.log("close"),
  },
};

export default meta;

type Story = StoryObj<typeof RakeInfoModalComponent>;

export const RakeInfoModal: Story = {};