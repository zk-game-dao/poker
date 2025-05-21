import { PublicTable } from "@declarations/table_index/table_index.did";
import { TableConfig } from "@declarations/table_canister/table_canister.did";
import { Principal } from "@dfinity/principal";

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
