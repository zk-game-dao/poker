import { memo, useMemo } from 'react';

import { useRouting } from '@/src/hooks/routing';
import { TableConfig } from '@declarations/table_index/table_index.did';
import { tournament_index } from '@declarations/tournament_index';
import { NewTournament, TournamentType } from '@declarations/tournament_index/tournament_index.did';
import {
  Config as AppeareanceConfig
} from '@lib/table/components/create-table-modal/steps/appeareance-step.config';
import {
  ConfigWithoutAutoKick as TimeLimitConfig
} from '@lib/table/components/create-table-modal/steps/time-limit-step.config';
import { useUser } from '@lib/user';
import { callActorMutation } from '@lib/utils/call-actor-mutation';
import { useMutation } from '@tanstack/react-query';
import {
  ButtonComponent, Modal, ModalFooterPortal, SteppedModalComponent, SteppedModalStep,
  TitleTextComponent, useToast
} from '@zk-game-dao/ui';

import { Config as BasicsConfig } from './steps/basics-step.config';
import { Config as BuyInConfig } from './steps/buy-in.config';
import { Config as PayoutsConfig } from './steps/payouts.config';
import { Config as PlayerSettingsStepConfig } from './steps/players-step.config';
import { Config as PreviewConfig } from './steps/preview-step.config';
import { Config as TypeConfig, defaultBuyInOptions } from './steps/type-step.config';

type StepsProps = NewTournament & Omit<TableConfig, 'require_proof_of_humanity' | 'name' | 'game_type' | 'currency_type' | 'table_type' | 'enable_rake'>;

const STEPS: SteppedModalStep<StepsProps>[] = [
  BasicsConfig,
  PlayerSettingsStepConfig,
  BuyInConfig,
  TypeConfig,
  PayoutsConfig,
  AppeareanceConfig,
  TimeLimitConfig,
  PreviewConfig,
];

type CreateTournamentModalProps = {
  open?: boolean;
  onCancel(): void;
  /** This value is only for debug purposes */
  initialStep?: number;
  initialType: 'BuyIn' | 'SitAndGo' | 'Freeroll' | 'SpinAndGo';
};

export const CreateTournamentModalComponent = memo<CreateTournamentModalProps>(
  ({ onCancel, open, initialStep = 0, initialType }) => {
    const { user, showSignup } = useUser();
    const { push, getHref } = useRouting();
    const { addToast } = useToast();

    const initialTournamentType = useMemo((): TournamentType => {
      switch (initialType) {
        case 'BuyIn':
          return { BuyIn: { SingleTable: defaultBuyInOptions } };
        case 'Freeroll':
          return { Freeroll: { SingleTable: defaultBuyInOptions } };
        case 'SitAndGo':
          return { SitAndGo: { SingleTable: defaultBuyInOptions } };
        case 'SpinAndGo':
          return {
            SpinAndGo: [{ SingleTable: defaultBuyInOptions }, { multiplier: 0n, payout_structure: [] }]
          };
      }
    }, [initialType]);

    const mutation = useMutation({
      mutationFn: async ({
        name,
        color,
        card_color,
        environment_color,
        description,
        tournament_type,
        start_time,
        currency,
        late_registration_duration_ns,
        starting_chips,
        buy_in,
        min_players,
        max_players,
        speed_type,
        payout_structure,
        timer_duration,
        auto_start_timer,
        max_inactive_turns,
        max_seated_out_turns,
        hero_picture,
        seats,
        require_proof_of_humanity,
      }: StepsProps) => {
        const newTournament: NewTournament = {
          name,
          start_time,
          description,
          currency,

          tournament_type,
          hero_picture,

          late_registration_duration_ns,
          starting_chips,
          buy_in,

          min_players,
          max_players,

          speed_type,

          payout_structure,
          require_proof_of_humanity,
        };

        const newTable: TableConfig = {
          name, // Will be overriden by BE
          game_type: { NoLimit: 0n }, // Will be overriden by BE
          seats: seats ?? 8, // Will be overriden by BE
          currency_type: { Fake: null }, // Will be overriden by BE
          enable_rake: [false], // Will be overriden by BE
          table_type: [], // Will be overriden by BE
          is_private: [false], // General defaults

          ante_type: [], // Not implemented yet

          color,
          card_color,
          environment_color,

          timer_duration,
          auto_start_timer,
          max_inactive_turns,
          max_seated_out_turns,
          is_shared_rake: [],

          is_paused: [],
          require_proof_of_humanity: [],
        };

        console.log({ newTournament, newTable });

        return await callActorMutation(
          tournament_index,
          "create_tournament",
          newTournament,
          newTable
        )
      },
      onSuccess: (principal) => {
        push(`/tournaments/${principal.toText()}`);
        addToast({
          children: "Tournament created",
          ctas: [
            {
              children: "Copy link",
              onClick: () => {
                navigator.clipboard.writeText(getHref(`${window.location.origin}/tournaments/${principal.toText()}`));
                addToast({ children: "Link copied" });
              },
            },
          ],
        });
      },
    });

    if (!user && open) {
      return (
        <Modal title="Login to create a table" onClose={onCancel} open={open}>
          <TitleTextComponent
            text="Login to your account to create a tournament."
          />
          <ModalFooterPortal>
            <ButtonComponent onClick={onCancel} variant="naked">
              Cancel
            </ButtonComponent>
            <ButtonComponent onClick={showSignup}>
              Login
            </ButtonComponent>
          </ModalFooterPortal>
        </Modal>
      );
    }

    return (
      <SteppedModalComponent
        steps={STEPS}
        onClose={onCancel}
        initialStep={initialStep}
        open={open}
        initialData={{ tournament_type: initialTournamentType }}
        {...mutation}
      />
    );
  },
);
CreateTournamentModalComponent.displayName = 'CreateTournamentModalComponent';
