import { createActor } from "@declarations/users_canister";
import { Principal } from "@dfinity/principal";
import { callActorMutation, useQuery } from "@zk-game-dao/ui";

import { Queries } from "../../data/query.context";
import { useGetUsersCanisterFromUserId } from "../context/users-canisters.context";

export const useUserFromUserId = (user_id?: Principal) => {
  const getUsersCanisterFromUserId = useGetUsersCanisterFromUserId();
  return useQuery({
    queryKey: Queries.userFromUserId.key(user_id),
    queryFn: async () => {
      if (!user_id) throw new Error("user_id is required");
      const users_canister_id = await getUsersCanisterFromUserId(user_id);
      const actor = createActor(users_canister_id);
      return await callActorMutation(actor, "get_user", user_id);
    },
  });
};
