import { Config } from 'ziggy-js';

export interface User {
    uuid: number;
    name: string;
    email: string;
    email_verified_at?: string;
    username?: string;
    active?: boolean;
}

export type PageProps<T extends Record<string, unknown> = Record<string, unknown>> = T & {
    auth: {
        user: User;
    };
    ziggy: Config & { location: string };
    appName: string;
};
