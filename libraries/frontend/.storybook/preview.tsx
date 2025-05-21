import '../src/styles.css';

import { clearAllMocks, fn } from '@storybook/test';
import { ProvideLayoutConfig, ProvideModalStack, ProvideUI, ToastProvider } from '@zk-game-dao/ui';
import isChromatic from 'chromatic';
import { MotionConfig, MotionGlobalConfig } from 'framer-motion';
import React from 'react';
import { createMemoryRouter, RouterProvider } from 'react-router-dom';

import { ProvideFeedbackContext } from '../src/context/feedback/feedback.context';
import { ProvideSound } from '../src/context/sound.context';
import { ElementBoxProvider } from '../src/hooks/element-box';
import { ProvideQuery } from '../src/lib/data';
import * as ctx from '../src/lib/table/context/table.context';
import { RawUserContextProvider } from '../src/lib/user/types/user.context';
import { AllMockUsers, PREVIEW_USERS, PreviewUsers } from '../src/lib/utils/mock';

import type { Preview } from "@storybook/react";
import { ProvideCurrencyContext } from '@zk-game-dao/currency';
import { Principal } from '@dfinity/principal';


(BigInt.prototype as any).toJSON = function () {
  return this.toString();
};

if (isChromatic()) {

  MotionGlobalConfig.skipAnimations = isChromatic()

  const fixedDate = new Date('2023-01-01T12:00:00Z');

  // Store the original Date object
  const OriginalDate = Date;

  // Create a custom Date class that returns the fixed date
  class MockDate extends Date {
    constructor(...params: [any?]) {
      super(...params);
      return params.length > 0 ? this : fixedDate;
    }

    // Static methods need to be copied from the original Date object
    static now() {
      return fixedDate.getTime();
    }

    static parse(s: string) {
      return OriginalDate.parse(s);
    }

    static UTC(...args: any[]) {
      return (OriginalDate.UTC as any)(...args);
    }

    static get [Symbol.species]() {
      return Date;
    }
  }

  global.Date = MockDate as any;

  global.Math.random = () => 0.5;

}


// initialize();


const preview: Preview = {
  globalTypes: {
    user: {
      description: 'User',
      toolbar: {
        icon: 'user',
        items: [
          { value: undefined, title: 'Logged out' },
          { value: 'karl', right: '👑', title: 'Karl' },
          { value: 'karl-no-wallet', right: '👑', title: 'Karl without wallet' },
          { value: 'aaron', title: 'Aaron' },
        ],
      },
    },
    platform: {
      description: 'Platform',
      toolbar: {
        icon: 'eye',
        items: [
          { value: 'purepoker', right: 'purepoker', title: 'Pure Poker' },
          { value: 'zkpoker', right: 'zkpoker', title: 'zkpoker' },
        ],
      },
    },
  },
  initialGlobals: {
    platform: 'purepoker',
  },
  decorators: [
    (Story) => {
      clearAllMocks();
      fn(ctx.useTableUserFromCanisterId)
        .mockImplementation((canisterId) => {
          if (!canisterId) return [undefined, undefined];
          const user = AllMockUsers.find(({ user }) => user.principal_id.compareTo(canisterId));
          if (!user) return [undefined, undefined];
          return [user.user, user.cumulative];
        });
      return <Story />
    },
    (Story, context) => (
      <RouterProvider
        key={context.globals.platform}
        router={createMemoryRouter([
          {
            path: '/',
            element: (
              <ProvideQuery>
                <ProvideUI theme={context.globals.platform}>
                  <ProvideCurrencyContext
                    enabledNetworks={context.globals.platform === 'purepoker' ? ['btc'] : ['ic', 'eth']}
                    siwbProviderCanisterId={undefined}
                  >
                    <ProvideModalStack>
                      <ProvideSound>
                        <RawUserContextProvider
                          value={{
                            isLoading: false,
                            ...PREVIEW_USERS[context.globals.user as PreviewUsers],
                            showProfile: () => { },
                            showSignup: () => { },
                            show: () => { },
                          }}
                        >
                          <ElementBoxProvider>
                            <MotionConfig>
                              <ToastProvider>
                                <ProvideFeedbackContext>
                                  <Story />
                                </ProvideFeedbackContext>
                              </ToastProvider>
                            </MotionConfig>
                          </ElementBoxProvider>
                        </RawUserContextProvider>
                      </ProvideSound>
                    </ProvideModalStack>
                  </ProvideCurrencyContext>
                </ProvideUI>
              </ProvideQuery>
            )
          }
        ])}
      />
    )
  ],

  loaders: [
    // mswLoader
  ],

  tags: ['autodocs']
};

export default preview;
