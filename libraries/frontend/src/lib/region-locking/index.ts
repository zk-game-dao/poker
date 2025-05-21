import { useMemo } from "react";
import { useQuery } from "@tanstack/react-query";
import axios from "axios";
import { ISO_3166_1Codes } from "./data";

export interface IPAPIResponse {
  ip: string;
  hostname: string;
  city: string;
  region: string;
  // ISO 3166-1 alpha-2 code
  country: string;
  loc: string;
  org: string;
  postal: string;
  timezone: string;
}

export const useRegionLockingQuery = () => {
  const query = useQuery({
    queryKey: ["region-locking"],
    queryFn: async () => {
      const { data } = await axios.get<IPAPIResponse>("https://ipinfo.io/json?token=d1ad21fa45007f");
      return {
        isBlocked: ISO_3166_1Codes.includes(data.country),
        ...data,
      };
    },
  });

  return query;
};

export const useIsRegionLocked = () => {
  const { data } = useRegionLockingQuery();

  return useMemo(() => data?.isBlocked ?? false, [data?.isBlocked]);
};
