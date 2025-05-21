import axios from 'axios';
import { createContext, memo, ReactNode, useContext } from 'react';

import { queryClient } from '@lib/data';
import { useUser } from '@lib/user';
import { useMutation, useQuery } from '@tanstack/react-query';
import { IsDev } from '@zk-game-dao/ui';

import { APIUrl } from '../../env/constants';

// console.log({ swUrl })
const Context = createContext<{
  connect: () => void;
  error: unknown;
  isConnecting: boolean;
  isConnected: boolean;
  enabled: boolean;
  isLoadingEnabled: boolean;
  setEnabled: (enabled: boolean) => void;
}>({
  connect: () => { },
  error: undefined as unknown,
  isConnecting: false,
  isConnected: false,
  enabled: false,
  isLoadingEnabled: false,
  setEnabled: () => { },
});

export const useNotifications = () => useContext(Context);

// Helper function to convert the VAPID public key to the required format
function urlBase64ToUint8Array(base64String: string) {
  const padding = "=".repeat((4 - (base64String.length % 4)) % 4);
  const base64 = (base64String + padding).replace(/-/g, "+").replace(/_/g, "/");
  const rawData = window.atob(base64);
  return Uint8Array.from([...rawData].map((char) => char.charCodeAt(0)));
}

// const PropagatePushNotification = (principal: Principal, notification: PushSubscription) => {
//   const subscription: {
//     endpoint: string,
//     keys: {
//       auth: string,
//       p256dh: string,
//     },
//   } = {
//     endpoint: notification.endpoint,
//     keys: {
//       auth: notification.getKey('auth') ? btoa(String.fromCharCode.apply(null, Array.from(new Uint8Array(notification.getKey('auth')!)))) : '',
//       p256dh: notification.getKey('p256dh') ? btoa(String.fromCharCode.apply(null, Array.from(new Uint8Array(notification.getKey('p256dh')!)))) : '',
//     },
//   }
//   return axios.post(`${APIUrl}/notification/${principal.toText()}`, { body: { subscription } });
// }

export const ProvideNotifications = memo<{ children: ReactNode }>(
  ({ children }) => {
    const { user } = useUser();

    const { data: subscriptionData = null, refetch, isFetching, error } = useQuery({
      queryKey: ["push-notifications", user?.principal_id.toText() ?? '-', 'data'],
      queryFn: async () => {
        if (!user || !("serviceWorker" in navigator)) return null;

        const {
          data: { publicKey },
        } = await axios.get(`${APIUrl}/get-public-key`);

        const registration = await navigator.serviceWorker.register(
          !IsDev ? "/sw.js" : "/dev-sw.js?dev-sw",
          { type: !IsDev ? "classic" : "module" },
        );

        let subscription = await registration.pushManager.getSubscription();

        try {

          if (!subscription)
            subscription = await registration.pushManager.subscribe({
              userVisibleOnly: true,
              applicationServerKey: urlBase64ToUint8Array(publicKey),
            });

        } catch (error) {
          console.log({ error })
        }

        registration.onupdatefound = () => {
          const installingWorker = registration.installing;
          if (!installingWorker) return;
          installingWorker.onstatechange = () => {
            if (installingWorker.state === "installed") {
              if (navigator.serviceWorker.controller) {
                console.log("New or updated content is available.");
              } else {
                console.log("Content is cached for offline use.");
              }
            }
          };
        };

        await queryClient.invalidateQueries({
          queryKey: ["notifications-status", user.principal_id.toText()],
        });

        return subscription;
      },
      retry: false,
    })

    // const { error, data, refetch, isFetching } = useQuery({
    //   queryKey: queryKey ?? ["push-notifications"],
    //   queryFn: async () => {
    //     if (!user || !("serviceWorker" in navigator)) return;

    //     const {
    //       data: { publicKey },
    //     } = await axios.get(`${APIUrl}/get-public-key`);

    //     const registration = await navigator.serviceWorker.register(
    //       !IsDev ? "/sw.js" : "/dev-sw.js?dev-sw",
    //       { type: !IsDev ? "classic" : "module" },
    //     );

    //     let subscription = await registration.pushManager.getSubscription();

    //     try {

    //       if (!subscription)
    //         subscription = await registration.pushManager.subscribe({
    //           userVisibleOnly: true,
    //           applicationServerKey: urlBase64ToUint8Array(publicKey),
    //         });

    //     } catch (error) {
    //       console.log({ error })
    //     }

    //     registration.onupdatefound = () => {
    //       const installingWorker = registration.installing;
    //       if (!installingWorker) return;
    //       installingWorker.onstatechange = () => {
    //         if (installingWorker.state === "installed") {
    //           if (navigator.serviceWorker.controller) {
    //             console.log("New or updated content is available.");
    //           } else {
    //             console.log("Content is cached for offline use.");
    //           }
    //         }
    //       };
    //     };

    //     await queryClient.invalidateQueries({
    //       queryKey: ["notifications-status", user.principal_id.toText()],
    //     });

    //     return subscription;
    //   },
    //   retry: false,
    //   enabled: !!queryKey,
    // });

    const { data: enabled = false, isFetching: isLoadingEnabled } = useQuery({
      queryKey: ["notifications-status", user?.principal_id.toText()],
      queryFn: async (): Promise<boolean> => {
        if (!user) return false;
        try {
          return !!(await axios.get(
            `${APIUrl}/notification/${user.principal_id.toText()}`
          )).data;
        } catch {
          return false;
        }
      },
      enabled: !!user,
      retry: false,
    });

    const { mutate: setEnabled, isPending: isMutatingEnabled } = useMutation({
      mutationFn: async (new_enabled: boolean) => {
        if (!user || new_enabled === enabled) return;
        if (new_enabled) {
          await axios.post(`${APIUrl}/notification/${user.principal_id.toText()}`, subscriptionData);
        } else {
          await axios.delete(`${APIUrl}/notification/${user.principal_id.toText()}`);
        }
        await queryClient.invalidateQueries({
          queryKey: ["notifications-status", user.principal_id.toText()],
        });
      },
      retry: false,
    });

    return (
      <Context.Provider
        value={{
          isConnecting: isFetching,
          connect: refetch,
          error,
          isConnected: !!subscriptionData || false,
          isLoadingEnabled: isLoadingEnabled || isMutatingEnabled,
          enabled: enabled && (!!subscriptionData || false),
          setEnabled,
        }}
      >
        {children}
      </Context.Provider>
    );
  },
);
ProvideNotifications.displayName = "ProvideNotifications";
