import './styles.css';

import classNames from 'classnames';
import { memo, useContext, useState } from 'react';
import { Outlet } from 'react-router-dom';

import { Principal } from '@dfinity/principal';
import { ProvideUser } from '@lib/user/context/user-provider.component';
import {
  CurrencyComponent, CurrencyType, ProvideCurrencyContext, useBalance
} from '@zk-game-dao/currency';
import {
  Interactable, LayoutProvider, LoadingAnimationComponent, PillComponent, ProvideConfirmModal,
  ProvideErrorModalContext, ProvideLayoutConfig, ProvideUI, ToastProvider
} from '@zk-game-dao/ui';

import AvatarComponent from './components/common/avatar/avatar.component';
import {
  RegionBlockModalComponent
} from './components/region-block-modal/region-block-modal.component';
import { ProvideAnalytics } from './context/analytics.context';
import { ProvideFeedbackContext } from './context/feedback/feedback.context';
import { ThemeContext, ThemeContextType } from './context/platform-theme.context';
import { ProvideSound } from './context/sound.context';
import { ElementBoxProvider } from './hooks/element-box';
import { ProvideUsersCanisters } from './lib/user/context/users-canisters.context';
import { useUser } from './lib/user/types/user.context';

const UserItem = memo(() => {
  const { shownCurrencyType } = useContext(ThemeContext);

  const { user, isLoading, showSignup, showProfile } = useUser();
  const balance = useBalance(shownCurrencyType);

  if (isLoading)
    return <LoadingAnimationComponent></LoadingAnimationComponent>;

  return (
    <>
      {!user && (
        <PillComponent
          size="small"
          className="pointer-events-auto ml-auto lg:ml-0"
          onClick={showSignup}
        >
          Sign in
        </PillComponent>
      )}
      <div
        className={classNames(
          "flex flex-row justify-center items-center",
          user ? "flex" : "lg:hidden",
        )}
      >
        {user && (
          <>
            <div className="h-[24px] w-[1px] bg-material-main-2 lg:hidden mr-6" />
            <Interactable
              className={classNames(
                "flex flex-row justify-center items-center lg:rounded-full lg:material gap-2 lg:py-1 lg:pl-1 lg:pr-2",
              )}
              onClick={showProfile}
            >
              <AvatarComponent
                size="between-small-and-microscopic"
                {...user}
              />
              <div className="hidden lg:flex">
                <CurrencyComponent
                  currencyType={shownCurrencyType}
                  currencyValue={balance}
                  variant="inline"
                  size="small"

                />
              </div>
            </Interactable>
          </>
        )}
      </div>
    </>
  );
});
UserItem.displayName = 'UserItem';

export const RootComponent = memo<Omit<ThemeContextType, 'setShownCurrencyType'>>((theme) => {
  const [shownCurrencyType, setShownCurrencyType] = useState<CurrencyType>();

  return (
    <ProvideUsersCanisters>
      <ThemeContext.Provider
        value={{
          ...theme,
          shownCurrencyType: shownCurrencyType ?? theme.shownCurrencyType,
          setShownCurrencyType: (type) => {
            setShownCurrencyType(type);
          }
        }}
      >
        <ProvideUI theme={theme.isBTC ? 'purepoker' : 'zkpoker'} banner={theme.banner}>
          <ProvideCurrencyContext
            enabledNetworks={theme.isBTC ? ['btc'] : ['ic', 'eth']}
            siwbProviderCanisterId={'CANISTER_ID_IC_SIWB_PROVIDER' in process.env ? Principal.from(process.env.CANISTER_ID_IC_SIWB_PROVIDER) : undefined}
          >
            <ProvideLayoutConfig
              navbarTabs={[
                // { type: "link", label: "Profile", onClick: () => theme.showProfile() },
                { type: "link", label: "Cash games", href: "/cash-games" },
                { type: "link", label: "Leaderboard", comingSoon: theme.isBTC, href: theme.isBTC ? undefined : "/leaderboard" },
                { type: "link", label: "Tournaments", href: "/tournaments" },
                { type: "seperator" },
                { type: "link", label: "Roadmap", href: "/roadmap" },
                { type: "link", label: "Changelog", href: "/changelog" },
                { type: "link", label: "Become a Host", href: "/become-host" },
                // TODO: Re-add support us
                // {
                //   type: "link",
                //   label: "Support us",
                //   onClick: () => setShowSupportUs(true),
                //   mobileOnly: true
                // },
                {
                  type: "link",
                  label: "Contact",
                  href: "/contact",
                  mobileOnly: true
                },
                { type: "link", label: "House rules", href: "/rules", mobileOnly: true },
                { type: "link", label: "Store", href: "/store", mobileOnly: true },
              ]}
              footerLinks={[
                { label: "Contact", href: "/contact" },
                { label: "Rake", href: "/rake" },
                // TODO: Re-add Feedback
                // { type: "link", label: "Feedback", onClick: openFeedback, mobileOnly: true },
                // TODO: Re-add Support us
                // { type: "link", label: "Support us", onClick: () => setShowSupportUs(true), mobileOnly: true },
                { label: "House rules", href: "/rules" },
              ]}
              NavbarUserComponent={UserItem}
            >
              <ProvideConfirmModal>
                <ElementBoxProvider>
                  <ToastProvider>
                    <ProvideUser>
                      <ProvideSound>
                        <ProvideFeedbackContext>
                          <ProvideAnalytics>
                            <LayoutProvider>
                              <ProvideErrorModalContext>
                                <RegionBlockModalComponent />
                                <Outlet />
                              </ProvideErrorModalContext>
                            </LayoutProvider>
                          </ProvideAnalytics>
                        </ProvideFeedbackContext>
                      </ProvideSound>
                    </ProvideUser>
                  </ToastProvider>
                </ElementBoxProvider>
              </ProvideConfirmModal>
            </ProvideLayoutConfig>
          </ProvideCurrencyContext>
        </ProvideUI >
      </ThemeContext.Provider >
    </ProvideUsersCanisters>
  );
});
RootComponent.displayName = 'RootComponent';