import type { User } from '$modules/user/types';

let auth = $state<{ user?: User }>({ user: undefined });

export const authState = {
    get auth() {
        return auth;
    },
    set auth(value) {
        auth = value;
    },
    setUser(user: User) {
        auth.user = user;
    },
    getUser() {
        return auth.user;
    },
};
