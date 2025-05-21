import { motion } from "framer-motion";
import { memo } from "react";

export const NotificationIndicatorComponent = memo(() =>
  <motion.span
    variants={{
      hidden: { scale: 0, x: '100%', y: '-50%' },
      visible: { scale: 1, x: '100%', y: '-50%' },
    }}
    initial={false}
    animate="visible"
    exit="hidden"
    transition={{ type: "spring", stiffness: 300, damping: 20 }}
    className='size-2 rounded-full bg-red-500 material absolute right-0 top-0 origin-center'
  />
);
NotificationIndicatorComponent.displayName = "NotificationIndicatorComponent";