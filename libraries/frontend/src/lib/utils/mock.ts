import { TableConfig } from "@declarations/table_canister/table_canister.did";
import { PublicTable } from "@declarations/table_index/table_index.did";
import { User } from "@declarations/users_canister/users_canister.did";
import { Ed25519KeyIdentity } from "@dfinity/identity";
import { Principal } from "@dfinity/principal";
import * as ctx from "@lib/table/context/table.context";
import { clearAllMocks } from "@storybook/test";

export type PreviewUsers = "karl" | "karl-no-wallet" | "aaron";

export const mockUser = (u: Partial<User>): User => ({
  users_canister_id: Principal.anonymous(),
  principal_id: Principal.anonymous(),
  user_name: "karl",
  balance: BigInt(100 * 10 ** 8),
  address: [],
  avatar: [{ Emoji: { emoji: 0n, style: 0n } }],
  active_tables: [],
  enlarge_text: [false],
  volume_level: [],
  transaction_history: [],
  experience_points: [10n],
  experience_points_pure_poker: [10n],
  eth_wallet_address: [],
  is_verified: [false],
  referrer: [],
  referred_users: [],
  referral_start_date: [],
  ...u,
});

export const mockCumulativeUserTableData = (
  u: Omit<Partial<ctx.SeatMetaData>, "data" | "user"> & {
    data?: Partial<ctx.SeatMetaData["data"]>;
    user?: Partial<User>;
  }
): ctx.SeatMetaData => ({
  status: { Empty: null },
  ...u,
  user: mockUser(u.user ?? {}),
  data: {
    player_action: { None: null },
    current_total_bet: 0n,
    total_bet: 0n,
    cards: [],
    show_card_requests: [],
    inactive_turns: 0,
    auto_check_fold: false,
    seated_out_turns: 0,
    experience_points: 0n,
    ...u.data,
  },
});

export const PREVIEW_USERS: {
  [key in PreviewUsers]: { user: User };
} = {
  "karl-no-wallet": {
    user: mockUser({
      user_name: "karl",
      balance: BigInt(100 * 10 ** 8),
      experience_points: [100n],
    }),
  },
  karl: {
    user: mockUser({
      user_name: "👑karl👑",
      balance: BigInt(100 * 10 ** 8),
      address: [Principal.anonymous().toString()],
    }),
  },
  aaron: {
    user: mockUser({
      user_name: "👉_aaron",
      balance: BigInt(1000 * 10 ** 8),
    }),
  },
};

type Data = {
  user: User;
  cumulative: ctx.SeatMetaData;
};

const templateUsers = [PREVIEW_USERS.karl, PREVIEW_USERS.aaron];
export const AllMockUsers: Data[] = Array.from({ length: 10 }, (_, i) => {
  const { user } = templateUsers[i % templateUsers.length];
  user.avatar = [{ Emoji: { emoji: BigInt(i), style: BigInt(i % 8) } }];
  return {
    user: {
      ...user,
      canister_id: Ed25519KeyIdentity.generate().getPrincipal(),
      internet_identity_principal_id:
        Ed25519KeyIdentity.generate().getPrincipal(),
      user_name: `${user.user_name}-${i}`,
    },
    cumulative: mockCumulativeUserTableData({ user }),
  };
});

export const mockUserArray = (
  count: number,
  modify: (user: Data, index: number) => Data = (user) => user
): Data[] => {
  const users = Array.from(
    { length: count },
    (
      _,
      i
    ): {
      user: User;
      cumulative: ctx.SeatMetaData;
    } => {
      const { user } = { ...AllMockUsers[i % AllMockUsers.length] };
      return modify(
        {
          user: {
            ...user,
            user_name: `${user.user_name}-${i}`,
          },
          cumulative: mockCumulativeUserTableData({ user }),
        },
        i
      );
    }
  );

  clearAllMocks();

  return users;
};

export const mockTable = ({
  config,
  ...data
}: Partial<
  Omit<PublicTable, "config"> & { config: Partial<TableConfig> }
> = {}): PublicTable => {
  const game_type = config?.game_type || { NoLimit: BigInt(1 * 10 ** 8) };
  const small_blind =
    "NoLimit" in game_type ? game_type.NoLimit : BigInt(10 ** 8);
  const big_blind =
    "NoLimit" in game_type ? game_type.NoLimit * 2n : BigInt(2 * 10 ** 8);

  return {
    id: Principal.anonymous(),
    pot: 0n,
    user_table_data: [],
    status: { Open: null },
    deal_stage: { Opening: null },
    // game_type,
    big_blind,
    small_blind,
    sorted_users: [],
    last_timer_started_timestamp: 0n,
    action_logs: [],
    last_raise: 0n,
    seats: [],
    round_ticker: 0n,
    community_cards: [],
    current_player_index: 0n,
    dealer_position: 0n,
    highest_bet: 0n,
    side_pots: [],
    winners: [],
    users: { users: [] },
    queue: [],
    config: {
      name: "",
      color: 0n,
      card_color: 0n,
      environment_color: 0n,
      game_type: { NoLimit: BigInt(1 * 10 ** 8) },
      timer_duration: 0,
      seats: 0,
      auto_start_timer: 0,
      max_inactive_turns: 4,
      currency_type: { Real: { ICP: null } } as any,
      enable_rake: [false],
      max_seated_out_turns: [20],
      is_private: [false],
      ante_type: [],
      table_type: [],
      is_shared_rake: [],
      require_proof_of_humanity: [],
      is_paused: [],
      ...config,
    },
    ...data,
  };
};

export const mockTableContext = (
  data?: Partial<ctx.TableContextValue>
): ctx.TableContextValue => ({
  table: mockTable(),
  isJoined: false,
  isOngoing: false,
  url: "",
  currentBet: 0n,
  currencyType: { Real: { ICP: null } },
  receiver: { principal: Principal.anonymous() },

  actor: null as any,
  getSeat: () => ({ status: { Empty: null } }),
  users: [],
  ...data,
});

function principalFromU64(n: bigint): Principal {
  const bytes = new Uint8Array(9); // 1 tag byte + 8 bytes for u64
  bytes[0] = 0x2a; // Self-authenticating or custom tag byte
  const view = new DataView(bytes.buffer);
  view.setBigUint64(1, n, false); // big endian
  return Principal.fromUint8Array(bytes);
}

export const mockPrincipal = (number: number | bigint) =>
  principalFromU64(BigInt(number));
