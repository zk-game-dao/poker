import axios, { AxiosError } from "axios";
import { useMemo } from "react";

// import swUrl from '@/src/sw.ts?url';
import { User } from "@declarations/users_canister/users_canister.did";
import { useQuery } from "@tanstack/react-query";
import { IsDev } from "@zk-game-dao/ui";

import { APIUrl } from "../lib/env/constants";

export const useServiceWorkerRegistration = (user?: User) => {
  const queryKey = useMemo(
    () =>
      !user ? undefined : ["push-notifications", user?.principal_id.toText()],
    [user]
  );

  const { error, data, refetch } = useQuery({
    queryKey: queryKey ?? ["push-notifications"],
    queryFn: async () => {
      if (!user || !("serviceWorker" in navigator)) return;

      const {
        data: { publicKey },
      } = await axios.get(`${APIUrl}/notifications/get-public-key`);

      const registration = await navigator.serviceWorker.register(
        !IsDev ? "/sw.js" : "/dev-sw.js?dev-sw",
        { type: !IsDev ? "classic" : "module" }
      );

      let subscription = await registration.pushManager.getSubscription();

      if (!subscription) {
        subscription = await registration.pushManager.subscribe({
          userVisibleOnly: true,
          applicationServerKey: urlBase64ToUint8Array(publicKey),
        });
      }

      try {
        await axios.post(`${APIUrl}/notifications/register-user`, {
          subscription,
          user_canister_principal_str: user.principal_id.toText(),
        });
      } catch (error) {
        if (!(error instanceof AxiosError) || error.response?.status !== 400)
          throw error;

        // This part is redundand and should be handled smarter
        await subscription.unsubscribe();
        subscription = await registration.pushManager.subscribe({
          userVisibleOnly: true,
          applicationServerKey: urlBase64ToUint8Array(publicKey),
        });
        await axios.post(`${APIUrl}/notifications/register-user`, {
          subscription,
          user_canister_principal_str: user.principal_id.toText(),
        });
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

      return true;
    },
    enabled: !!queryKey,
    retry: false,
  });

  return useMemo(
    () => ({
      connect: refetch,
      error,
      connected: data,
    }),
    [error, refetch, data]
  );
};

// Helper function to convert the VAPID public key to the required format
function urlBase64ToUint8Array(base64String: string) {
  const padding = "=".repeat((4 - (base64String.length % 4)) % 4);
  const base64 = (base64String + padding).replace(/-/g, "+").replace(/_/g, "/");
  const rawData = window.atob(base64);
  return Uint8Array.from([...rawData].map((char) => char.charCodeAt(0)));
}
