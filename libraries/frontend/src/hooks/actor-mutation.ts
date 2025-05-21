import { queryClient } from "@lib/data";
import { QueryKey, useMutation } from "@tanstack/react-query";
import { useToast } from "@zk-game-dao/ui";
import { ReactNode, useCallback } from "react";

import {
  ActorMethodWithResult,
  callActorMutation,
} from "../lib/utils/call-actor-mutation";

export const useActorMutation = <
  Actor extends object,
  MethodName extends keyof Actor,
  Method extends Actor[MethodName] extends ActorMethodWithResult<any, any>
    ? Actor[MethodName]
    : never,
  Params extends Parameters<Method>,
  ReturnType extends Method extends ActorMethodWithResult<infer OkType, any>
    ? OkType
    : never,
  NormalizeParamsFN extends (...args: any[]) => Promise<Params> = (
    ...args: Params[]
  ) => Promise<Params>,
>(
  actor: Actor,
  methodName: MethodName,
  options?: Partial<{
    normalizeParams: NormalizeParamsFN;
    validateParams: (params: Params) => void;
    successToast: (result: ReturnType) => ReactNode;
    onSuccess: (result: ReturnType) => void;
    invalidateQueries: QueryKey[];
  }>
) => {
  const { addToast } = useToast();

  const mut = useMutation({
    mutationFn: async (p: Parameters<NormalizeParamsFN>) => {
      let params: Params = p as any;
      if (options?.normalizeParams) {
        params = (await options.normalizeParams(...(p as any))) as any;
      }

      if (options?.validateParams) {
        options.validateParams(params);
      }

      return await callActorMutation(actor, methodName, ...(params as any));
    },
    onSuccess: (result) => {
      if (options?.invalidateQueries) {
        options.invalidateQueries.forEach((queryKey) =>
          queryClient.invalidateQueries({ queryKey })
        );
      }
      if (options?.successToast) {
        addToast({ children: options.successToast(result) });
      }

      if (options?.onSuccess) {
        options.onSuccess(result);
      }
    },
  });

  const mutate = useCallback(
    async (...p: Parameters<NormalizeParamsFN>) => mut.mutate(p),
    [mut]
  );

  const mutateAsync = mutate;

  return {
    ...mut,
    mutate,
    mutateAsync,
  };
};
