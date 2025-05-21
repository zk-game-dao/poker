import { callActorMutation } from "@/src/lib/utils/call-actor-mutation";

import { createActor } from "@declarations/users_canister";
import { User } from "@declarations/users_index/users_index.did";
import { useQuery } from "@tanstack/react-query";

import { Queries } from "@lib/data";
import { useIsBTC } from "@zk-game-dao/currency";

export const useUserExperiencePoints = ({
  principal_id,
  users_canister_id,
}: Partial<Pick<User, "principal_id" | "users_canister_id">> = {}) => {
  const isBTC = useIsBTC();
  return useQuery({
    queryKey: Queries.userExperiencePoints.key(principal_id),
    queryFn: () => {
      if (!users_canister_id) throw new Error("No users_canister_id provided");
      if (!principal_id) throw new Error("No principal_id provided");
      const actor = createActor(users_canister_id);
      return callActorMutation(
        actor,
        isBTC
          ? "get_pure_poker_experience_points_by_uid"
          : "get_experience_points_by_uid",
        principal_id
      );
    },
    refetchInterval: 1000 * 10, // 10 seconds
  });
};
