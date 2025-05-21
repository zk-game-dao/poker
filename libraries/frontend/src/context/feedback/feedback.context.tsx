import { createContext, memo, ReactNode, useContext, useState, lazy } from 'react';

const FeedbackModalComponent = lazy(
  () => import("./feedback-modal.component"),
);

type FeedbackContextType = {
  openFeedback(message?: string): void;
};

const FeedbackContext = createContext<FeedbackContextType>({
  openFeedback: () => { },
});

export const ProvideFeedbackContext = memo<{ children: ReactNode }>(
  ({ children }) => {
    const [state, setState] = useState<
      { isOpen: false } | { isOpen: true; message?: string }
    >({ isOpen: false });
    return (
      <>
        <FeedbackContext.Provider
          value={{
            openFeedback: (message) => setState({ isOpen: true, message }),
          }}
        >
          {children}
        </FeedbackContext.Provider>
        <FeedbackModalComponent
          onClose={() => setState({ isOpen: false })}
          {...state}
        />
      </>
    );
  },
);
ProvideFeedbackContext.displayName = 'ProvideFeedbackContext';

export const useFeedbackContext = () => useContext(FeedbackContext);
