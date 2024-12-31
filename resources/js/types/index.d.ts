import { Config } from 'ziggy-js';

export interface User {
    uuid: number;
    name: string;
    email: string;
    email_verified_at: bool;
    name: string;
    active: boolean;
    avatar?: string;
}

export type PageProps<T extends Record<string, unknown> = Record<string, unknown>> = T & {
    auth: {
        user: User;
    };
    ziggy: Config & { location: string };
    appName: string;
};
