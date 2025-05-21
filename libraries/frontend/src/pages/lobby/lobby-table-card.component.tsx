import { useCopyToClipboard } from '#hooks/clipboard';
import classNames from 'classnames';
import { formatDuration } from 'date-fns';
import { motion } from 'framer-motion';
import { memo, useMemo } from 'react';

import { PublicTable, TableConfig } from '@declarations/table_index/table_index.did';
import {
  EnvironmentBackgroundComponent
} from '@lib/table/components/environment-background/environment-background.component';
import {
  TableBackgroundComponent
} from '@lib/table/components/table/table-background/table-background.component';
import { useTableUrl } from '@lib/table/context/table.context';
import { TokenAmountToString } from '@lib/utils/token-amount-conversion';
import {
  CurrencyComponent, CurrencyTypeComponent, useCurrencyManagerMeta, useIsBTC
} from '@zk-game-dao/currency';
import { Interactable, List, ListItem, PillComponent, useConfig } from '@zk-game-dao/ui';

export type LobbyTableData = Pick<PublicTable, "big_blind" | "small_blind" | "seats">;

export const LobbyTableCardComponent = memo<
  LobbyTableData &
  Partial<Pick<PublicTable, "id" | "round_ticker">> & {
    index: number;
    variant?: "large" | "small";
    config: Pick<TableConfig,
      'name' | 'color' | 'card_color' | 'environment_color' | 'game_type' | 'timer_duration' | 'currency_type' | 'seats' |
      'table_type' | 'table_type'
    >;
    className?: string;
  }
>(
  ({
    id,
    index,
    big_blind,
    small_blind,
    seats,
    variant = "large",
    round_ticker,
    config,
    className
  }) => {
    const { theme } = useConfig();
    const url = useTableUrl({ id, config });
    const relativeUrl = useTableUrl({ id, config }, false);
    const copyUrlToClipboard = useCopyToClipboard(url);
    const {
      name,
      color,
      card_color,
      environment_color,
      game_type,
      timer_duration,
    } = config;

    const meta = useCurrencyManagerMeta(config.currency_type);
    const isBTC = useIsBTC();

    const userAmount = useMemo(() => seats.filter(v => !("Empty" in v)).length, [seats]);

    const subtitle = `${userAmount} / ${config.seats} seats - Round ${Number(round_ticker ?? 0n)}`;

    if (variant === "small")
      return (
        <motion.div
          variants={{
            hidden: { y: -4, opacity: 0, transition: { delay: 0 } },
            visible: { y: 0, opacity: 1, transition: { delay: index * 0.02 } },
          }}
          initial="hidden"
          animate="visible"
          className={classNames(className, "flex rounded-[16px] material relative")}
        >
          <EnvironmentBackgroundComponent
            color={Number(environment_color)}
            className="absolute inset-0 rounded-[16px] z-0"
          />
          <Interactable
            href={variant === "small" ? relativeUrl : undefined}
            className={classNames(
              "w-full h-full flex relative z-1",
              variant === "small"
                ? "cursor-pointer flex-row gap-4 p-6"
                : "flex-col gap-4 lg:gap-8 p-4 lg:p-8",
            )}
          >
            <TableBackgroundComponent
              className={classNames(
                variant === "small" ? " h-[69px] w-[120px] " : "h-48 lg:-mx-4",
              )}
              community_cards={[]}
              visuals={{ color: Number(color), cardColor: Number(card_color) }}
            />

            <div
              className={classNames(
                "flex flex-col gap-[10px] type-body text-neutral-200/70",
                { "justify-center": variant === "small" },
              )}
            >
              <p className="type-button-1 text-white">{name}</p>
              <p className="type-button-3 text-white">
                {userAmount} / {config.seats} seats
              </p>
            </div>
          </Interactable>
        </motion.div>
      );


    return (
      <Interactable
        href={isBTC ? relativeUrl : undefined}
        className={classNames(className, 'relative rounded-[16px] flex flex-col p-6 gap-4', {
          'border-material-main-3 bg-[#3d3c3d] shadow-inner overflow-hidden group': theme === 'purepoker',
          'border-material-medium-1 border-t': theme === 'zkpoker'
        })}
      >
        {isBTC && (
          <>
            <div className={classNames(
              'absolute z-[0] bg-white/60 blur-[50px] w-full aspect-square',
              'transition-all duration-300',
              '-left-1/2 -top-1/2 group-hover:w-[200%] group-hover:-top-full group-hover:left-0'
            )} />
            <div className={classNames(
              'absolute bg-white blur-[50px] z-[1] aspect-square',
              'transition-all duration-500 w-4/5',
              '-left-1/2 -top-1/2 group-hover:w-[120%] group-hover:-top-full group-hover:left-0',
              // '-left-1/2 -top-1/2',
              'opacity-30',
              // 'w-4/5 group-hover:w-[200%]'
            )} />
          </>
        )}
        <EnvironmentBackgroundComponent
          color={Number(environment_color)}
          className={classNames(
            'absolute overflow-hidden',
            {
              "rounded-[16px] -top-[1px] bottom-0 left-0 right-0 -z-1": !isBTC,
              "rounded-[15px] absolute z-[0] inset-px ": isBTC,
            }
          )}
        />

        <TableBackgroundComponent
          className={classNames("h-[170px]", { 'z-[1]': isBTC })}
          community_cards={[]}
          visuals={{ color: Number(color), cardColor: Number(card_color) }}
        />

        <div className="flex flex-col py-3 gap-1 z-1">
          <p className="type-header">{name}</p>
          <div className="flex flex-row type-button-3 text-material-medium-3">{subtitle}</div>
        </div>

        <List
          variant={{
            type: 'default',
            readonly: true
          }}
        >

          <ListItem
            rightLabel={<CurrencyComponent size="small" forceFlex className='self-center' currencyValue={small_blind} currencyType={config.currency_type} />}
          >
            Small blind
          </ListItem>
          <ListItem
            rightLabel={<CurrencyComponent size="small" forceFlex className='self-center' currencyValue={big_blind} currencyType={config.currency_type} />}
          >
            Big blind
          </ListItem>

          <ListItem rightLabel={formatDuration({ seconds: timer_duration })}>
            Timer
          </ListItem>

          {!isBTC && (
            <ListItem rightLabel={<CurrencyTypeComponent currencyType={config.currency_type} />}>
              Currency
            </ListItem>
          )}

          {(() => {
            if ("NoLimit" in game_type)
              return <ListItem rightLabel="No limit">Game type</ListItem>;
            if ("SpreadLimit" in game_type)
              return (
                <>
                  <ListItem rightLabel="Spread Limit">Game type</ListItem>
                  <ListItem
                    rightLabel={
                      <CurrencyComponent
                        currencyType={config.currency_type}
                        variant="inline"
                        currencyValue={game_type.SpreadLimit[0]}
                      />
                    }
                  >
                    Min bet
                  </ListItem>
                  <ListItem
                    rightLabel={
                      <CurrencyComponent
                        currencyType={config.currency_type}
                        variant="inline"
                        currencyValue={game_type.SpreadLimit[1]}
                      />
                    }
                  >
                    Max bet
                  </ListItem>
                </>
              );
            if ("FixedLimit" in game_type)
              return (
                <>
                  <ListItem rightLabel="Fixed Limit">Game type</ListItem>
                  <ListItem
                    rightLabel={
                      <CurrencyComponent
                        currencyType={config.currency_type}
                        variant="inline"
                        currencyValue={game_type.FixedLimit[0]}
                      />
                    }
                  >
                    Small bet
                  </ListItem>
                  <ListItem
                    rightLabel={
                      <CurrencyComponent
                        currencyType={config.currency_type}
                        variant="inline"
                        currencyValue={game_type.FixedLimit[1]}
                      />
                    }
                  >
                    Big bet
                  </ListItem>
                </>
              );
            return <ListItem rightLabel="Unknown">Game type</ListItem>;
          })()}


        </List>

        {
          id && !isBTC && (
            <div className='flex flex-row gap-4 justify-center'>
              <PillComponent href={relativeUrl}>
                Join
              </PillComponent>
              <PillComponent onClick={copyUrlToClipboard}>
                Copy link
              </PillComponent>
            </div>
          )
        }
      </Interactable >
    )
  },
);
LobbyTableCardComponent.displayName = "LobbyTableCardComponent";
