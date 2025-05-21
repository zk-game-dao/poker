import { memo } from 'react';

import { Principal } from '@dfinity/principal';
import {
  requestVerifiablePresentation, VerifiablePresentationResponse
} from '@dfinity/verifiable-credentials/request-verifiable-presentation';
import { useMutation } from '@tanstack/react-query';
import { useAuth } from '@zk-game-dao/currency';
import {
  ButtonComponent, ErrorComponent, Interactable, Modal, ModalFooterPortal, UserError, useToast
} from '@zk-game-dao/ui';

import { callActorMutation } from '../../utils/call-actor-mutation';
import { UnwrapOptional } from '../../utils/optional';
import { useUser } from '../types';

export const VerifyModal = memo<{
  open: boolean;
  onClose(): void;
}>(({ open, onClose }) => {

  const { authData } = useAuth();
  const { user, actor } = useUser();
  const { addToast } = useToast();

  const verifyUserMutation = useMutation({
    mutationFn: async () => {
      if (!user) throw new UserError("No user");
      if (!actor) throw new UserError("No actor");
      if (!authData) throw new UserError("No auth data");
      if (UnwrapOptional(user.is_verified)) throw new UserError("User is already verified");

      if (authData.type !== 'internet_identity')
        throw new UserError("Verification is currently only supported for Internet Identity");

      const jwt: string = await new Promise((resolve, reject) => {
        requestVerifiablePresentation({
          onSuccess: async (verifiablePresentation: VerifiablePresentationResponse) => {
            if ('Ok' in verifiablePresentation) {
              resolve(verifiablePresentation.Ok);
            } else {
              reject(new Error(verifiablePresentation.Err));
            }
          },
          onError(err) {
            reject(new Error(err));
          },
          issuerData: {
            origin: 'https://id.decideai.xyz',
            canisterId: Principal.fromText('qgxyr-pyaaa-aaaah-qdcwq-cai'),
          },
          credentialData: {
            credentialSpec: {
              credentialType: 'ProofOfUniqueness',
              arguments: {},
            },
            credentialSubject: authData.principal,
          },
          identityProvider: new URL('https://identity.ic0.app/'),
          derivationOrigin: window.location.origin,
        });
      });

      return callActorMutation(
        actor,
        "verify_credential",
        user.principal_id,
        jwt,
        window.location.origin,
        authData.principal
      );
    },
    onSuccess: () => {
      addToast({ children: <>Verified user</> });
    }
  });

  if (!open || !authData) return null;

  if (authData.type !== 'internet_identity')
    return (
      <Modal open={open} onClose={onClose} title="Verify your identity">
        <p>Verification is currently only supported for Internet Identity accounts.</p>
      </Modal>
    )

  return (
    <Modal open={open} onClose={onClose} title="Verify your identity">

      {verifyUserMutation.error ? (
        <>
          <ErrorComponent error={verifyUserMutation.error} />

          <p className='type-body rounded-2xl border-purple-500 border-2 p-2'>
            It seems that there you have not yet verified your unique personhood.<br />
            Simply head over to <a className='underline hover:no-underline' href="https://id.decideai.xyz/">DecideAI</a>, sign in using the same Internet Identity that you used to access ZKP and follow the instructions to prove your unique personhood.<br />
            It's simple to do, takes only a few minutes and only needs to be done once.<br />
          </p>
        </>
      ) : (
        <>
          <p className='type-body rounded-2xl border-purple-500 border-2 p-2'>
            In order to verify that you are a real and unique person you will need to provide the "Unique Person" credentials that you receive from <Interactable isOutLink href="https://id.decideai.xyz/" className='underline hover:underline'>DecideAI</Interactable>.<br />
            Make sure to complete the above process before hitting 'Verify'.
          </p>
        </>
      )}

      <ModalFooterPortal>
        <ButtonComponent variant="naked" onClick={onClose}>Cancel</ButtonComponent>
        <ButtonComponent
          onClick={() => verifyUserMutation.mutate()}
          isLoading={verifyUserMutation.isPending}
        >
          Verify
        </ButtonComponent>
      </ModalFooterPortal>
    </Modal>
  );
});
VerifyModal.displayName = "VerifyModal";