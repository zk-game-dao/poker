import "react-json-view-lite/dist/index.css";

import {
  _SERVICE,
  TableConfig,
  User,
} from "@declarations/table_canister/table_canister.did";
import {
  CurrencyType,
  PublicTable,
  SeatStatus,
  UserTableData,
} from "@declarations/table_index/table_index.did";
import { Principal } from "@dfinity/principal";
import { useUser } from "@lib/user";
import { CurrencyReceiver } from "@zk-game-dao/currency";
import { differenceInMilliseconds } from "date-fns";
import { createContext, useContext, useEffect, useMemo, useState } from "react";
import { useUserFromUserId } from "../../user/hooks/use-user";

/**
 * User is the user that is logged in
 * Player is a player on the table
 * Seat is a position on the table
 */
/**
 * - Freshly created table
 *   DealStage: 'Fresh'
 *
 * Deal Stage: 'Fresh'
 *  -> dealer calls start_new_betting_round (moves to deal stage 'Blinds')
 *
 * Deal Stage: 'Blinds'
 *  -> Go through each player_index and collect blinds by calling place_bet with the transaction block nbumber...
 *  -> call confirm_blinds (moves to deal stage 'Opening')
 *
 * Deal Stage: 'Opening'
 *
 *
 */
export type SeatMetaData = {
  status: SeatStatus;
  data?: UserTableData;
  user?: User;
  canister_id?: Principal;
};

export type TableContextValue = {
  actor: _SERVICE;
  table: PublicTable;
  isJoined: boolean;
  isOngoing: boolean;

  currencyType: CurrencyType;
  receiver: CurrencyReceiver;

  user?: SeatMetaData;
  userIndex?: bigint;
  url: string;

  /** The current bet, derrived from highest bet of the betting round */
  currentBet: bigint;

  getSeat(seatIndex: number): SeatMetaData;
  users: SeatMetaData[];
};

const TableContext = createContext<TableContextValue>({
  actor: null as any,
  table: null as any,
  isJoined: false,
  url: "",
  currentBet: 0n,
  isOngoing: false,
  currencyType: { Fake: null },
  receiver: { principal: Principal.anonymous() },
  getSeat: () => ({ status: { Empty: null } }),
  users: [],
});

export const useTable = () => useContext(TableContext);

export const { Provider: ProvideTable, Consumer: ConsumeTable } = TableContext;

export const useTableUrl = (
  table?: Partial<
    Pick<PublicTable, "id"> &
      Partial<{ config: Partial<Pick<TableConfig, "table_type">> }>
  >,
  fullUrl = true
) =>
  useMemo(() => {
    const origin = fullUrl ? window.location.origin : "";
    if (!table) return `${origin}/`;
    if (
      table.config?.table_type?.[0] &&
      "Tournament" in table.config.table_type[0]
    )
      return `${origin}/tournaments/${table.config.table_type[0].Tournament.tournament_id}/table/${table.id}`;
    return `${origin}/tables/${table.id}`;
  }, [table?.id]);

export const useCurrentTableTurnProgress = (enabled: boolean) => {
  const {
    table: {
      last_timer_started_timestamp,
      config: { timer_duration },
      sorted_users,
    },
    isOngoing,
  } = useTable();

  const hasWinners = useMemo(() => !!sorted_users[0], [sorted_users]);

  const [progress, setProgress] = useState<number>();

  useEffect(() => {
    if (!enabled || !last_timer_started_timestamp || hasWinners || !isOngoing) {
      setProgress(undefined);
      return;
    }

    const handle = () =>
      requestAnimationFrame(() => {
        const elapsed = differenceInMilliseconds(
          new Date(),
          new Date(Number(last_timer_started_timestamp / BigInt(1e6)) - 2000)
        );
        setProgress(
          Math.min(1, Math.max(0, elapsed / (timer_duration * 1000)))
        );
      });

    const interval = setInterval(handle, 200);

    handle();

    return () => clearInterval(interval);
  }, [enabled, last_timer_started_timestamp, timer_duration, hasWinners]);

  return useMemo(() => progress, [progress]);
};

export const useCurrentTableTurnProgressRemainder = (enabled: boolean) => {
  const progress = useCurrentTableTurnProgress(enabled);
  return useMemo(
    () =>
      enabled
        ? progress === undefined
          ? progress
          : Math.max(0, 1 - progress)
        : undefined,
    [progress, enabled]
  );
};

export const useNewRoundProgress = (enabled: boolean) => {
  const {
    table: {
      config: { auto_start_timer },
      action_logs,
    },
  } = useTable();

  const timerStarted = useMemo(
    () =>
      action_logs.find((v) =>
        "Stage" in v.action_type && "Showdown" in v.action_type.Stage.stage
          ? v
          : undefined
      )?.timestamp,
    [action_logs]
  );

  const [progress, setProgress] = useState<number>();

  useEffect(() => {
    if (!enabled || !timerStarted) {
      setProgress(undefined);
      return;
    }

    const handle = () =>
      requestAnimationFrame(() => {
        const elapsed = differenceInMilliseconds(
          new Date(),
          new Date(Number(timerStarted / BigInt(1e6)) + 2000)
        );
        setProgress(
          Math.min(1, Math.max(0, elapsed / (auto_start_timer * 1000)))
        );
      });

    const interval = setInterval(handle, 200);

    handle();

    return () => clearInterval(interval);
  }, [enabled, timerStarted, auto_start_timer]);

  return useMemo(() => progress, [progress]);
};

export const useTableUserFromCanisterId = (canister_id?: Principal) => {
  const { table } = useTable();
  const tableData = useMemo(() => {
    if (!table || !canister_id) return undefined;
    return table.user_table_data.find(
      ([id]) => id.compareTo(canister_id) === "eq"
    )?.[1];
  }, [table, canister_id]);

  const tableUser = useMemo(() => {
    if (!tableData || !canister_id) return undefined;
    return table?.users.users.find(
      ([id]) => id.compareTo(canister_id) === "eq"
    )?.[1];
  }, [table, canister_id]);

  const { data: user } = useUserFromUserId(canister_id);

  return useMemo(
    () =>
      [
        tableUser
          ? ({
              ...tableUser,
              avatar: user?.avatar ?? tableUser?.avatar,
              user_name: user?.user_name ?? tableUser?.user_name,
            } as User)
          : undefined,
        tableData,
      ] as const,
    [user, tableUser, tableData]
  );
};

export const useMyTableUser = () => {
  const { user } = useUser();
  return useTableUserFromCanisterId(user?.principal_id);
};
