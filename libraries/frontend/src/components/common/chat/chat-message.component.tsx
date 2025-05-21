import { memo } from "react";
import { ChatMessage } from "@declarations/table_canister/table_canister.did";
import { useFormatDateDistance } from "../../../hooks/countdown";
import { BigIntTimestampToDate } from "@zk-game-dao/ui";
import classNames from "classnames";
import { motion } from "framer-motion";

type Props = Pick<ChatMessage, 'content' | 'sender_name' | 'timestamp'> & {
  isFirst?: boolean;
  isLast?: boolean;
  isSelf?: boolean;
}

export const ChatMessageComponent = memo<Props>(({
  content,
  sender_name,
  timestamp,
  isFirst = true,
  isLast = true,
  isSelf = false
}) => {
  const dist = useFormatDateDistance(BigIntTimestampToDate(timestamp));

  return (
    <motion.div
      variants={{
        initial: { opacity: 0, x: isSelf ? 32 : -32 },
        animate: { opacity: 1, x: 0 },
      }}
      className={classNames(
        "flex flex-col gap-1 pointer-events-auto",
        isSelf ? ' pl-6' : 'pr-6',
        isLast ? 'mb-4' : 'mb-0.5',
      )}
    >
      <div
        className={classNames(
          "z-10 rounded-full px-4 gap-1 flex flex-col transition-all duration-75 border-2",
          isSelf ? 'bg-green-500/95 border-green-400 ml-auto' : 'bg-neutral-400/95 border-neutral-300/8 mr-auto',
          isFirst ? 'rounded-t-[16px] pt-1.5' : 'rounded-t-[4px] pt-1',
          isLast ? 'rounded-b-[16px] pb-1.5' : 'rounded-b-[4px] pb-1',
        )}
      >
        {isFirst && !isSelf && <div className="type-button-3 text-material-medium-1 ">{sender_name}</div>}
        <div>{content}</div>
      </div>
      {isLast && <div className={classNames("type-tiny px-4 text-material-main-3", {
        'text-right': isSelf,
      })}>{dist?.string} ago</div>}
    </motion.div>
  );
}, (prevProps, nextProps) => (
  prevProps.content === nextProps.content &&
  prevProps.timestamp === nextProps.timestamp &&
  prevProps.sender_name === nextProps.sender_name &&
  prevProps.isFirst === nextProps.isFirst &&
  prevProps.isLast === nextProps.isLast &&
  prevProps.isSelf === nextProps.isSelf
));
ChatMessageComponent.displayName = "ChatMessageComponent";
