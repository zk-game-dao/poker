import Analytics from 'analytics';
import {
  createContext, memo, PropsWithChildren, useCallback, useContext, useEffect, useMemo
} from 'react';
import { useLocation } from 'react-router-dom';

import googleAnalytics from '@analytics/google-analytics';
import { PublicTable } from '@declarations/table_canister/table_canister.did';
import { useUser } from '@lib/user';
import { IsDev, SelectEnv, useModal } from '@zk-game-dao/ui';

// import googleTagManager from "@analytics/google-tag-manager";
const analytics = Analytics({
  app: "ZK Poker",
  plugins: [
    googleAnalytics({
      measurementIds: [
        SelectEnv<string>({
          development: 'G-BKMFZKJ5SD',
          staging: 'G-BKMFZKJ5SD',
          production: "G-K7K2N6EW8F"
        }),
      ],
    }),
    // googleTagManager({
    //   containerId: SelectEnv<string>({
    //     development: 'GTM-M3HLPD8W',
    //     staging: 'GTM-M3HLPD8W',
    //     production: "GTM-KBV68N6V",
    //   }),
    // }),
  ],
  debug: IsDev,
});

type AnalyticsEvent =
  | ["button-click", { content: string }]
  | ["create-table", PublicTable];

const AnalyticsContext = createContext<{
  track: (name: string, extraProps?: { [key: string]: any }) => void;
}>({ track: () => { } });

export const ProvideAnalytics = memo<PropsWithChildren>(({ children }) => {
  const { user } = useUser();
  const { pathname } = useLocation();

  useEffect(() => {
    if (!user?.principal_id) return;
    analytics.identify(user?.principal_id.toText(), user);
  }, [user?.principal_id.toText()]);

  const track = useCallback(
    (name: string, extraProps?: { [key: string]: any }) => {
      analytics.track(name, extraProps);
    },
    [],
  );

  useEffect(() => {
    analytics.page();
  }, [pathname]);

  return (
    <AnalyticsContext.Provider value={{ track }}>
      {children}
    </AnalyticsContext.Provider>
  );
});
ProvideAnalytics.displayName = "ProvideAnalytics";

export const useAnalytics = () => {
  const { track } = useContext(AnalyticsContext);
  const { title: modal } = useModal();

  const contextualTrack = useCallback(
    (...eventProps: AnalyticsEvent) => {
      track(eventProps[0], {
        modal,
        ...eventProps[1],
      });
    },
    [modal, track],
  );

  return useMemo(() => ({ track: contextualTrack }), [contextualTrack]);
};
