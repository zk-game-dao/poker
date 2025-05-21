import { memo, useState } from 'react';

import { users_index } from '@declarations/users_index';
import { Queries } from '@lib/data';
import { callActorMutation } from '@lib/utils/call-actor-mutation';
import { useAuth } from '@zk-game-dao/currency';
import {
  ButtonComponent, ErrorComponent, ErrorModalComponent, Interactable, List, ListItem, Modal,
  ModalFooterPortal, SwitchInputComponent, TextInputComponent, useMutation, UserError,
  WrapOptional
} from '@zk-game-dao/ui';

import { useReferrer } from '../../../hooks/referrer';
import { useUsersCanisters } from '../context/users-canisters.context';

export const CreateUserModalComponent = memo<{ onClose(): void; }>(({ onClose }) => {
  const { authData, logout, error } = useAuth();
  const { storeUser } = useUsersCanisters();

  const [userName, setUserName] = useState<string | undefined>();
  const [agreesToRules, setAgreesToRules] = useState(false);
  const referrer = useReferrer();

  const {
    mutate: createUser,
    error: createUserError,
    isPending: isCreatingUser,
  } = useMutation({
    mutationFn: async () => {
      if (!authData?.principal) throw new Error("No principal");
      if (!userName) throw new UserError("Username is required");
      if (userName.length < 3)
        throw new UserError("Username must be at least 3 characters");
      if (userName.length > 42)
        throw new UserError("Username must be at most 42 characters");
      if (!agreesToRules)
        throw new UserError("You must agree to the terms and conditions");
      return callActorMutation(
        users_index,
        "create_user",
        userName,
        [],
        authData?.principal,
        [],
        WrapOptional(referrer)
      );
    },
    onSuccess: (user) => {
      storeUser(user);
      Queries.auth.invalidate();
      Queries.userFromUserId.invalidate(authData?.principal);
      Queries.userSelf.invalidate(authData?.principal);
    },
  });

  if (!authData)
    return <ErrorModalComponent onClose={onClose}>{new UserError("No auth data")}</ErrorModalComponent>;

  return (
    <Modal onClose={onClose} title="Create account">
      <List>
        <ListItem
          rightLabel={
            <Interactable
              className="text-red-500 whitespace-nowrap"
              onClick={logout}
            >
              Log out
            </Interactable>
          }
        >
          Logged in
          <div className="text-material-heavy-1 ml-1">
            {authData.provider.type}
            {/* <LoginProviderLabel provider={authData.provider} /> */}
          </div>
        </ListItem>
        <TextInputComponent
          label="Username"
          value={userName}
          onChange={setUserName}
        />
        <SwitchInputComponent
          label={
            <div className="inline whitespace-pre">
              I agree to the{" "}
              <Interactable
                href="/rules"
                className="underline hover:no-underline inline-flex"
              >
                House rules
              </Interactable>
            </div>
          }
          onChange={setAgreesToRules}
          checked={agreesToRules}
        />
      </List>

      <ErrorComponent error={error || createUserError} className="mb-4" />

      {!!authData?.principal && (
        <ModalFooterPortal>
          <ButtonComponent
            isLoading={isCreatingUser}
            className="ml-auto"
            onClick={createUser}
          >
            Create account
          </ButtonComponent>
        </ModalFooterPortal>
      )}
    </Modal>
  );
},
);
CreateUserModalComponent.displayName = "CreateUserModalComponent";
