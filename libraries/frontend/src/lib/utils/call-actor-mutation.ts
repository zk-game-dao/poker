export type ActorMethodWithResult<OkType, ErrType> = (
  ...args: any[]
) => Promise<{ Ok: OkType } | { Err: ErrType }>;

export async function callActorMutation<
  Actor extends object,
  MethodName extends keyof Actor,
  Method extends Actor[MethodName] extends ActorMethodWithResult<any, any>
    ? Actor[MethodName]
    : never,
>(
  actor: Actor,
  methodName: MethodName,
  ...args: Parameters<Method> extends [] ? [] : Parameters<Method>
): Promise<
  Method extends ActorMethodWithResult<infer OkType, any> ? OkType : never
> {
  const method = actor[methodName] as Method;
  const result = await method(...(args as any));

  if ("Err" in result) throw result.Err;

  return result.Ok;
}
