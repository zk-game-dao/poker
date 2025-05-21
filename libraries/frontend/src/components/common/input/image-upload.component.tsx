import { PinataSDK } from 'pinata-web3';
import { memo, ReactNode } from 'react';

import { useMutation } from '@tanstack/react-query';
import {
  ErrorComponent, Image, Interactable, Label, LoadingAnimationComponent, PillComponent
} from '@zk-game-dao/ui';

const PinataApiJWT = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiI1ZGVmN2M4Yi0wMmQ2LTQ5Y2YtYWZiMS1mNWQ5NTcyMjQyNTMiLCJlbWFpbCI6InB1cmVzYW1hcmlAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsInBpbl9wb2xpY3kiOnsicmVnaW9ucyI6W3siZGVzaXJlZFJlcGxpY2F0aW9uQ291bnQiOjEsImlkIjoiRlJBMSJ9LHsiZGVzaXJlZFJlcGxpY2F0aW9uQ291bnQiOjEsImlkIjoiTllDMSJ9XSwidmVyc2lvbiI6MX0sIm1mYV9lbmFibGVkIjpmYWxzZSwic3RhdHVzIjoiQUNUSVZFIn0sImF1dGhlbnRpY2F0aW9uVHlwZSI6InNjb3BlZEtleSIsInNjb3BlZEtleUtleSI6Ijg5MWMwNDE2ZGRhYWEyYzljYWMzIiwic2NvcGVkS2V5U2VjcmV0IjoiZWQ1YjFmYWRlOTA4MjVlYzNmZDgwY2UxNzM2NDM5Y2UyNTM2OTE3MGY1NzliMTU4MmFjZWE4YjAzYmM4MjU3ZSIsImV4cCI6MTc3MTQ5NTgwOX0.FJ7uQ3Qjlr70EPsoikInTYiOIjI6vophcu_OjipCOgE';
const PinataGateway = 'pink-accessible-partridge-21.mypinata.cloud';

const pinata = new PinataSDK({
  pinataJwt: `${PinataApiJWT}`,
  pinataGateway: `${PinataGateway}`,
})

export const ImageUploadComponent = memo<{ label?: ReactNode; imageUrl?: string; setImageUrl(imageUrl?: string): void }>(({ label, imageUrl, setImageUrl }) => {

  const dropMutation = useMutation({
    mutationFn: async (e: React.DragEvent<HTMLDivElement>) => {
      setImageUrl(undefined);
      if ('preventDefault' in e) e.preventDefault();
      const file = e.dataTransfer.files[0];
      if (!file) throw new Error("No file dropped");

      const upload = await pinata.upload.file(file);
      return await pinata.gateways.convert(upload.IpfsHash)
    },
    onSuccess: (url) => setImageUrl(url)
  });

  return (
    <>
      <ErrorComponent error={dropMutation.error} />
      <div className='flex flex-col gap-2'>
        {label && <Label>{label}</Label>}
        <div
          className='bg-material-main-1 rounded-[12px] p-4 flex w-full flex-col gap-6 max-h-[200px] min-h-[132px] justify-center items-center cursor-pointer w-'
          onDrop={dropMutation.mutateAsync}
          onDragOver={(e) => e.preventDefault()}
        >
          {/* eslint-disable-next-line no-extra-boolean-cast */}
          {(!!imageUrl) ?
            <div className='relative h-full'>
              <img src={imageUrl} alt="Uploaded" className='h-32 rounded-[10px]' />
              <Interactable
                className='absolute top-0 right-0 bg-material-main-1 rounded-full p-1'
                onClick={() => setImageUrl(undefined)}
              >
                <Image src='/icons/xmark.svg' alt='Close' type="svg" />
              </Interactable>
            </div> :
            (
              <>
                {dropMutation.isPending ? <LoadingAnimationComponent variant="shimmer">Uploading</LoadingAnimationComponent> : (
                  <>
                    <p className='full type-button-3 text-material-medium-1 '>Drop here or...</p>
                    <PillComponent
                      onClick={() => {
                        const input = document.createElement('input');
                        input.type = 'file';
                        input.accept = 'image/*';
                        input.onchange = (e) => {
                          const file = (e.target as HTMLInputElement).files?.[0];
                          if (!file) return;
                          dropMutation.mutateAsync({ dataTransfer: { files: [file] } } as any);
                        }
                        input.click();
                      }}
                    >
                      Select
                    </PillComponent>
                  </>
                )}
              </>
            )}
        </div>
      </div>

    </>
  );
});
ImageUploadComponent.displayName = 'ImageUploadComponent';
