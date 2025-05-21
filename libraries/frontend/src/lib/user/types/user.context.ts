import { _SERVICE, User } from "@declarations/users_canister/users_canister.did";
import { createContext, useContext } from "react";

export const WalletTypeLocalStorageKey = "zkp:wallet-type-v1";

const UserContext = createContext<{
  user?: User;
  actor?: _SERVICE;
  isLoading: boolean;
  show(): void;
  /** @deprecated */
  showProfile(): void;
  /** @deprecated */
  showSignup(): void;
}>({
  isLoading: true,
  show: () => {},
  showProfile: () => {},
  showSignup: () => {},
});

export const {
  Provider: RawUserContextProvider,
  Consumer: UserContextConsumer,
} = UserContext;

export const useUser = () => {
  const context = useContext(UserContext);
  if (!context) throw new Error("useUser must be used within a UserProvider");
  return context;
};
