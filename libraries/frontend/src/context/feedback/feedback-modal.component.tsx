import axios from 'axios';
import { memo, useCallback, useEffect, useState } from 'react';

import { useMutation } from '@tanstack/react-query';
import {
  ButtonComponent, ErrorComponent, IsDev, Modal, ModalFooterPortal, TextInputComponent,
  useToast
} from '@zk-game-dao/ui';

import { ImageUploadComponent } from '../../components/common/input/image-upload.component';
import { APIUrl } from '../../lib/env/constants';

export const FeedbackModalComponent = memo<{
  isOpen: boolean;
  message?: string;
  onClose(): void;
}>(({ isOpen, message, onClose }) => {
  const { addToast } = useToast();

  const [feedback, setFeedback] = useState(typeof message === 'object' ? JSON.stringify(message) : message);
  const [imageUrl, setImageUrl] = useState<string | undefined>(IsDev ? 'https://pink-accessible-partridge-21.mypinata.cloud/ipfs/bafybeifbyn3bum2ypkvu3uoaqz5kkxgoeycx2eveidpmvfsa765vhrkhla' : undefined);

  useEffect(() => setFeedback(typeof message === 'object' ? JSON.stringify(message) : message), [message]);

  const {
    mutateAsync: submit,
    isPending,
    error,
    reset,
  } = useMutation({
    mutationFn: async () => {
      if (!feedback) throw new Error("Feedback is required");

      const body = {
        text: feedback,
        image_url: imageUrl,
      }

      // if (user)
      //   formData.append(
      //     "user",
      //     JSON.stringify({
      //       id: Number(user.principal_id),
      //       user_name: user.user_name,
      //     }),
      //   );

      await axios.post(`${APIUrl}/feedback`, body);

      return true;
    },
    onSuccess: () => {
      addToast({ children: "Feedback submitted" });
      close();
    },
  });

  const close = useCallback(() => {
    onClose();
    reset();
    setFeedback(undefined);
    setImageUrl(undefined);
  }, [onClose, reset]);

  return (
    <Modal title="Feedback" open={isOpen} onClose={close}>
      <div className="gap-3 flex flex-col justify-center items-center">
        <h1 className="text-center type-top">Spotted an issue?</h1>
        <p className="text-center type-body text-neutral-200/70 ">
          Let us know and we’ll fix it. If you have an idea on improvement or
          suggestion, let us know as well. We’re curious what you think.
        </p>
      </div>

      <TextInputComponent
        isArea
        label="Your feedback..."
        value={feedback}
        onChange={setFeedback}
      />
      {/* <PhotoInputComponent
        label="Upload optional picture"
        value={file}
        onChange={setFile}
      /> */}
      <ImageUploadComponent
        label="Upload optional picture"
        imageUrl={imageUrl}
        setImageUrl={setImageUrl}
      />

      <ErrorComponent error={error} />

      <ModalFooterPortal>
        <ButtonComponent onClick={close} variant="naked">
          Cancel
        </ButtonComponent>
        <ButtonComponent onClick={submit} isLoading={isPending}>
          Send feedback
        </ButtonComponent>
      </ModalFooterPortal>
    </Modal>
  );
});
FeedbackModalComponent.displayName = 'FeedbackModalComponent';

export default FeedbackModalComponent;
