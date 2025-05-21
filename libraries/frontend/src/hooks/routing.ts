import { useCallback } from "react";
import { useNavigate } from "react-router-dom";

export const useRouting = () => {
  const navigate = useNavigate();
  const getHref = useCallback(
    (path: string, includeOrigin?: boolean) =>
      includeOrigin ? `${window.location.origin}${path}` : path,
    []
  );
  const push = useCallback((path: string) => navigate(getHref(path)), []);

  return { push, getHref };
};
