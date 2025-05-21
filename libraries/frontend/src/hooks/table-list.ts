import { useMemo } from "react";

import { createActor as createTableActor } from "@declarations/table_canister";
import { PublicTable } from "@declarations/table_index/table_index.did";
import { Principal } from "@dfinity/principal";
import { Queries } from "@lib/data";
import { callActorMutation } from "@lib/utils/call-actor-mutation";
import { useQueries } from "@tanstack/react-query";

export const useTableList = (tableIds?: Principal[]) => {
  // Manually memoize the table queries because principals are not serializable
  const tableQueries = useMemo(() => {
    if (!tableIds) return [];
    return tableIds.map((id) => ({
      queryKey: Queries.table.key({ id }),
      queryFn: async (): Promise<PublicTable> =>
        await callActorMutation(createTableActor(id), "get_table"),
      refetchInterval: 10000,
    }));
  }, [JSON.stringify(tableIds)]);

  const tables = useQueries({
    queries: tableQueries,
    combine: (result) =>
      result.map((r) => r.data).filter((r): r is PublicTable => !!r),
  });

  return useMemo(() => tables, [JSON.stringify(tables)]);
};
