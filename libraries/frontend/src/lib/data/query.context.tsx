import { FilterOptions, GameType, PublicTable } from '@declarations/table_index/table_index.did';
import { AccountIdentifier } from '@dfinity/ledger-icp';
import { Principal } from '@dfinity/principal';
import {
  Currency, CurrencyReceiver, CurrencyType, CurrencyTypeSerializer, WalletType
} from '@zk-game-dao/currency';
import { queryKeyFactory } from '@zk-game-dao/ui';

export { queryClient, ProvideQuery } from "@zk-game-dao/ui";

export const Queries = {
  icrc_allowance: queryKeyFactory((currencyType: CurrencyType) => ["icrc-allowance", CurrencyTypeSerializer.serialize(currencyType)]),
  lobby: queryKeyFactory((options: FilterOptions, range: [number, number]) => [
    "lobby",
    JSON.stringify(options),
    ...range,
  ]),
  chain_fusion_transaction_fees: queryKeyFactory((authenticated: boolean) => [
    "chain-fusion-transaction-fees",
    authenticated.toString(),
  ]),
  tournaments: queryKeyFactory((isBTC: boolean, type?: number) => ["tournaments", type ?? 'all', isBTC ? 'btc' : 'non-icp']),
  tournament: queryKeyFactory((tournament_principal?: Principal) => ["tournament", tournament_principal ? tournament_principal.toText() : "unknown"]),
  tournamentPrizePool: queryKeyFactory((tournament_principal?: Principal) => ["tournament-prize-pool", tournament_principal ? tournament_principal.toText() : "unknown"]),
  tournament_leaderboard: queryKeyFactory((tournament_principal?: Principal) => ["tournament-leaderboard", tournament_principal ? tournament_principal.toText() : "unknown"]),
  tournamentIsRunning: queryKeyFactory((tournament_principal?: Principal) => ["tournament-is-running", tournament_principal ? tournament_principal.toText() : "unknown"]),
  rakeInfo: queryKeyFactory((small_blind: bigint, currency: Currency, game_type: GameType) => [
    "rake-info",
    small_blind.toString(),
    CurrencyTypeSerializer.serialize({ Real: currency }),
    JSON.stringify(game_type),
  ]),
  table: queryKeyFactory((table?: Pick<PublicTable, "id">) => [
    "table-data",
    table?.id.toText() ?? "unknown",
  ]),
  tableLogStore: queryKeyFactory(
    (
      table_principal: Principal | undefined,
      start_timestamp: bigint,
      end_timestamp: bigint,
      offset: number,
      limit: number,
    ) => [
        "table-log-store",
        table_principal?.toText() ?? "unknown",
        start_timestamp.toString(),
        end_timestamp.toString(),
        offset.toString(),
        limit.toString(),
      ],
  ),
  // userFromCanisterId: queryKeyFactory((canisterId?: Principal) => [
  //   "user-from-canister-id",
  //   canisterId?.toText() ?? "unknown",
  // ]),
  // userCanisterIdFromAuthPrincipal: queryKeyFactory((principal?: Principal) => [
  //   "user-canister-id-auth-principal",
  //   principal?.toText() ?? "unknown",
  // ]),
  userFromUserId: queryKeyFactory((user_id?: Principal) => [
    "user-user-id",
    user_id?.toText() ?? "unknown",
  ]),
  userSelf: queryKeyFactory((user_id?: Principal) => [
    "user-self",
    user_id?.toText() ?? "unknown",
  ]),
  auth: queryKeyFactory(() => ["auth"]),
  transactionFee: queryKeyFactory((currencyType: CurrencyType) => ["transaction-fee", CurrencyTypeSerializer.serialize(currencyType)]),

  walletBalance: queryKeyFactory(
    (currencyType: CurrencyType, authData?: { accountIdentifier?: AccountIdentifier }) => [
      "wallet-balance",
      CurrencyTypeSerializer.serialize(currencyType),
      authData?.accountIdentifier?.toHex() ?? "unknown",
    ],
  ),

  _balanceModalBalance: queryKeyFactory((currencyType: CurrencyType) => [
    "balance-modal-balance",
    CurrencyTypeSerializer.serialize(currencyType),
  ]),

  preferredWallet: queryKeyFactory(
    (type: WalletType | "external", eternalAddress?: string) => [
      "preferred-wallet",
      type,
      eternalAddress ?? "-",
    ],
  ),

  leaderboard: queryKeyFactory((type: 'verified' | 'all', page: bigint, pageSize: bigint) => [
    "leaderboard", type, page.toString(), pageSize.toString()
  ]),
  leaderboardSize: queryKeyFactory((type: 'verified' | 'all') => ["leaderboard-size", type]),

  userExperiencePoints: queryKeyFactory((canisterId?: Principal) => [
    "user-experience-points",
    canisterId?.toText() ?? "unknown",
  ]),

  allowance: queryKeyFactory((currency?: CurrencyType, receiver?: CurrencyReceiver, wallet?: Principal) => [
    "allowance",
    currency ? CurrencyTypeSerializer.serialize(currency) : '-',
    receiver ? JSON.stringify(receiver) : '-',
    wallet?.toText() ?? '-',
  ]),
};
