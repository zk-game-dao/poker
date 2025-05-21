declare module "random-bigint" {
  export default function RandomBigInt(size: number): bigint;
}

declare module "ic-stoic-identity" {
  import { Principal } from "@dfinity/principal";
  import { SignIdentity } from "@dfinity/agent";

  export class StoicIdentity extends SignIdentity {
    private _principal: Principal;
    private _publicKey: PublicKey;

    constructor(principal: Principal, pubkey: PublicKey);

    static disconnect(): void;

    static connect(host?: string): Promise<StoicIdentity>;

    static load(host?: string): Promise<StoicIdentity | false>;

    getPublicKey(): PublicKey;

    sign(data: Uint8Array): Promise<string>;

    accounts(): Promise<any>;

    transformRequest(request: Request): Promise<Request>;

    private _transport(data: string): Promise<any>;
  }

  export class PublicKey {
    private _der: Uint8Array;
    private _type: string;

    constructor(der: Uint8Array, type: string);

    getType(): string;

    toDer(): Uint8Array;
  }

  interface Request {
    body: any;
    [key: string]: any;
  }
}

/// <reference types="vite/client" />

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

declare module "@analytics/google-tag-manager" {
  type AnalyticsPlugin = import("analytics").AnalyticsPlugin;

  type GoogleTagManagerConfig = {
    auth?: string;
    containerId: string;
    customScriptSrc?: string;
    dataLayerName?: string;
    debug?: boolean;
    execution?: string;
    preview?: string;
  };

  function googleTagManager(config: GoogleTagManagerConfig): AnalyticsPlugin;
  export default googleTagManager;
}

declare module "@analytics/google-analytics" {
  type GoogleAnalyticsOptions = {
    /** Google Analytics MEASUREMENT IDs */
    measurementIds: string[];

    /** Enable Google Analytics debug mode */
    debug?: boolean;

    /** The optional name for dataLayer object. Defaults to 'ga4DataLayer'. */
    dataLayerName?: string;

    /** The optional name for the global gtag function. Defaults to 'gtag'. */
    gtagName?: string;

    /** Configuration for gtag, including anonymizing IP and cookie settings */
    gtagConfig?: {
      anonymize_ip?: boolean;
      cookie_domain?: string;
      cookie_expires?: number;
      cookie_prefix?: string;
      cookie_update?: boolean;
      cookie_flags?: string;
    };

    /** Custom URL for google analytics script, if proxying calls */
    customScriptSrc?: string;
  };

  type AnalyticsPlugin = {
    /** Name of plugin */
    name: string;

    /** Exposed events of the plugin */
    EVENTS?: unknown;

    /** Configuration of the plugin */
    config?: unknown;

    /** Method to load analytics scripts */
    initialize?: (...params: unknown[]) => unknown;

    /** Page visit tracking method */
    page?: (...params: unknown[]) => unknown;

    /** Custom event tracking method */
    track?: (...params: unknown[]) => unknown;

    /** User identify method */
    identify?: (...params: unknown[]) => unknown;

    /** Function to determine if analytics script is loaded */
    loaded?: (...params: unknown[]) => unknown;

    /** Fire function when the plugin is ready */
    ready?: (...params: unknown[]) => unknown;
  };

  function GoogleAnalytics(options: GoogleAnalyticsOptions): AnalyticsPlugin;
  export default GoogleAnalytics;
}

/// <reference types="vite/client" />

// Describes metadata related to a provider based on EIP-6963.
interface EIP6963ProviderInfo {
  rdns: string;
  uuid: string;
  name: string;
  icon: string;
}

// Represents the structure of a provider based on EIP-1193.
interface EIP1193Provider {
  isStatus?: boolean;
  host?: string;
  path?: string;
  sendAsync?: (
    request: { method: string; params?: Array<unknown> },
    callback: (error: Error | null, response: unknown) => void
  ) => void;
  send?: (
    request: { method: string; params?: Array<unknown> },
    callback: (error: Error | null, response: unknown) => void
  ) => void;
  request: (request: {
    method:
      | "eth_sendTransaction"
      | "eth_requestAccounts"
      | "eth_getTransactionCount"
      | "wallet_revokePermissions";
    params?: Array<unknown>;
  }) => Promise<unknown>;
}

// Combines the provider's metadata with an actual provider object, creating a complete picture of a
// wallet provider at a glance.
interface EIP6963ProviderDetail {
  info: EIP6963ProviderInfo;
  provider: EIP1193Provider;
}

// Represents the structure of an event dispatched by a wallet to announce its presence based on EIP-6963.
type EIP6963AnnounceProviderEvent = {
  detail: {
    info: EIP6963ProviderInfo;
    provider: Readonly<EIP1193Provider>;
  };
};

// An error object with optional properties, commonly encountered when handling eth_requestAccounts errors.
interface WalletError {
  code?: string;
  message?: string;
}
