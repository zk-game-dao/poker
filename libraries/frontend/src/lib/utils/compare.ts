import {
  Card,
  GameType,
  PlayerAction,
  Suit,
  UserAvatar,
  Value,
} from "@declarations/table_index/table_index.did";
import {
  PayoutPercentage,
  TournamentState,
} from "@declarations/tournament_canister/tournament_canister.did";

import { JoinType } from "../tournament/context/tournament.context";

export const IsSameAvatar = (a?: UserAvatar, b?: UserAvatar) =>
  a?.Emoji.emoji === b?.Emoji.emoji && a?.Emoji.style === b?.Emoji.style;

export const IsSameCardSuit = (a?: Suit, b?: Suit) =>
  JSON.stringify(a) === JSON.stringify(b);

export const IsSameCardValue = (a?: Value, b?: Value) =>
  JSON.stringify(a) === JSON.stringify(b);

export const IsSameCard = (a?: Card, b?: Card) =>
  IsSameCardSuit(a?.suit, b?.suit) && IsSameCardValue(a?.value, b?.value);

export const IsSameHand = (
  a?: (Card | undefined)[],
  b?: (Card | undefined)[]
) =>
  !!(
    a?.length === b?.length &&
    a?.every((card, index) => IsSameCard(card, b?.[index])) &&
    b?.every((card, index) => IsSameCard(card, a?.[index]))
  );

export const IsSameTournamentState = (
  a?: TournamentState,
  b?: TournamentState
) => {
  if (a === undefined && b === undefined) return true;
  if (a === undefined || b === undefined) return false;

  if ("Running" in a) return "Running" in b;
  if ("Registration" in a) return "Registration" in b;
  if ("FinalTable" in a) return "FinalTable" in b;
  if ("LateRegistration" in a) return "LateRegistration" in b;
  if ("Cancelled" in a) return "Cancelled" in b;
  if ("Completed" in a) return "Completed" in b;

  return false;
};

export const IsSameTournamentJoinType = (a?: JoinType, b?: JoinType) =>
  a?.type === b?.type && a?.amount === b?.amount;

export const IsSamePayoutPercentage = (
  a?: PayoutPercentage,
  b?: PayoutPercentage
) => a?.percentage === b?.percentage && a?.position === b?.position;

export const IsSamePayoutStructure = (
  a?: PayoutPercentage[],
  b?: PayoutPercentage[]
) => {
  if (a === undefined && b === undefined) return true;
  if (a === undefined || b === undefined) return false;

  if (a.length !== b.length) return false;
  return (
    a.every((payout, index) => IsSamePayoutPercentage(payout, b[index])) &&
    b.every((payout, index) => IsSamePayoutPercentage(payout, a[index]))
  );
};

export const IsSamePlayerAction = (a?: PlayerAction, b?: PlayerAction) => {
  if (a === undefined && b === undefined) return true;
  if (a === undefined || b === undefined) return false;

  if ("Joining" in a) return "Joining" in b;
  if ("Folded" in a) return "Folded" in b;
  if ("None" in a) return "None" in b;
  if ("SittingOut" in a) return "SittingOut" in b;
  if ("AllIn" in a) return "AllIn" in b;
  if ("Checked" in a) return "Checked" in b;
  if ("Raised" in a) return "Raised" in b && a.Raised === b.Raised;
  if ("Called" in a) return "Called" in b;

  return false;
};

export const IsSameGameType = (a?: GameType, b?: GameType) => {
  if (a === undefined && b === undefined) return true;
  if (a === undefined || b === undefined) return false;

  if ("NoLimit" in a) return "NoLimit" in b && a.NoLimit === b.NoLimit;
  if ("PotLimit" in a) return "PotLimit" in b && a.PotLimit === b.PotLimit;
  if ("FixedLimit" in a)
    return (
      "FixedLimit" in b &&
      a.FixedLimit[0] === b.FixedLimit[0] &&
      a.FixedLimit[1] === b.FixedLimit[1]
    );

  if ("SpreadLimit" in a)
    return (
      "SpreadLimit" in b &&
      a.SpreadLimit[0] === b.SpreadLimit[0] &&
      a.SpreadLimit[1] === b.SpreadLimit[1]
    );

  return false;
};
