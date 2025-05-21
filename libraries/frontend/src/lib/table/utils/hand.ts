import {
  Card,
  PublicTable,
  UserTableData,
} from "@declarations/table_canister/table_canister.did";

export const BuildHand = (
  isSelf: boolean,
  table: Pick<PublicTable, "deal_stage" | "sorted_users" | "user_table_data">,
  data?: Pick<UserTableData, "cards" | "player_action">
): Card[] => {
  // Hide the cards for your own user
  if (!data?.cards || isSelf) return [];
  if ("Fresh" in table.deal_stage) return data.cards;
  const cards = new Array(2).fill(undefined).map((_, i) => data.cards[i]);
  const noCards = new Array(2).fill(undefined);
  if (!("Showdown" in table.deal_stage) || !table.sorted_users[0]?.length)
    return noCards;
  if ("Folded" in data.player_action || "SittingOut" in data.player_action)
    return noCards;

  const activePlayers = table.user_table_data.filter(
    (u) =>
      !("Folded" in u[1].player_action) &&
      !("SittingOut" in u[1]) &&
      !("Joining" in u[1].player_action)
  );

  // Check if is winning by default (everyone else folded or is sitting out)
  if (activePlayers.length === 1) return noCards;

  return cards;
};
