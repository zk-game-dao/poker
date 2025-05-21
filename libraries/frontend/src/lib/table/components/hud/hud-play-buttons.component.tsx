import { memo, useState } from 'react';

import {
  PlayerAction, TournamentState
} from '@declarations/tournament_canister/tournament_canister.did';
import { Principal } from '@dfinity/principal';
import { IsSamePrincipal } from '@zk-game-dao/currency';
import { WeirdKnobComponent } from '@zk-game-dao/ui';

import { useFormatDateDistance } from '../../../../hooks/countdown';
import {
  EnterTournamentModalComponent
} from '../../../tournament/components/enter-modal.component';
import { JoinType } from '../../../tournament/context/tournament.context';
import {
  IsSamePlayerAction, IsSameTournamentJoinType, IsSameTournamentState
} from '../../../utils/compare';
import { BigIntTimestampToDate } from '../../../utils/time';
import { DynamicLoadingIndicator } from './hud-dynamic-loading-indicator.component';
import { TurnButtonsComponent } from './turn-buttons.component';

export const HudPlayButtonsComponent = memo<{
  tournament_table_id?: Principal;
  tournament_is_running?: boolean;
  tournament_start_time?: bigint;
  tournament_join_type?: JoinType;
  tournament_state?: TournamentState;
  tournamentUserTextsTitle?: string;

  isSittingOut?: boolean;
  isSittingBackIn?: boolean;
  isSittingOutPending: boolean;
  sitOut: () => void;
  rejoin: () => void;

  userIndex?: bigint;
  userPlayerAction?: PlayerAction;
  userIsQueuedForNextRound?: boolean;

  isTableOngoing: boolean;
  current_player_index: bigint;
  tableId: Principal;
  isTablePaused?: boolean;
  tableHasMoreThanOnePlayer: boolean;
}>(({
  tournament_table_id,
  tournament_is_running,
  tournament_start_time,
  tournament_state,
  tournament_join_type,
  tournamentUserTextsTitle,

  sitOut,
  rejoin,
  isSittingOutPending,
  isSittingOut,
  isSittingBackIn,

  userIndex,
  userPlayerAction,
  userIsQueuedForNextRound,

  isTableOngoing,
  current_player_index,
  tableId,
  isTablePaused,
  tableHasMoreThanOnePlayer
}) => {
  const diff = useFormatDateDistance(tournament_start_time ? BigIntTimestampToDate(tournament_start_time) : undefined);

  const [showEnterTournamentModal, setShowEnterTournamentModal] = useState(false);

  if (tournament_state) {
    if (tournament_table_id && tournament_table_id?.compareTo(tableId) !== "eq")
      return (
        <WeirdKnobComponent href={`/tournaments/${tournament_table_id.toText()}/my-table`}>
          Go to your table
        </WeirdKnobComponent>
      );

    if (diff && tournament_is_running !== true)
      return <DynamicLoadingIndicator>Tournament{diff.number > 0 ? ` in ${diff.string}` : ' is starting'}</DynamicLoadingIndicator>;
  }

  if (isTablePaused) {
    if (!tournament_state)
      return <DynamicLoadingIndicator>Paused</DynamicLoadingIndicator>
    return <DynamicLoadingIndicator>Paused for addon period</DynamicLoadingIndicator>;
  }

  if (userIndex === undefined || userIndex < 0) {
    if (tournament_state) {
      if (tournament_join_type) {
        switch (tournament_join_type.type) {
          case "rebuy":
            return (
              <>
                <EnterTournamentModalComponent open />
                <DynamicLoadingIndicator>
                  Rebuy into the tournament
                </DynamicLoadingIndicator>
              </>
            );
          default:
            return (
              <>
                <EnterTournamentModalComponent
                  open={showEnterTournamentModal}
                  onClose={() => setShowEnterTournamentModal(false)}
                />
                <WeirdKnobComponent mutate={() => setShowEnterTournamentModal(true)}>
                  {tournamentUserTextsTitle ?? 'Join the tournament'}
                </WeirdKnobComponent>
              </>
            );

        }
      }

      if ("Registration" in tournament_state)
        return <DynamicLoadingIndicator>The tournament hasn't started yet</DynamicLoadingIndicator>;

      return <DynamicLoadingIndicator>You can't join at the moment</DynamicLoadingIndicator>;
    }

    return <DynamicLoadingIndicator>Take a seat</DynamicLoadingIndicator>;
  }

  if (userIsQueuedForNextRound)
    return (
      <DynamicLoadingIndicator>
        Waiting for the next round
      </DynamicLoadingIndicator>
    );

  if (!tableHasMoreThanOnePlayer)
    return (
      <DynamicLoadingIndicator>
        Waiting for more players to join
      </DynamicLoadingIndicator>
    );

  if (isSittingOut || userPlayerAction && "SittingOut" in userPlayerAction) {
    if (isSittingBackIn)
      return (
        <DynamicLoadingIndicator>Rejoining next round</DynamicLoadingIndicator>
      );
    return (
      <WeirdKnobComponent mutate={rejoin} isPending={isSittingOutPending}>
        Rejoin
      </WeirdKnobComponent>
    );
  }

  if (!isTableOngoing) {
    return (
      <>
        <WeirdKnobComponent mutate={sitOut} isPending={isSittingOutPending}>
          Sit out
        </WeirdKnobComponent>
      </>
    );
  }

  if (userPlayerAction) {
    if ("Folded" in userPlayerAction)
      return <DynamicLoadingIndicator>You folded</DynamicLoadingIndicator>;

    if ("AllIn" in userPlayerAction)
      return <DynamicLoadingIndicator>All in</DynamicLoadingIndicator>;
  }

  if (current_player_index !== userIndex)
    return <DynamicLoadingIndicator>Awaiting turn</DynamicLoadingIndicator>;

  return <TurnButtonsComponent />;
},
  (prevProps, nextProps) =>
    IsSamePrincipal(prevProps.tournament_table_id, nextProps.tournament_table_id) &&
    prevProps.tournament_is_running === nextProps.tournament_is_running &&
    prevProps.tournament_start_time === nextProps.tournament_start_time &&
    IsSameTournamentState(prevProps.tournament_state, nextProps.tournament_state) &&
    IsSameTournamentJoinType(
      prevProps.tournament_join_type,
      nextProps.tournament_join_type
    ) &&
    prevProps.tournamentUserTextsTitle === nextProps.tournamentUserTextsTitle &&

    prevProps.isSittingOut === nextProps.isSittingOut &&
    prevProps.isSittingBackIn === nextProps.isSittingBackIn &&
    prevProps.isSittingOutPending === nextProps.isSittingOutPending &&
    prevProps.sitOut === nextProps.sitOut &&
    prevProps.rejoin === nextProps.rejoin &&

    prevProps.userIndex === nextProps.userIndex &&
    IsSamePlayerAction(prevProps.userPlayerAction, nextProps.userPlayerAction) &&
    prevProps.userIsQueuedForNextRound === nextProps.userIsQueuedForNextRound &&

    prevProps.isTableOngoing === nextProps.isTableOngoing &&
    prevProps.current_player_index === nextProps.current_player_index &&
    IsSamePrincipal(prevProps.tableId, nextProps.tableId) &&
    prevProps.isTablePaused === nextProps.isTablePaused &&
    prevProps.tableHasMoreThanOnePlayer === nextProps.tableHasMoreThanOnePlayer
);
HudPlayButtonsComponent.displayName = "HudButtons";
