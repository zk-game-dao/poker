import { LayoutComponent, SmallHeroComponent } from '@zk-game-dao/ui';
import { memo } from 'react';

export const StorePage = memo(() => (
  <LayoutComponent footer className="items-center justify-center">
    <SmallHeroComponent
      title="The Store is coming soon."
      subtitle="Our store will soon feature a new selection of exclusive NFTs."
      icon={{
        type: "svg",
        src: "/icons/store-white-large.svg",
        alt: "Coming soon",
      }}
      onBack={() => window.history.back()}
    />
  </LayoutComponent>
));
StorePage.displayName = "StorePage";

export default StorePage;
