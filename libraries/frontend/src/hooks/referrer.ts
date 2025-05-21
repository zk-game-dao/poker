import { Principal } from "@dfinity/principal";
import { useEffect, useMemo, useState } from "react";

export const useReferrer = () => {
  const referrerInUrl = new URLSearchParams(window.location.search).get(
    "referrer"
  );

  const referrer = localStorage.getItem("referrer") || referrerInUrl;

  useEffect(() => {
    if (referrerInUrl && !localStorage.getItem("referrer")) {
      localStorage.setItem("referrer", referrerInUrl);
    }
  }, [referrerInUrl]);

  return useMemo(
    () => (referrer ? Principal.fromText(referrer) : undefined),
    [referrer]
  );
};
