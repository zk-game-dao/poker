import type { Meta, StoryObj } from "@storybook/react";

import { AvatarComponent } from "./avatar.component";

const meta: Meta<typeof AvatarComponent> = {
  title: "Common/Avatar",
  component: AvatarComponent,
  args: {
    size: "medium",
    avatar: [{ Emoji: { emoji: 0n, style: 0n } }],
  },
  argTypes: {
    size: {
      options: ["microscopic", "small", "medium", "big"],
      control: {
        type: "select",
        defaultValue: "medium",
      },
    },
    progress: {
      control: {
        type: "number",
        min: 0,
        max: 1,
      },
    },
  },
};
export default meta;

type Story = StoryObj<typeof AvatarComponent>;

export const Medium: Story = { args: { size: "medium" } };
export const MediumMobile: Story = {
  globals: {
    viewport: 'mobile1',
  },
};
export const NoAvatar: Story = { args: { avatar: [] } };
export const NoAvatarVerfied: Story = { args: { avatar: [], is_verified: [true] } };
export const Small: Story = { args: { size: "small" } };
export const Large: Story = { args: { size: "big" } };
export const LargeVerifiedInProgress: Story = { args: { size: 'big', is_verified: [true], progress: 0.4 } };
export const Microscopic: Story = { args: { size: "microscopic" } };
export const MicroscopicVerified: Story = { args: { size: "microscopic", is_verified: [true] } };
export const IsTurn: Story = {
  args: {
    progress: 0,
  },
};
export const Progress20: Story = {
  args: {
    progress: 0.2,
  },
};
export const Progress74: Story = {
  args: {
    progress: 0.75,
  },
};
export const ProgressFull: Story = {
  args: {
    progress: 1,
  },
};
