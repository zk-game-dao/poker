import classNames from 'classnames';
import { AnimatePresence, motion } from 'framer-motion';
import { memo, useMemo, useState } from 'react';

import { useUser } from '@lib/user';
import {
  ButtonComponent, DynamicSizeComponent, Modal, ModalFooterPortal, TitleTextComponent,
  UnwrapOptional,
  WeirdKnobComponent
} from '@zk-game-dao/ui';

import {
  useCurrentTableTurnProgressRemainder, useNewRoundProgress, useTable
} from '../../context/table.context';
import { CardComponent } from '../card/card.component';
import { HudBalanceComponent } from './hud-balance.component';
import { HUDBettingConsumer, ProvideHUDBettingContext, useSitOut } from './hud-betting.context';
import { HudPlayButtonsComponent } from './hud-play-buttons.component';
import { HUDQuickActionsComponent } from './hud-quick-actions.component';
import { HudSeperator } from './hud-seperator.component';
import { useTournament } from '../../../tournament/context/tournament.context';
import { useEnterTexts } from '../../../tournament/components/enter-modal.component';

export const HUDComponent = memo(() => {
  const { isOngoing, table, isJoined, userIndex, user } = useTable();
  const { user: zkpUser } = useUser();

  const turnProgress = useCurrentTableTurnProgressRemainder(
    isJoined && table.current_player_index === userIndex,
  );
  const newRoundProgress = useNewRoundProgress(isJoined);
  const progress = useMemo(
    () => newRoundProgress ?? turnProgress,
    [newRoundProgress, turnProgress],
  );
  const sitout = useSitOut();

  const [showSitOutModal, setShowSitOutModal] = useState(false);
  const tournament = useTournament();
  const texts = useEnterTexts();

  return (
    <AnimatePresence>
      {!!zkpUser && (
        <motion.div
          initial={{ opacity: 0, y: 32, scale: 1.1 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          exit={{ opacity: 0, y: 16, scale: 0.9 }}
          className="flex flex-col relative z-10 items-center px-4 lg:px-0"
        >
          <ProvideHUDBettingContext>
            <HUDBettingConsumer>
              {({ raise }) => (
                <AnimatePresence>
                  {(
                    isJoined &&
                    table.current_player_index === userIndex &&
                    isOngoing &&
                    raise &&
                    raise.quickActions?.length > 0
                  ) && (
                      <HUDQuickActionsComponent
                        quickActions={raise.quickActions}
                        mutate={raise.cta.mutateExplicit}
                      />
                    )}
                </AnimatePresence>
              )}
            </HUDBettingConsumer>

            <div className='relative flex items-center flex-col'>

              <div className='flex flex-row absolute bottom-full z-0 translate-y-2 group'>
                <AnimatePresence>
                  {user?.data?.cards.map((card, i) => (
                    <motion.div
                      variants={{
                        visible: {
                          y: 16,
                          rotate: 0,
                          scale: 1,
                          opacity: 1,
                        },
                        hidden: {
                          y: 0,
                          rotate: i === 0 ? -2 : 2,
                          scale: 0.9,
                          opacity: 0,
                        },
                        turn: {
                          y: -32,
                          rotate: i === 0 ? -2 : 2,
                          scale: 1,
                          opacity: 1,
                        },
                      }}
                      initial="hidden"
                      animate={isJoined &&
                        table.current_player_index === userIndex &&
                        isOngoing ? "turn" : "visible"}
                      exit="hidden"
                      key={i}
                      className={classNames({
                        [isJoined &&
                          table.current_player_index === userIndex &&
                          isOngoing ? '-ml-6' : '-ml-4']: i > 0
                      })}
                    >
                      <CardComponent
                        card={card}
                        size={isJoined && table.current_player_index === userIndex && isOngoing ? "medium" : 'small'}
                      />
                    </motion.div>
                  ))}
                </AnimatePresence>
              </div>

              <div className="material rounded-[12px] lg:rounded-[24px] z-10 relative gap-2 flex flex-col items-center justify-center p-2 lg:p-3 lg:whitespace-nowrap w-full lg:w-auto ">
                <div className="absolute inset-0 rounded-[12px] lg:rounded-[24px] overflow-hidden">
                  {progress !== undefined && (
                    <motion.div
                      variants={{
                        visible: (v) => ({
                          right: !v ? "100%" : `${Math.floor((1 - v) * 100)}%`,
                        }),
                      }}
                      initial={false}
                      className={classNames(
                        "absolute -left-4 -inset-y-4 blur-[8px] transition-colors",
                        progress < 0.2
                          ? "animate-pulse bg-material-medium-1"
                          : "bg-material-main-3",
                      )}
                      animate="visible"
                      custom={progress}
                    />
                  )}
                  <div
                    style={{
                      backgroundImage: "url(/images/grain.png)",
                      backgroundSize: "1194px 834px",
                    }}
                    className="absolute inset-0 mix-blend-screen opacity-[0.08] z-0 pointer-events-none"
                  />
                </div>

                <HUDBettingConsumer>
                  {({ tableUser, raise, autoCheckFold, currencyType }) => (
                    <AnimatePresence>
                      {isJoined && tableUser && (
                        <HudBalanceComponent
                          balance={tableUser.balance}
                          currencyType={currencyType}
                        />
                      )}

                      <div className="gap-2 flex flex-row items-center justify-center">
                        {!raise?.showInlineInput && isJoined && (
                          <>

                            {!sitout.isSittingOut && autoCheckFold && isOngoing && (
                              <div className={classNames('transition-transform', { 'scale-90': autoCheckFold.data })}>
                                <WeirdKnobComponent
                                  mutate={() => autoCheckFold.mutate(!autoCheckFold.data)}
                                  isPending={autoCheckFold.isPending}
                                  variant={autoCheckFold.data ? 'gray' : "transparent"}
                                >
                                  Check/Fold
                                </WeirdKnobComponent>
                              </div>
                            )}

                            {isOngoing && !sitout.isSittingOut && (
                              <>
                                <WeirdKnobComponent
                                  mutate={() => setShowSitOutModal(true)}
                                  isPending={sitout.isPending || showSitOutModal}
                                  variant="transparent"
                                >
                                  Sit out
                                </WeirdKnobComponent>

                                <Modal
                                  open={showSitOutModal}
                                  onClose={() => setShowSitOutModal(false)}
                                >
                                  <TitleTextComponent
                                    title="Sit Out"
                                    text="If you choose to sit out while the game is in progress, your hand will automatically fold."
                                  />
                                  <ModalFooterPortal>
                                    <ButtonComponent
                                      variant="naked"
                                      onClick={() => setShowSitOutModal(false)}
                                    >
                                      Cancel
                                    </ButtonComponent>
                                    <ButtonComponent
                                      color="red"
                                      onClick={async () => {
                                        await sitout.sitOut();
                                        setShowSitOutModal(false);
                                      }}
                                      isLoading={sitout.isPending}
                                    >
                                      Fold & Sit out
                                    </ButtonComponent>
                                  </ModalFooterPortal>
                                </Modal>
                              </>
                            )}

                            {(!sitout.isSittingOut || (autoCheckFold && !isOngoing) || (!autoCheckFold && isOngoing)) && <HudSeperator desktopOnly />}
                          </>
                        )}

                        <DynamicSizeComponent
                          animateWidth
                          animateHeight={false}
                          className="whitespace-nowrap justify-center items-center"
                        >
                          <div className="flex flex-row">
                            <HudPlayButtonsComponent
                              tournament_table_id={tournament?.user?.table?.id}
                              tournament_is_running={tournament?.isRunning}
                              tournament_start_time={tournament?.data.start_time}
                              tournament_state={tournament?.data.state}
                              tournament_join_type={tournament?.joinType}
                              tournamentUserTextsTitle={texts?.title}

                              isSittingOut={sitout.isSittingOut}
                              isSittingBackIn={sitout.isSittingBackIn}
                              isSittingOutPending={sitout.isPending}
                              rejoin={sitout.rejoin}
                              sitOut={sitout.sitOut}

                              userIndex={userIndex}
                              userPlayerAction={user?.data?.player_action}
                              userIsQueuedForNextRound={user && "QueuedForNextRound" in user.status}

                              isTableOngoing={isOngoing}
                              current_player_index={table.current_player_index}
                              tableId={table.id}
                              isTablePaused={UnwrapOptional(table.config.is_paused)}
                              tableHasMoreThanOnePlayer={table.seats.filter(v => !("Empty" in v)).length > 1}
                            />
                          </div>
                        </DynamicSizeComponent>
                      </div>
                    </AnimatePresence>
                  )}
                </HUDBettingConsumer>


              </div>
            </div>
          </ProvideHUDBettingContext>
        </motion.div>
      )}
    </AnimatePresence>
  );
});
HUDComponent.displayName = "HUDComponent";
