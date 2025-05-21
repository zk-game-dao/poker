import { addMinutes } from 'date-fns';
import { memo } from 'react';

import { useRouting } from '@/src/hooks/routing';
import { canisterId, table_index } from '@declarations/table_index';
import { TableConfig } from '@declarations/table_index/table_index.did';
import { Principal } from '@dfinity/principal';
import { useUser } from '@lib/user';
import { WrapOptional } from '@lib/utils/optional';
import { useMutation } from '@tanstack/react-query';
import { useAllowance, useAuth, useIsBTC } from '@zk-game-dao/currency';
import {
  ButtonComponent, Modal, SteppedModalComponent, SteppedModalStep, useToast
} from '@zk-game-dao/ui';

import { callActorMutation } from '../../../utils/call-actor-mutation';
import { RevenueShareCostPP, RevenueShareCostZKP } from './constants';
import { Config as AppeareanceConfig } from './steps/appeareance-step.config';
import { Config as GameTypeConfig } from './steps/game-type-step.config';
import { Config as NameConfig } from './steps/name-step.config';
import { Config as PlayerCountConfig } from './steps/player-count-step.config';
import { Config as PreviewConfig } from './steps/preview-step.config';
import { Config as TimeLimitConfig } from './steps/time-limit-step.config';

const STEPS: SteppedModalStep<TableConfig, any>[] = [
  GameTypeConfig,
  TimeLimitConfig,
  AppeareanceConfig,
  PlayerCountConfig,
  NameConfig,
  PreviewConfig,
];

type CreateTableModalProps = {
  open?: boolean;
  onCancel(): void;
  /** This value is only for debug purposes */
  initialStep?: number;
};

export const CreateTableModalComponent = memo<CreateTableModalProps>(
  ({ onCancel, open, initialStep = 0 }) => {
    const { user, showSignup } = useUser();
    const { authData } = useAuth();
    const { push, getHref } = useRouting();
    const { addToast } = useToast();
    const isBTC = useIsBTC();
    const allowance = useAllowance({
      currencyType: { Real: isBTC ? { BTC: null } : { ICP: null } },
      receiver: { principal: Principal.fromText(canisterId) },
      name: 'Table'
    });

    const mutation = useMutation({
      mutationFn: async (table: TableConfig) => {
        if (!authData) throw new Error('You are not logged in');
        if (table.is_shared_rake?.[0]?.length === 2)
          await allowance.require({
            amount: isBTC ? RevenueShareCostPP : RevenueShareCostZKP,
            reason: 'Create table'
          }, addMinutes(new Date(), 2));

        return await callActorMutation(
          table_index,
          "create_table",
          {
            ...table,
            enable_rake: [],
            ante_type: [],
            is_paused: [],
            table_type: []
          },
          WrapOptional(authData.principal),
        );
      },
      onSuccess: ({ id }) => {
        push(`/tables/${id}`);
        addToast({
          children: "Table created",
          ctas: [
            {
              children: "Copy link",
              onClick: () => {
                navigator.clipboard.writeText(getHref(`/tables/${id}`, true));
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
          <ButtonComponent className="mx-auto" onClick={showSignup}>
            Login to create a table
          </ButtonComponent>
        </Modal>
      );
    }

    return (
      <SteppedModalComponent
        steps={STEPS}
        onClose={onCancel}
        initialStep={initialStep}
        open={open}
        {...mutation}
      />
    );
  },
);
CreateTableModalComponent.displayName = "CreateTableModalComponent";