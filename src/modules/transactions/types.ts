import type { Tag } from '$modules/tags/types';

export interface Transaction {
    id: number;
    party: string;
    description?: string;
    currency_id: number;
    conversion_rate: number;
    amount: number;
    category: TransactionCategory;
    medium: TransactionMedium;
    status: TransactionStatus;
    account_id: number;
    transacted_at: number;
    created_at: string;
    updated_at: string;
    tags: Tag[];
}

export enum TransactionStatus {
    Completed = 'completed',
    Declined = 'declined',
    Pending = 'pending',
}

export enum TransactionCategory {
    Inflow = 'inflow',
    Outflow = 'outflow',
}

export enum TransactionMedium {
    Cash = 'cash',
    BankTransfer = 'bank transfer',
    CreditCard = 'credit card',
    DebitCard = 'debit card',
    DigitalPayment = 'digital payment',
    Crypto = 'crypto',
    Check = 'check',
    MobileWallet = 'mobile wallet',
    Other = 'other',
}
