import { createContext, memo, ReactNode, useContext, useRef } from 'react';

import { User } from '@declarations/users_canister/users_canister.did';
import { users_index } from '@declarations/users_index';
import { Principal } from '@dfinity/principal';
import { callActorMutation } from '@zk-game-dao/ui';

type UsersCanistersMap = Map<Principal, Principal>;

type UsersCanistersContextType = {
  getUsersCanisterFromUserId(
    principal: Principal,
  ): Promise<Principal>;
  storeUser: (user: Pick<User, 'users_canister_id' | 'principal_id'>) => void;
};

const UsersCanistersContext = createContext<UsersCanistersContextType>({
  getUsersCanisterFromUserId: async () => {
    throw new Error("getUserCanisterFromUserId not implemented");
  },
  storeUser: () => {
    throw new Error("addUser not implemented");
  }
});

function SaveUsersCainstersMapInLocalStorage(map: UsersCanistersMap) {
  const mapObj: Record<string, string> = {};
  for (const [key, value] of map.entries()) {
    mapObj[key.toText()] = value.toText();
  }
  localStorage.setItem('usersCanistersMap', JSON.stringify(mapObj));
}

function LoadUsersCainstersMapFromLocalStorage(): UsersCanistersMap {
  const mapObj = localStorage.getItem('usersCanistersMap');
  if (!mapObj) return new Map();
  const parsedMapObj = JSON.parse(mapObj) as Record<string, string>;
  const map = new Map<Principal, Principal>();
  for (const [key, value] of Object.entries(parsedMapObj)) {
    map.set(Principal.fromText(key), Principal.fromText(value));
  }
  return map;
}

export const ProvideUsersCanisters = memo<{ children: ReactNode }>(({ children }) => {
  const ref = useRef<UsersCanistersMap>(LoadUsersCainstersMapFromLocalStorage());

  const storeUser = ({ users_canister_id, principal_id }: Pick<User, 'users_canister_id' | 'principal_id'>): Principal => {
    ref.current.set(principal_id, users_canister_id);
    SaveUsersCainstersMapInLocalStorage(ref.current);
    return users_canister_id;
  };

  const getUserCanisterFromUserId = async (principal_id: Principal) => {
    if (ref.current.has(principal_id))
      return ref.current.get(principal_id)!;
    const users_canister_id = await callActorMutation(
      users_index,
      'get_users_canister_principal_by_id',
      principal_id,
    );
    return storeUser({ users_canister_id, principal_id });
  };

  return (
    <UsersCanistersContext.Provider value={{ getUsersCanisterFromUserId: getUserCanisterFromUserId, storeUser }}>
      {children}
    </UsersCanistersContext.Provider>
  );
});
ProvideUsersCanisters.displayName = "ProvideUsersCanisters";

export const useUsersCanisters = () => useContext(UsersCanistersContext);
export const useGetUsersCanisterFromUserId = () => useUsersCanisters().getUsersCanisterFromUserId;
