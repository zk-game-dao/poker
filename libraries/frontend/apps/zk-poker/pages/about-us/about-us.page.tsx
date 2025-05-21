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
import { FloatToTokenAmount } from '@lib/utils/token-amount-conversion';
import { ButtonComponent, LayoutComponent } from '@zk-game-dao/ui';

const Element = memo<{
  icon?: string;
  title: string;
  text: string;
  variant?: "default" | "green";
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
      title: "Poker, minus the trust issues.",
      subTitle:
        "Play poker online in a new, safer way. Our platform runs on blockchain technology, meaning no middlemen, just you and other players.",
      ctas: [
        { children: 'Create table', onClick: () => setModalOpen(true) },
        { children: 'Go to lobby', href: '/', filled: true }
      ],
      image: {
        type: 'png',
        src: '/images/hero-desktop.png',
        sizes: 1,
        alt: 'About us',
        className: 'scale-[120%]',
        width: 125,
        height: 59,
      },
      mobileImage: {
        type: 'png',
        src: '/images/hero-mobile.png',
        alt: 'About us',
        width: 344,
        height: 381,
      }
    }}>
      {/* <AboutUsHeroComponent openCreateTableModal={() => setModalOpen(true)} /> */}

      <div className="container mx-auto mb-24 flex flex-col gap-[120px] mt-16">

        <div className="flex flex-col lg:flex-row items-start gap-8 justify-center">
          <Element
            icon="/icons/peer-to-peer-green.svg"
            title="Peer-to-peer"
            variant="green"
            text="Real-life play with digital assets, securely transacting bets between players."
          />
          <Element
            icon="/icons/instant-payouts-green.svg"
            title="Instant payouts"
            variant="green"
            text="Smart contracts ensure almost instant deposits and withdrawals, executed on the blockchain."
          />
          <Element
            icon="/icons/multichain-green.svg"
            title="Multichain"
            variant="green"
            text="Fully on-chain with no central points of failure."
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
            title="Digital poker face"
            text="We utilize Zero Knowledge Proofs (ZKPs) for secure game outcomes, shuffling verification, hand verification, and enhanced privacy."
          />
        </div>

        <div className="flex flex-col lg:flex-row items-center">
          <Element
            title="The Store for any visual upgrade"
            text="Personalize your gaming experience with unique tables, decks, environments, and other items available as NFTs."
          />
          <div className="flex flex-col justify-center items-center w-full relative gap-16 my-16 lg:my-0">
            <div className="flex flex-col mr-20">
              <ChipsStackComponent
                currencyType={{ Real: { ICP: null } }}
                value={FloatToTokenAmount(15.12345, {
                  decimals: 8
                })}
              />
              <ChipsStackComponent
                currencyType={{ Real: { CKETHToken: { USDC: null } } }}
                value={FloatToTokenAmount(231.55331, {
                  decimals: 6
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

        <div className="flex flex-col lg:flex-row items-center">
          <div className="w-full flex justify-center items-center">
            <img src="/images/chat.png" className="max-w-[345px]" />
          </div>
          <Element
            icon="/icons/og.svg"
            title="OpenChat integration"
            text="Chat with other players seamlessly using OpenChat, integrated directly into our platform."
            href="https://oc.app/"
            cta="Visit oc.app"
          />
        </div>

        <div className="flex flex-col lg:flex-row items-center">
          <Element
            icon="/icons/icp.svg"
            title="Secure and seamless login"
            text="We use Internet Computer for our backend infrastructure, ensuring decentralization and scalability. Log in effortlessly with Internet Identity for a secure and private experience."
            href="https://internetcomputer.org/"
            cta="Visit internetcomputer.org"
          />
          <div className="w-full flex justify-center items-center">
            <img src="/images/ic.png" className="max-w-[345px]" />
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
