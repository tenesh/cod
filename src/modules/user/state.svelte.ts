import type { User } from '$modules/user/types';

let user = $state<User | null>(null);

export const userState = {
    get user() {
        return user;
    },
    set user(value) {
        user = value;
    },
};
