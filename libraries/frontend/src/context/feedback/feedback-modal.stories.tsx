import type { Meta, StoryObj } from "@storybook/react";

import { FeedbackModalComponent } from "./feedback-modal.component";

const meta: Meta<typeof FeedbackModalComponent> = {
  title: "Global/FeedbackModal",
  component: FeedbackModalComponent,
  argTypes: {
    isOpen: { control: { type: "boolean" } },
    message: { control: { type: "text" } },
    onClose: { action: "closed" },
  },
};

export default meta;

type Story = StoryObj<typeof FeedbackModalComponent>;

export const Default: Story = {};
export const WithMessage: Story = { args: { message: "This is a message" } };
