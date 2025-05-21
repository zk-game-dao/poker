import { useQuery } from "@tanstack/react-query";
import { formatDistance } from "date-fns";
import { useMemo } from "react";

export const useFormatDateDistance = (to?: Date) => {
  const query = useQuery({
    queryKey: ["format-date-distance", to],
    queryFn: async () => {
      if (!to) return null;
      const now = Date.now();
      return { string: formatDistance(to, now), number: to.getTime() - now };
    },
    refetchInterval: 1000,
    initialData: to && { string: formatDistance(to, Date.now()), number: to.getTime() - Date.now() },
  });

  return useMemo(() => query.data || undefined, [query.data]);
};
