import classNames from 'classnames';
import { memo, useMemo } from 'react';

import { Card, Rank } from '@declarations/table_canister/table_canister.did';
import { useQuery } from '@tanstack/react-query';
import { ErrorComponent, LoadingAnimationComponent, Modal } from '@zk-game-dao/ui';

import { useTable } from '../../context/table.context';
import { CardComponent } from '../card/card.component';

const AllRankNames: Rank[] = [
  { StraightFlush: 0 },
  { FourOfAKind: 0 },
  { FullHouse: 0 },
  { Flush: 0 },
  { Straight: 0 },
  { ThreeOfAKind: 0 },
  { TwoPair: 0 },
  { OnePair: 0 },
  { HighCard: 0 },
];

type RankCard = {
  card: Card;
  /** Whether the card is relevant to the rank */
  isRelevant: boolean;
};

const RankComponent = memo<{ rank: Rank; highlighted: boolean }>(
  ({ rank, highlighted }) => {
    // The cards that make up the rank
    const rankCards = useMemo(():
      | {
        title: string;
        cards: [RankCard, RankCard, RankCard, RankCard, RankCard];
      }
      | undefined => {
      if ("StraightFlush" in rank)
        return {
          title: "Straight Flush",
          cards: [
            {
              card: { suit: { Diamond: null }, value: { Ace: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Diamond: null }, value: { Two: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Diamond: null }, value: { Three: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Diamond: null }, value: { Four: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Diamond: null }, value: { Five: null } },
              isRelevant: true,
            },
          ],
        };
      if ("Straight" in rank)
        return {
          title: "Straight",
          cards: [
            {
              card: { suit: { Diamond: null }, value: { Ace: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Club: null }, value: { Two: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Heart: null }, value: { Three: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Spade: null }, value: { Four: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Diamond: null }, value: { Five: null } },
              isRelevant: true,
            },
          ],
        };
      if ("OnePair" in rank)
        return {
          title: "One Pair",
          cards: [
            {
              card: { suit: { Diamond: null }, value: { Ace: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Club: null }, value: { Ace: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Heart: null }, value: { Three: null } },
              isRelevant: false,
            },
            {
              card: { suit: { Spade: null }, value: { Four: null } },
              isRelevant: false,
            },
            {
              card: { suit: { Diamond: null }, value: { Five: null } },
              isRelevant: false,
            },
          ],
        };
      if ("FullHouse" in rank)
        return {
          title: "Full House",
          cards: [
            {
              card: { suit: { Diamond: null }, value: { King: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Club: null }, value: { King: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Heart: null }, value: { King: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Spade: null }, value: { Six: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Diamond: null }, value: { Six: null } },
              isRelevant: true,
            },
          ],
        };
      if ("TwoPair" in rank)
        return {
          title: "Two Pair",
          cards: [
            {
              card: { suit: { Diamond: null }, value: { King: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Club: null }, value: { King: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Heart: null }, value: { Six: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Spade: null }, value: { Six: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Diamond: null }, value: { Five: null } },
              isRelevant: false,
            },
          ],
        };
      if ("HighCard" in rank)
        return {
          title: "High Card",
          cards: [
            {
              card: { suit: { Diamond: null }, value: { Ace: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Club: null }, value: { King: null } },
              isRelevant: false,
            },
            {
              card: { suit: { Heart: null }, value: { Six: null } },
              isRelevant: false,
            },
            {
              card: { suit: { Spade: null }, value: { Four: null } },
              isRelevant: false,
            },
            {
              card: { suit: { Diamond: null }, value: { Three: null } },
              isRelevant: false,
            },
          ],
        };
      if ("ThreeOfAKind" in rank)
        return {
          title: "Three of a Kind",
          cards: [
            {
              card: { suit: { Diamond: null }, value: { Ace: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Club: null }, value: { Ace: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Heart: null }, value: { Ace: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Spade: null }, value: { Four: null } },
              isRelevant: false,
            },
            {
              card: { suit: { Diamond: null }, value: { Three: null } },
              isRelevant: false,
            },
          ],
        };
      if ("Flush" in rank)
        return {
          title: "Flush",
          cards: [
            {
              card: { suit: { Diamond: null }, value: { Ace: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Diamond: null }, value: { King: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Diamond: null }, value: { Six: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Diamond: null }, value: { Four: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Diamond: null }, value: { Three: null } },
              isRelevant: true,
            },
          ],
        };
      if ("FourOfAKind" in rank)
        return {
          title: "Four of a Kind",
          cards: [
            {
              card: { suit: { Diamond: null }, value: { Ace: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Club: null }, value: { Ace: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Heart: null }, value: { Ace: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Spade: null }, value: { Ace: null } },
              isRelevant: true,
            },
            {
              card: { suit: { Diamond: null }, value: { Three: null } },
              isRelevant: false,
            },
          ],
        };
      return undefined;
    }, [rank]);

    return (
      <tr>
        <td
          className={classNames("p-[2px] pr-4 text-right rounded-l-[8px]", {
            "bg-material-main-1": highlighted,
          })}
        >
          {rankCards?.title}
        </td>
        {rankCards?.cards.map(({ card, isRelevant }, i, arr) => (
          <td
            key={i}
            className={classNames("p-[2px]", {
              "bg-material-main-1": highlighted,
              "rounded-r-[8px]": i === arr.length - 1,
            })}
          >
            <CardComponent
              card={card}
              size="between-microscopic-and-small"
              className={classNames({ "opacity-50": !isRelevant })}
            />
          </td>
        ))}
      </tr>
    );
  },
);
RankComponent.displayName = "RankComponent";

export const RankCardsModalComponent = memo<{
  isOpen: boolean;
  onClose(): void;
}>(({ isOpen, onClose }) => {
  const { user, table, actor: service } = useTable();

  const { data, isFetching, error } = useQuery({
    queryKey: ["rank-cards", user?.data?.cards, table?.community_cards],
    queryFn: async () => {
      if (!user || !user.data) return;
      const response = await service.rank_cards([
        ...user.data.cards,
        ...(table?.community_cards || []),
      ]);
      if ("Err" in response) throw response.Err;
      return response.Ok;
    },
    enabled: !!user,
  });

  return (
    <Modal
      open={isOpen}
      onClose={onClose}
      title="Hands"
      contentClassName="pb-4"
    >
      {data && (
        <table>
          <colgroup>
            <col style={{ width: "100%" }} />
            <col span={5} />
          </colgroup>
          <tbody>
            {AllRankNames.map((rank) => (
              <RankComponent
                key={JSON.stringify(rank)}
                rank={rank}
                highlighted={Object.keys(data)[0] === Object.keys(rank)[0]}
              />
            ))}
          </tbody>
        </table>
      )}
      <ErrorComponent error={error} />
      {isFetching && (
        <LoadingAnimationComponent>Cards</LoadingAnimationComponent>
      )}
    </Modal>
  );
});
RankCardsModalComponent.displayName = "RankCardsModalComponent";
