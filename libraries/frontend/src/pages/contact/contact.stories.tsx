import type { Meta, StoryObj } from "@storybook/react";

import { ContactPage } from "./contact.page";

const meta: Meta<typeof ContactPage> = {
  title: "Pages/Contact",
  component: ContactPage,
  parameters: {
    layout: "fullscreen",
  },
};

export default meta;

type Story = StoryObj<typeof ContactPage>;

export const Contact: Story = {};
