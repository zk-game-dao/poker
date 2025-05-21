import type { Meta, StoryObj } from "@storybook/react";

import { SpinnerComponent } from "./spinner.component";

const meta: Meta<typeof SpinnerComponent> = {
  title: "Common/Spinner",
  component: SpinnerComponent,
  args: {
    result: 1123412,
    symbol: "$",
  },
};
export default meta;

type Story = StoryObj<typeof SpinnerComponent>;

export const Spinner: Story = {};