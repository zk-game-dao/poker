import type { Meta, StoryObj } from "@storybook/react";

import { ChatMessageComponent } from "../chat-message.component";
import { DateToBigIntTimestamp } from "@zk-game-dao/ui";

const meta: Meta<typeof ChatMessageComponent> = {
  title: "Chat/Message",
  component: ChatMessageComponent,
  args: {
    content: 'Death to the bourgeois',
    sender_name: 'Karl marx',
    timestamp: DateToBigIntTimestamp(new Date("2023-01-01T12:00:00Z")),
    isFirst: true,
    isLast: true,
  },
};

export default meta;

type Story = StoryObj<typeof ChatMessageComponent>;

export const ChatMessage: Story = {};
export const SelfChatMessage: Story = {
  args: { isSelf: true }
};
