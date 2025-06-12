import classNames from 'classnames';
import { memo, useState } from 'react';

import { CardColor, TableColor } from '@/src/models/table-color.model';
import { CardComponent } from '@lib/table/components/card/card.component';
import { ChipsStackComponent } from '@lib/table/components/chips-stack/chips-stack.component';
import {
  CreateTableModalComponent
} from '@lib/table/components/create-table-modal/create-table-modal.component';
import {
  TableBackgroundComponent
} from '@lib/table/components/table/table-background/table-background.component';
import { ProvideTableVisuals } from '@lib/table/context/table-visuals.context';
import { FloatToTokenAmount } from '@zk-game-dao/currency';
import { ButtonComponent, LayoutComponent } from '@zk-game-dao/ui';
import Big from 'big.js';

import Preview from './preview.png';

const Element = memo<{
  icon?: string;
  title: string;
  text: string;
  variant?: "default" | 'purple' | "green";
  className?: string;
  href?: string;
  cta?: string;
}>(
  ({
    icon,
    title,
    text,
    variant = "default",
    className,
    href,
    cta,
  }) => (
    <div
      className={classNames(
        "flex flex-col items-center gap-4 max-w-[650px] text-center w-full mx-auto text-material-medium-2",
        className,
      )}
    >
      {icon && <img src={icon} alt={title} />}
      <p
        className={classNames("type-top", {
          "text-green-500": variant === "green",
          "text-white": variant === "default",
          "text-purple-500": variant === "purple",
        })}
      >
        {title}
      </p>
      <p className={classNames("type-header")}>{text}</p>
      {href && cta && (
        <ButtonComponent href={href} variant="naked" isOutLink>
          {cta}
        </ButtonComponent>
      )}
    </div>
  ),
);
Element.displayName = "Element";

const CardStacks = memo<{ className?: string; color: CardColor }>(
  ({ color, className }) => (
    <ProvideTableVisuals cardColor={color}>
      <div
        className={classNames(
          "flex flex-row justify-center items-end gap-2",
          className,
        )}
      >
        {[10, 5].map((value) => (
          <div
            className="flex flex-col relative"
            style={{ paddingTop: value * 2 }}
            key={value}
          >
            {Array.from({ length: value }, (_, index) => (
              <CardComponent
                key={index}
                size="large"
                className={classNames({
                  absolute: index > 0,
                })}
                style={{ bottom: `${index * 2}px` }}
              />
            ))}
          </div>
        ))}
      </div>
    </ProvideTableVisuals>
  ),
);
CardStacks.displayName = "CardStacks";

export const AboutUsPage = memo(() => {
  const [isModalOpen, setModalOpen] = useState(false);

  return (
    <LayoutComponent footer hero={{
      title: "Pure Poker",
      subTitle: "The Worlds First Fully Onchain Bitcoin Poker Room",
      ctas: [
        { children: 'Create table', onClick: () => setModalOpen(true) },
        { children: 'Go to lobby', href: '/cash-games', filled: true }
      ],
      image: {
        type: 'png',
        src: Preview,
        sizes: 1,
        alt: 'About us',
        className: 'scale-[120%]',
        width: 125,
        height: 59,
      },
      // mobileImage: {
      //   type: 'png',
      //   src: Preview,
      //   alt: 'About us',
      //   width: 344,
      //   height: 381,
      // }
    }}>
      {/* <AboutUsHeroComponent openCreateTableModal={() => setModalOpen(true)} /> */}

      <div className="container mb-24 flex flex-col gap-[120px] mt-16 mx-auto">

        <div className="flex flex-col lg:flex-row items-start gap-8 justify-center">
          <Element
            icon="/icons/peer-to-peer-green.svg"
            title="Provable Fair Gamplay"
            variant="green"
            text="Card shuffling and RNG derived from Onchain Randomness (VRF)"
          />
          <Element
            icon="/icons/instant-payouts-green.svg"
            title="Native BTC Deposits & Withdrawals"
            variant="green"
            text="Smart contracts ensure secure deposits and withdrawals, executed directly on the bitcoin network"
          />
          <Element
            icon="/icons/multichain-green.svg"
            title="Anti-Bot"
            variant="green"
            text="We utilise proof of humanity systems (through DecideAI PoH) to element bots playing on our site"
          />
        </div>

        <div className="flex flex-col gap-8 lg:flex-row items-center">
          <div className="flex flex-row justify-center items-center w-full">
            <div className="flex flex-col gap-6">
              <ProvideTableVisuals cardColor={CardColor.Red}>
                <div className="flex flex-row justify-center items-center">
                  <CardComponent
                    card={{ suit: { Club: null }, value: { Ace: null } }}
                    size="large"
                    className="-rotate-[15deg] translate-y-[16px] translate-x-[20px]"
                  />
                  <CardComponent
                    card={{ suit: { Heart: null }, value: { King: null } }}
                    size="large"
                    className="-rotate-[3deg] translate-x-[4px]"
                  />
                  <CardComponent
                    card={{ suit: { Spade: null }, value: { Six: null } }}
                    size="large"
                    className="rotate-[3deg] -translate-x-[4px]"
                  />
                  <CardComponent
                    card={{ suit: { Diamond: null }, value: { King: null } }}
                    size="large"
                    className="rotate-[15deg] translate-y-[16px] -translate-x-[20px]"
                  />
                </div>
                <CardStacks color={CardColor.Red} />
              </ProvideTableVisuals>
            </div>
          </div>
          <Element
            title="Your keys, your sats"
            text="We never have custody over your funds. Your wallet is bound to your login, meaning you remain in full control at all times."
          />
        </div>

        <div className="flex flex-col lg:flex-row items-center">
          <Element
            title="The Store for any visual upgrade"
            text="Personalize your gaming experience with unique tables, decks, environments, and other items available as NFTs and purchaseables."
          />
          <div className="flex flex-col justify-center items-center w-full relative gap-16 my-16 lg:my-0">
            <div className="flex flex-col mr-20">
              <ChipsStackComponent
                currencyType={{ Real: { BTC: null } }}
                value={FloatToTokenAmount(Big(0.12345), {
                  decimals: 8,
                  thousands: 10 ** 8,
                  transactionFee: 10000n,
                  isFetched: true,
                  symbol: 'BTC'
                })}
              />
            </div>
            <div className="flex flex-row">
              <CardStacks
                className="relative z-1 -rotate-[2deg]"
                color={CardColor.Red}
              />
              <CardStacks
                className="relative z-1 -translate-x-[30%] translate-y-[30%] rotate-[2deg]"
                color={CardColor.Green}
              />
            </div>
            <TableBackgroundComponent
              className="absolute z-0 left-[70%]"
              visuals={{ color: TableColor.Blue }}
            />
            <TableBackgroundComponent
              className="absolute z-0 translate-y-[20%] scale-90 left-[80%]"
              visuals={{ color: TableColor.Purple }}
            />
          </div>
        </div>
      </div>

      <CreateTableModalComponent
        open={isModalOpen}
        onCancel={() => setModalOpen(false)}
      />
    </LayoutComponent>
  );
});
AboutUsPage.displayName = "AboutUsPage";

export default AboutUsPage;
