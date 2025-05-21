declare global {
  interface Window {
    ic: {
      plug: {
        isConnected: () => Promise<boolean>;
        requestConnect: (options: { host: string }) => Promise<void>;
      };
    };
  }
}

declare module "*.md" {
  const markdown: string;
  export { markdown };
}
