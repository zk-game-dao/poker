import { memo, useState } from 'react';

import { ButtonComponent } from '@zk-game-dao/ui';
import { motion } from 'framer-motion';

export const ChatInputComponent = memo<{ isSending: boolean; onSend: (message: string) => void; }>(({ isSending, onSend }) => {
  const [content, setContent] = useState<string>();
  return (
    <motion.form
      variants={{
        initial: { opacity: 0 },
        animate: { opacity: 1 },
      }}
      initial='initial'
      animate='animate'
      exit="initial"
      className="flex flex-row gap-1 pointer-events-auto"
      onSubmit={(e) => {
        e.preventDefault();
        if (!content) return;
        onSend(content);
        setContent('');
      }}
    >
      <input
        className="w-full bg-material-main-1 rounded-l-full rounded-r-[8px] overflow-hidden pl-6 pr-3 py-2 text-material-main-3 text-white"
        value={content}
        onChange={(e) => setContent(e.target.value)}
      />
      <ButtonComponent className='rounded-r-full pl-3' isLoading={isSending} onClick={() => onSend('')}>
        Send
      </ButtonComponent>
    </motion.form>
  );
});
ChatInputComponent.displayName = 'ChatInputComponent';
