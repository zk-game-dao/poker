import { memo, useMemo } from 'react';

import { ActionLog } from '@declarations/table_canister/table_canister.did';
import { CurrencyComponent } from '@zk-game-dao/currency';
import { LoadingAnimationComponent } from '@zk-game-dao/ui';

import { useTable } from '../../context/table.context';
import { CardComponent } from '../card/card.component';
import { useUserFromUserId } from '../../../user/hooks/use-user';

export const ActionLogComponent = memo<
  Pick<ActionLog, "action_type" | "user_principal"> & { expanded?: boolean }
>(({ expanded = false, action_type, user_principal }) => {
  const { currencyType: currency } = useTable();
  const relevantUser = useUserFromUserId(user_principal[0]);

  const userLabel = useMemo(
    () =>
      !relevantUser ? (
        <LoadingAnimationComponent className="mr-2" variant="shimmer">
          Loading user
        </LoadingAnimationComponent>
      ) : (
        relevantUser.data?.user_name
      ),
    [relevantUser],
  );

  if ("Bet" in action_type)
    return (
      <>
        {userLabel} placed a bet of{" "}
        <CurrencyComponent
          currencyType={currency}
          variant="inline"
          currencyValue={action_type.Bet.amount}
        />
      </>
    );

  if ("Win" in action_type)
    return (
      <>
        {userLabel} won{" "}
        <CurrencyComponent
          currencyType={currency}
          variant="inline"
          currencyValue={action_type.Win.amount}
        />
      </>
    );

  if ("Leave" in action_type) return <>{userLabel} left</>;

  if ("Call" in action_type) return <>{userLabel} called</>;

  if ("Raise" in action_type)
    return (
      <>
        {userLabel} raised to{" "}
        <CurrencyComponent
          currencyType={currency}
          variant="inline"
          currencyValue={action_type.Raise.amount}
        />
      </>
    );

  if ("Check" in action_type) return <>{userLabel} checked</>;

  if ("Folded" in action_type || "Fold" in action_type)
    return <>{userLabel} folded</>;

  if ("Join" in action_type) return <>{userLabel} joined</>;

  if ("BigBlind" in action_type) return <> {userLabel} placed big blind</>;

  if ("SmallBlind" in action_type) return <> {userLabel} placed small blind</>;

  if ("AllIn" in action_type)
    return (
      <>
        {" "}
        {userLabel} went all in with{" "}
        <CurrencyComponent
          currencyType={currency}
          variant="inline"
          currencyValue={action_type.AllIn.amount}
        />
      </>
    );

  if ("Stage" in action_type)
    return <>Stage {Object.keys(action_type.Stage.stage)[0]}</>;

  if ("Kicked" in action_type)
    return (
      <>
        {userLabel} was kicked "{action_type.Kicked.reason}"
      </>
    );

  if ("Reveal" in action_type) return <>{userLabel} revealed their cards</>;

  if ("PlayersHandsRankedMainPot" in action_type)
    return expanded ? (
      <>
        {action_type.PlayersHandsRankedMainPot.hands.map(
          ([name, cards, amount], i) => (
            <div key={name + i}>
              {name} won{" "}
              <CurrencyComponent currencyType={currency} variant="inline" currencyValue={amount} />{" "}
              with{" "}
              {cards.map((card, i) => (
                <CardComponent key={i} card={card} />
              ))}
            </div>
          ),
        )}
      </>
    ) : (
      <>
        {action_type.PlayersHandsRankedMainPot.hands
          .map((v) => v[0])
          .join(" and ")}{" "}
        won the pot
      </>
    );

  if ("PlayersHandsRankedSidePot" in action_type)
    return expanded ? (
      <>
        {action_type.PlayersHandsRankedSidePot.hands.map(
          ([name, cards, amount], i) => (
            <div key={name + i}>
              {name} won sidepot of{" "}
              <CurrencyComponent currencyType={currency} variant="inline" currencyValue={amount} />{" "}
              with{" "}
              {cards.map((card, i) => (
                <CardComponent key={i} card={card} />
              ))}
            </div>
          ),
        )}
      </>
    ) : (
      <>
        {action_type.PlayersHandsRankedSidePot.hands
          .map((v) => v[0])
          .join(" and ")}{" "}
        won the side pot
      </>
    );

  if ("SidePotCreated" in action_type) return <>Side pot created</>;

  return <>Unknown action {Object.keys(action_type)[0]}</>;
});
ActionLogComponent.displayName = "ActionLogComponent";
