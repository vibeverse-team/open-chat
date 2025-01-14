import type { Failure, Success } from "./response";

export const E8S_PER_TOKEN = 100_000_000;

export const CHAT_SYMBOL = "CHAT";
export const ICP_SYMBOL = "ICP";
export const CKBTC_SYMBOL = "ckBTC";
export const SNS1_SYMBOL = "SNS1";
export const KINIC_SYMBOL = "KINIC";
export const HOTORNOT_SYMBOL = "HOT";
export const GHOST_SYMBOL = "GHOST";

export const LEDGER_CANISTER_ICP = "ryjl3-tyaaa-aaaaa-aaaba-cai";
export const LEDGER_CANISTER_CHAT = "2ouva-viaaa-aaaaq-aaamq-cai";

export type CryptocurrencyDetails = {
    name: string;
    symbol: string;
    ledger: string;
    decimals: number;
    transferFee: bigint;
    logo: string;
    howToBuyUrl: string;
    infoUrl: string;
    transactionUrlFormat: string;
    added: bigint;
    lastUpdated: bigint;
};

export type NervousSystemSummary = {
    rootCanisterId: string;
    governanceCanisterId: string;
    ledgerCanisterId: string;
    indexCanisterId: string;
    isNns: boolean;
    proposalRejectionFee: bigint;
    submittingProposalsEnabled: boolean;
};

export type NervousSystemDetails = {
    governanceCanisterId: string;
    rootCanisterId: string;
    ledgerCanisterId: string;
    indexCanisterId: string;
    isNns: boolean;
    proposalRejectionFee: bigint;
    submittingProposalsEnabled: boolean;
    token: CryptocurrencyDetails;
};

// approximate dollar exchange rates - until we come up with something better
const dollarToICP = 0.34;

export const dollarExchangeRates: Record<string, number> = {
    icp: to2SigFigs(dollarToICP),
    chat: to2SigFigs(dollarToICP / 0.04805),
    hot: to2SigFigs(dollarToICP / 0.003),
    kinic: to2SigFigs(dollarToICP / 0.378),
    ckbtc: to2SigFigs(dollarToICP / 7777.004),
    dkp: to2SigFigs(dollarToICP / 280),
    ghost: to2SigFigs(dollarToICP / 0.00001685),
    mod: to2SigFigs(dollarToICP / 0.0065),
    cat: to2SigFigs(dollarToICP / 0.0068998),
    boom: to2SigFigs(dollarToICP / 0.00339),
    icx: to2SigFigs(dollarToICP / 0.0055),
    nua: to2SigFigs(dollarToICP / 0.0134),
    sonic: to2SigFigs(dollarToICP / 0.053),
};

function to2SigFigs(num: number): number {
    return parseFloat(num.toPrecision(2));
}

type AccountTransactionCommon = {
    timestamp: Date;
    id: bigint;
    memo?: string;
    createdAt?: Date;
    amount: bigint;
    to?: string;
    from?: string;
};

type AccountTransactionBurn = AccountTransactionCommon & {
    kind: "burn";
    spender?: string;
};

type AccountTransactionMint = AccountTransactionCommon & {
    kind: "mint";
};

type AccountTransactionApprove = AccountTransactionCommon & {
    kind: "approve";
    fee?: bigint;
    expectedAllowance?: bigint;
    expiredAt?: bigint;
    spender?: string;
};

type AccountTransactionTransfer = AccountTransactionCommon & {
    kind: "transfer";
    fee?: bigint;
    spender?: string;
};

export type AccountTransaction =
    | AccountTransactionBurn
    | AccountTransactionMint
    | AccountTransactionTransfer
    | AccountTransactionApprove;

export type AccountTransactions = {
    transactions: AccountTransaction[];
    oldestTransactionId?: bigint;
};

export type AccountTransactionResult = Failure | (Success & AccountTransactions);
