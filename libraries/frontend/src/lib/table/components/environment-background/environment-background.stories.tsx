import type { Meta, StoryObj } from "@storybook/react";

import { EnvironmentBackgroundComponent } from "./environment-background.component";
import { EnvironmentColor } from "@/src/models/table-color.model";

const meta: Meta<typeof EnvironmentBackgroundComponent> = {
  title: "Table/Environment/Background",
  component: EnvironmentBackgroundComponent,
  args: {
    className: "fixed inset-0",
    color: EnvironmentColor.Purple,
  },
  argTypes: {
    color: {
      options: [
        EnvironmentColor.Green,
        EnvironmentColor.Red,
        EnvironmentColor.Blue,
        EnvironmentColor.Purple,
        EnvironmentColor.Yellow,
        EnvironmentColor.Black,
      ],
      control: {
        type: "select",
      },
    },
  },
};

export default meta;

type Story = StoryObj<typeof EnvironmentBackgroundComponent>;

export const Background: Story = { args: { color: EnvironmentColor.Purple } };
export const BackgroundRed: Story = { args: { color: EnvironmentColor.Red } };
export const BackgroundBlue: Story = { args: { color: EnvironmentColor.Blue } };
export const BackgroundPurple: Story = {
  args: { color: EnvironmentColor.Purple },
};
export const BackgroundYellow: Story = {
  args: { color: EnvironmentColor.Yellow },
};
export const BackgroundBlack: Story = {
  args: { color: EnvironmentColor.Black },
};
