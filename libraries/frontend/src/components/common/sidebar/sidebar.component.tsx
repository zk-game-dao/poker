import classNames from 'classnames';
import { memo, ReactNode } from 'react';
import { useMatch } from 'react-router-dom';

import { Interactable } from '@zk-game-dao/ui';

export type SidebarItem = {
  type: 'tab' | 'link';
  icon: ReactNode;
  title: ReactNode;
  value: string;
  disabled?: boolean;
  hidden?: boolean;
  content?: ReactNode;
};

const SidebarItemComponent = memo<SidebarItem & { activeItemValue?: string; setActiveItem?(value: string): void; }>(({ activeItemValue, setActiveItem, ...item }) => {
  const match = useMatch(item.value);

  return (
    <Interactable
      className={classNames(
        'flex whitespace-nowrap h-[44px] items-center flex-row px-4 gap-2 type-subheadline rounded-[8px]',
        { 'bg-material-main-1': activeItemValue === item.value || (item.type === 'link' && match) }
      )}
      onClick={setActiveItem && item.type === 'tab' ? () => setActiveItem(item.value) : undefined}
      href={item.type === 'link' ? item.value : undefined}
      disabled={item.disabled}
    >
      <div className='flex w-[24px] justify-center items-center flex-col '>
        {item.icon}
      </div>
      <span className='hidden lg:inline'>
        {item.title}
      </span>
    </Interactable>
  );
});
SidebarItemComponent.displayName = 'SidebarItemComponent';

export const SidebarComponent = memo<{
  activeItemValue?: string;
  items: SidebarItem[];
  setActiveItem?(item: string): void;
}>(({ items, activeItemValue, setActiveItem }) => (
  <div className='flex flex-col'>
    {items.filter((v) => !v.hidden).map((item, i) => (
      <SidebarItemComponent
        key={i}
        activeItemValue={activeItemValue}
        setActiveItem={setActiveItem}
        {...item}
      />
    ))}
  </div>
));
SidebarComponent.displayName = 'SidebarComponent';
