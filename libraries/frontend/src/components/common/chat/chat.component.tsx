import { AnimatePresence, motion } from 'framer-motion';
import { memo } from 'react';

import {
  callActorMutation, ErrorComponent, LoadingSpinnerComponent, queryClient, useMutation, useQuery
} from '@zk-game-dao/ui';

import { useTable } from '../../../lib/table/context/table.context';
import { useUser } from '../../../lib/user';
import { ChatHistoryComponent } from './chat-history.component';
import { ChatInputComponent } from './chat-input.component';

export const ChatComponent = memo(() => {
  const { user: zkpUser } = useUser();
  const { actor, table, user } = useTable();

  const messages = useQuery({
    queryKey: ['chat-history', table.id.toText()],
    queryFn: async () => !actor ? [] : (
      (await callActorMutation(actor, 'get_recent_chat_messages', [], 100n))
        .sort((a, b) => Number(b.timestamp - a.timestamp))
    ),
    refetchInterval: 5000,
  })

  const sendMessageMutation = useMutation({
    mutationFn: (content: string) => {
      if (!zkpUser) throw new Error("No auth data");
      if (!actor) throw new Error("No actor");
      if (!content) throw new Error("No message");

      return callActorMutation(
        actor,
        'send_chat_message',
        zkpUser.principal_id,
        content,
        { TableMessage: null },
        []
      )
    },
    onSuccess: () => {
      console.log("Message sent successfully");
      queryClient.invalidateQueries({
        queryKey: ['chat-history', table.id.toText()]
      });
    }
  });

  return (
    <motion.div
      variants={{
        initial: { opacity: 0, x: 32 },
        animate: { opacity: 1, x: 0 },
      }}
      initial='initial'
      animate='animate'
      exit="initial"
      className='flex flex-col w-[320px] inset-y-0 right-0 pr-4 absolute pt-20 pb-18 z-[50] pointer-events-none gap-4'
    >
      <AnimatePresence >
        <motion.div
          variants={{
            initial: { opacity: 0 },
            animate: { opacity: 1 },
          }}
          initial='initial'
          animate='animate'
          style={{
            filter: 'blur(20px)',
            WebkitMask: 'linear-gradient(90deg, transparent, black 50%)',
          }}
          className='absolute -inset-[20px] z-0 bg-black/30'
        />
        {messages.isPending && <LoadingSpinnerComponent />}
        {messages.data && <ChatHistoryComponent messages={messages.data} />}
        <ErrorComponent error={messages.error || sendMessageMutation.error} />
        {zkpUser && user && <ChatInputComponent isSending={sendMessageMutation.isPending} onSend={sendMessageMutation.mutateAsync} />}
      </AnimatePresence>
    </motion.div>
  );
});
ChatComponent.displayName = 'ChatComponent';
