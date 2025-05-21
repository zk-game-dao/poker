import type { Meta, StoryObj } from "@storybook/react";


import { ChatInputComponent } from '../chat-input.component';

const meta: Meta<typeof ChatInputComponent> = {
  title: "Chat/Input",
  component: ChatInputComponent,
  args: {
    onSend: console.log,
    isSending: false
  },
};

export default meta;

type Story = StoryObj<typeof ChatInputComponent>;

export const ChatInput: Story = {};
export const IsSending: Story = {
  args: { isSending: true }
};
