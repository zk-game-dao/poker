import { memo, useMemo } from 'react';

import { ButtonComponent, LayoutComponent } from '@zk-game-dao/ui';
import { useIsBTC } from '@zk-game-dao/currency';

export const ContactPage = memo(() => {
  const isBTC = useIsBTC();
  const tmLink = useMemo(() => isBTC ? 'https://t.me/purepokerapp' : 'https://t.me/zkpokercommunity', [isBTC]);
  return (
    <LayoutComponent
      footer
      hero={{
        title: "Contact us.",
        subTitle:
          "Got questions or need assistance? Our team is here to help you with any inquiries related to our poker games or website.",
        children: (
          <div className="flex flex-col justify-center items-center">
            <ButtonComponent
              className="mx-auto"
              isOutLink
              href={tmLink}
            >
              Contact us on telegram
            </ButtonComponent>
          </div>
        ),
      }}
    />
  )
});
ContactPage.displayName = "ContactPage";

export default ContactPage;
