import { AVATAR_EMOJI_STYLES, AvatarComponent } from '@/src/components/common/avatar/avatar.component';
import { User, UserAvatar } from '@declarations/users_canister/users_canister.did';
import { Queries } from '@lib/data';
import { useAuth, useIsBTC, WalletModalContent } from '@zk-game-dao/currency';
import {
  ButtonComponent,
  ErrorComponent,
  ExperiencePointsComponent,
  FormComponent,
  Image,
  Interactable,
  List,
  ListItem,
  LoadingAnimationComponent,
  Modal,
  ModalFooterPortal,
  useMutation,
  useToast,
} from '@zk-game-dao/ui';
import classNames from 'classnames';
import { memo, useEffect, useMemo, useState } from 'react';

import { callActorMutation } from '../../utils/call-actor-mutation';
import { ConvertNat16ToPerc, ConvertPercToNat16 } from '../../utils/nat16';
import { NotificationsModalComponent } from '../notifications-modal/notifications-modal.component';
import { useUser } from '../types/user.context';
import { VerifyModal } from './verify-modal.component';
import { DefaultSoundVolume } from '../../utils/sound';
import { InviteAFriendModalComponent } from './invinte-a-friend-modal.component';

const AvatarSelector = memo<{
  onChange(avatar: [UserAvatar]): void;
  value: [] | [UserAvatar];
}>(({ onChange, value }) => {
  const emojis = useMemo(
    () => Array.from({ length: 24 }, (_, i) => BigInt(i)),
    [],
  );

  return (
    <>
      {/* <TabsComponent
        tabs={[
          { label: 'Emojis', value: 'emojis' },
          { label: 'Images', value: 'images', disabled: true },
          { label: 'Avatar', value: 'custom', disabled: true }
        ]}
        value='emojis'
        onChange={() => { }}
      /> */}

      <div className="flex flex-row flex-wrap gap-2">
        <p className="type-callout text-material-medium-2 w-full">Emoji</p>
        {emojis.map((emoji) => (
          <Interactable
            key={emoji}
            className={classNames(
              "bg-material-main-1 w-16 h-16 flex items-center justify-center rounded-[12px] border-[3px]",
              "transition-[transform,border] duration-75 active:scale-95",
              value[0]?.Emoji.emoji == emoji
                ? "ring-material-diabolical shadow-[0px,3px,8px,0px,#0000001F]"
                : "border-transparent",
            )}
            onClick={() =>
              onChange([
                { Emoji: { emoji, style: value[0]?.Emoji.style || 0n } },
              ])
            }
          >
            <Image
              width={32}
              height={32}
              type="png"
              src={`/nfts/avatars/emojis/${emoji}.png`}
              alt={`emoji-${emoji}`}
            />
          </Interactable>
        ))}
      </div>

      <div className="flex flex-row flex-wrap gap-2">
        <p className="type-callout text-material-medium-2 w-full">Style</p>
        {AVATAR_EMOJI_STYLES.map((_, i) => BigInt(i)).map((style) => {
          const emoji = value[0]?.Emoji.emoji || 0n;
          return (
            <Interactable
              key={style}
              onClick={() => onChange([{ Emoji: { emoji, style } }])}
              className={classNames(
                "border-2 p-0.5 rounded-full",
                value[0]?.Emoji.style === style
                  ? "border-material-diabolical"
                  : "border-transparent",
              )}
            >
              <AvatarComponent
                size="medium"
                avatar={[{ Emoji: { emoji, style } }]}
              />
            </Interactable>
          );
        })}
      </div>
    </>
  );
});
AvatarSelector.displayName = "AvatarSelector";

export const ProfileModalContent = memo<{
  onBack(): void;
}>(({ onBack }) => {
  const { authData, logout } = useAuth();
  const { user, actor: service } = useUser();
  const { addToast } = useToast();

  const [isEditingAvatar, setIsEditingAvatar] = useState(false);

  const [userForm, setUserForm] = useState<
    Pick<User, "user_name" | "avatar" | "volume_level">
  >({ user_name: "", avatar: [], volume_level: [] });
  useEffect(
    () =>
      setUserForm({
        volume_level: user?.volume_level || [DefaultSoundVolume],
        user_name: user?.user_name || "",
        avatar: user?.avatar || [],
      }),
    [user],
  );

  const [showNotificationsModal, setShowNotificationsModal] = useState(false);

  const {
    mutateAsync: mutate,
    error,
    isPending: isUpdatingUser,
  } = useMutation({
    mutationFn: async ([user_name]: [string]) => {
      if (!authData?.principal) throw "No principal";
      if (!user) throw "No user";
      if (!service) throw "No service";
      if (user_name.length < 3) throw "Username must be at least 3 characters";
      if (user_name.length > 42) throw "Username must be at most 42 characters";

      return callActorMutation(
        service,
        "update_user",
        authData.principal,
        [user_name],
        [],
        [authData.principal.toText()],
        userForm.avatar,
        [],
        userForm.volume_level,
        // eth_wallet_address
        []
      );
    },
    onSuccess: () => {
      Queries.userSelf.invalidate();
      Queries.userFromUserId.invalidate(authData?.principal);

      addToast({ children: "User updated" });
      setIsEditingAvatar(false);
    },
  });

  useEffect(() => {
    if (!authData?.principal) return onBack();
  }, [JSON.stringify(authData?.principal), onBack]);

  const [isVerifying, setIsVerifying] = useState(false);
  const [isShowingWallet, setIsShowingWallet] = useState(false);
  const isBtc = useIsBTC();
  const [isShowingInviteAFriend, setIsShowingInviteAFriend] =
    useState(false);

  if (!user)
    return <LoadingAnimationComponent>Loading user</LoadingAnimationComponent>;

  return (
    <>
      <VerifyModal open={isVerifying} onClose={() => setIsVerifying(false)} />
      <Modal open={isShowingWallet} onClose={() => setIsShowingWallet(false)}>
        <WalletModalContent onBack={() => setIsShowingWallet(false)} />
      </Modal>
      <ErrorComponent className="mb-4" error={error} />
      <AvatarComponent
        size="big"
        onEdit={!isEditingAvatar ? () => setIsEditingAvatar(true) : undefined}
        className="mx-auto"
        {...user}
        {...userForm}
      />

      <div className="text-center flex flex-col justify-center">

        <p className="type-header mx-auto">{user.user_name}</p>

        <ExperiencePointsComponent
          {...user}
          experience_points={isBtc ? user.experience_points_pure_poker : user.experience_points}
          className="px-2 py-1 text-material-heavy-1 mx-auto mb-2"
        />

        {!isEditingAvatar && (
          <>
            <List className="my-4 lg:mb-0">
              <ListItem onClick={() => setIsShowingWallet(true)}>Wallet</ListItem>
              <ListItem onClick={() => setShowNotificationsModal(true)}>
                Notifications
              </ListItem>
              <ListItem onClick={user.is_verified[0] ? undefined : () => setIsVerifying(true)}>
                {user.is_verified[0] ? 'Verified' : 'Verify Identity'}
              </ListItem>

              <ListItem onClick={() => setIsShowingInviteAFriend(true)}>
                Invite A Friend
              </ListItem>

            </List>

            <InviteAFriendModalComponent isOpen={isShowingInviteAFriend} onClose={() => setIsShowingInviteAFriend(false)} principal={user.principal_id} />

            <NotificationsModalComponent
              isOpen={showNotificationsModal}
              onClose={() => setShowNotificationsModal(false)}
            />
          </>
        )}
      </div >

      {isEditingAvatar ? (
        <>
          <AvatarSelector
            onChange={(avatar) => setUserForm((v) => ({ ...v, avatar }))}
            value={userForm.avatar}
          />
          <ModalFooterPortal>
            <ButtonComponent variant="naked">Cancel</ButtonComponent>
            <ButtonComponent
              onClick={() => mutate([userForm.user_name])}
              isLoading={isUpdatingUser}
            >
              Save
            </ButtonComponent>
          </ModalFooterPortal>
        </>
      ) : (
        <>
          <FormComponent
            onConfirm={([user_name]) => mutate([user_name as string])}
            onChange={([user_name, volume_level]) =>
              setUserForm((v) => ({
                ...v,
                user_name: user_name as string,
                volume_level: [
                  ConvertPercToNat16((volume_level as number) / 100),
                ],
              }))
            }
            onCancel={async () => {
              await logout();
              onBack();
            }}
            confirmLabel="Save"
            cancelLabel="Logout"
            cancelColor="red"
            fields={[
              { label: "Username", type: "text" },
              {
                label: "Volume level",
                type: "slider",
                min: 0,
                max: 100,
                step: 1,
              },
            ]}
            values={[
              userForm.user_name,
              ConvertNat16ToPerc(userForm.volume_level[0]) * 100,
            ]}
            isLoading={isUpdatingUser}
          />
        </>
      )}
    </>
  );
});
ProfileModalContent.displayName = "ProfileModalContent";
