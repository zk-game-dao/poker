import { AnimatePresence, motion } from 'framer-motion';
import { memo, useEffect, useRef } from 'react';

import { ChatMessage } from '@declarations/table_canister/table_canister.did';

import { useUser } from '../../../lib/user';
import { ChatMessageComponent } from './chat-message.component';

// Compare if 2 messages are part of the same group
// It has to be the same sender and the timestamp should be within 30 seconds
// Timestamps are nanosec bigint
const getIsPartOfGroup = (
  prev: ChatMessage,
  next: ChatMessage
) => {
  if (prev.sender.compareTo(next.sender) !== 'eq') return false;
  const thirtySecondsInNanoseconds = BigInt(30 * 1_000_000_000);
  return (next.timestamp - prev.timestamp) <= thirtySecondsInNanoseconds;
}

export const ChatHistoryComponent = memo<{ messages: ChatMessage[] }>(({
  messages
}) => {
  const { user } = useUser();

  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (ref.current) {
      requestAnimationFrame(() => {
        ref.current?.scrollIntoView({
          behavior: 'smooth',
          block: 'end',
          inline: 'end'
        });
      })
    }
  }, [ref.current, messages.length]);

  return (
    <div
      className='flex flex-col basis-full overflow-y-scroll overflow-x-hidden pt-[32px]'
      style={{
        mask: 'linear-gradient(180deg, transparent, black 32px)',
      }}
    >
      <motion.div
        className="flex flex-col-reverse basis-full justify-end z-[50] relative"
        variants={{
          initial: { opacity: 0, x: 32 },
          animate: { opacity: 1, x: 0, transition: { staggerChildren: 0.03 } },
        }}
        initial='initial'
        animate='animate'
        exit="initial"
        ref={ref}
      >
        <AnimatePresence>
          {messages.length === 0 && (
            <motion.p
              variants={{
                initial: { opacity: 0, y: 32 },
                animate: { opacity: 1, y: 0 },
              }}
              initial='initial'
              animate='animate'
              exit="initial"
              className="type-button-3 text-material-medium-1 text-center mt-4 h-full flex items-center justify-center"
            >
              No messages yet.
            </motion.p>
          )}
          {messages.map((message, index, arr) => {
            // Its in reverse
            const prevMessage = arr[index + 1];
            const nextMessage = arr[index - 1];
            return (
              <ChatMessageComponent
                {...message}
                key={message.id}
                isFirst={!prevMessage || !getIsPartOfGroup(prevMessage, message)}
                isLast={nextMessage && !getIsPartOfGroup(message, nextMessage)}
                // isFirst={index === 0 || arr[index - 1].sender.compareTo(message.sender) !== 'eq'}
                // isLast={index === arr.length - 1 || arr[index + 1].sender.compareTo(message.sender) !== 'eq'}
                isSelf={user && message.sender.compareTo(user.principal_id) === 'eq'}
              />
            )
          })}
        </AnimatePresence>
      </motion.div>
    </div>
  )
});
ChatHistoryComponent.displayName = 'ChatHistoryComponent';
