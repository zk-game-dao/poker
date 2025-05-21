import { memo, ReactNode, useEffect, useMemo, useState } from 'react';

import { createActor } from '@declarations/users_canister';
import { Queries } from '@lib/data';
import { useQuery } from '@tanstack/react-query';
import { SignupModalContentComponent, useAuth } from '@zk-game-dao/currency';
import { LoadingAnimationComponent, Modal } from '@zk-game-dao/ui';

import { useIsRegionLocked } from '../../region-locking';
import { callActorMutation } from '../../utils/call-actor-mutation';
import { CreateUserModalComponent } from '../components/create-user-modal.component';
import { ProfileModalContent } from '../components/profile-modal-content.component';
import { RawUserContextProvider } from '../types/user.context';
import { ProvideNotifications } from './notifications.context';
import { useGetUsersCanisterFromUserId } from './users-canisters.context';

declare global {
  interface Window {
    getUserPrincipal?: () => string | undefined;
  }
}

const Content = memo<{
  isLoadingUser: boolean;
  hasUser: boolean;
  hasAuth: boolean;
  onClose: () => void;
}>(({ isLoadingUser, onClose, hasUser, hasAuth }) => {

  if (isLoadingUser)
    return (
      <Modal
        open
        title="Profile"
        onClose={onClose}
      >
        <LoadingAnimationComponent>Loading</LoadingAnimationComponent>
      </Modal>
    )

  if (!hasAuth)
    return <SignupModalContentComponent onClose={onClose} />

  if (!hasUser)
    return <CreateUserModalComponent onClose={onClose} />;

  return (
    <Modal
      open
      title="Profile"
      onClose={onClose}
    >
      <ProfileModalContent onBack={onClose} />
    </Modal>
  )
});
Content.displayName = "UserModalContent";

const _useUser = () => {
  const { authData } = useAuth();
  const isRegionLocked = useIsRegionLocked();
  const getUserCanisterFromUserId = useGetUsersCanisterFromUserId()

  const { data, isPending } = useQuery({
    queryKey: Queries.userSelf.key(authData?.principal),
    queryFn: async () => {
      if (!authData?.principal || isRegionLocked) return null;
      // Find the user by their identities principal
      const id = await getUserCanisterFromUserId(authData.principal);

      // Use the user object to get the users_canister id
      const actor = createActor(id, authData);

      return {
        id,
        actor,
        user: await callActorMutation(actor, 'get_user', authData.principal),
      };
    },
    retry: false,
    refetchOnWindowFocus: true,
    refetchInterval: false,
  });

  return useMemo(
    () => [data?.user, data?.actor, isPending] as const,
    [data, isPending],
  );
};

export const ProvideUser = memo<{ children: ReactNode }>(({ children }) => {
  const [isOpen, setIsOpen] = useState(false);
  const { authData } = useAuth();
  const [user, actor, isLoadingUser] = _useUser();

  useEffect(() => {
    const principal = authData?.principal.toText();
    window.getUserPrincipal = () => principal;
  }, [authData?.principal.toText()]);

  return (
    <RawUserContextProvider
      value={{
        actor,
        user,
        isLoading: isLoadingUser,
        show: () => setIsOpen(true),
        showProfile: () => setIsOpen(true),
        showSignup: () => setIsOpen(true),
      }}
    >
      <ProvideNotifications>
        {isOpen && (
          <Content
            isLoadingUser={isLoadingUser}
            hasAuth={!!authData}
            hasUser={!!user}
            onClose={() => setIsOpen(false)}
          />
        )}
        {children}
      </ProvideNotifications>
    </RawUserContextProvider>
  );
});
ProvideUser.displayName = "ProvideUser";
