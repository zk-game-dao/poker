import type { Meta, StoryObj } from "@storybook/react";

import { ChatHistoryComponent } from "../chat-history.component";
import { DateToBigIntTimestamp } from "@zk-game-dao/ui";
import { Principal } from "@dfinity/principal";
import { ChatMessage } from "@declarations/table_canister/table_canister.did";
import { RawUserContextProvider } from "../../../../lib/user";
import { mockUser } from "../../../../lib/utils/mock";

function principalFromNumber(num: number) {
  // Convert number to a Uint8Array (e.g., single byte for simplicity)
  const buffer = new Uint8Array([num]);
  return Principal.fromUint8Array(buffer);
}

const senders = [
  {
    sender_name: 'Karl Marx',
    sender: principalFromNumber(1)
  },
  {
    sender_name: 'Elon musk',
    sender: principalFromNumber(2)
  },
  {
    sender_name: 'Friedrich Engels',
    sender: principalFromNumber(3)
  }
]

const GenerateMessage = (id: bigint, sender: number, content: string): ChatMessage => ({
  id,
  ...senders[sender],
  content,
  timestamp: DateToBigIntTimestamp(new Date("2023-01-01T12:00:00Z")),
  edited: false,
  recipient: [],
  edit_timestamp: [],
  message_type: {
    TableMessage: null
  }
});

const meta: Meta<typeof ChatHistoryComponent> = {
  title: "Chat/History",
  component: ChatHistoryComponent,
  args: {
    messages: [
      GenerateMessage(1n, 0, "Workers of the world, unite!"),
      GenerateMessage(2n, 1, "But what about my Mars colony?"),
      GenerateMessage(3n, 2, "Elon, your Mars colony won't save capitalism."),
      GenerateMessage(4n, 0, "Exactly, Friedrich."),
      GenerateMessage(5n, 0, "The proletariat will rise."),
      GenerateMessage(6n, 0, "We must seize the means of production."),
      GenerateMessage(7n, 1, "But I have electric cars and rockets!"),
      GenerateMessage(8n, 2, "And yet, inequality persists. Checkmate, Elon."),
      GenerateMessage(9n, 2, "The system is rigged against the working class."),
      GenerateMessage(10n, 0, "The revolution is inevitable."),
      GenerateMessage(11n, 1, "Fine, I'll join the revolution. Can I still build rockets?"),
      GenerateMessage(12n, 0, "Only if they're for the people."),
      GenerateMessage(13n, 2, "Welcome to the cause, comrade Elon."),
      GenerateMessage(14n, 0, "History is the history of class struggles."),
    ]
  },
  decorators: [
    (Story) => (
      <RawUserContextProvider
        value={{
          user: mockUser({
            principal_id: senders[0].sender,
            users_canister_id: senders[0].sender,
          }),
          isLoading: false,
          show: () => { },
          showProfile: () => { },
          showSignup: () => { },
        }}
      >
        <Story />
      </RawUserContextProvider>
    ),
  ],
};

export default meta;

type Story = StoryObj<typeof ChatHistoryComponent>;

export const ChatHistory: Story = {};
export const LimitHeight: Story = {
  decorators: [
    (Story) => (
      <Story />
    ),
  ],
};