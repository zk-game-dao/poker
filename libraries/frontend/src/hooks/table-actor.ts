import { useMemo } from "react";

import { createActor } from "@declarations/table_canister";
import { Principal } from "@dfinity/principal";

export const useTableActor = (principal?: Principal) =>
  useMemo(() => principal && createActor(principal), [principal]);
