import { UserTableData } from "@declarations/table_canister/table_canister.did";
import { User } from "@declarations/users_canister/users_canister.did";
import { Ed25519KeyIdentity } from "@dfinity/identity";
import { Principal } from "@dfinity/principal";

export type PreviewUsers = "karl" | "karl-no-wallet" | "aaron";

export type CumulativeUserTableData = UserTableData & {
  principal_id: Principal;
  users_canister_id: Principal;
};

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
  u: Omit<Partial<CumulativeUserTableData>, "experience_points">
): CumulativeUserTableData => ({
  ...mockUser({}),
  player_action: { None: null },
  current_total_bet: 0n,
  total_bet: 0n,
  cards: [],
  show_card_requests: [],
  inactive_turns: 0,
  auto_check_fold: false,
  seated_out_turns: 0,
  experience_points: 0n,
  ...u,
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
  cumulative: CumulativeUserTableData;
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
    cumulative: mockCumulativeUserTableData(user),
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
      cumulative: CumulativeUserTableData;
    } => {
      const { user } = { ...AllMockUsers[i % AllMockUsers.length] };
      return modify(
        {
          user: {
            ...user,
            user_name: `${user.user_name}-${i}`,
          },
          cumulative: mockCumulativeUserTableData(user),
        },
        i
      );
    }
  );

  return users;
};
