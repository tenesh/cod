<script lang="ts">
    import { createAvatar } from '@dicebear/core';
    import { micah } from '@dicebear/collection';

    let users = [{ username: 'tenesh' }, { username: 'demo' }, { username: 'jane_doe' }, { username: 'maple syeiu' }];

    const generateAvatar = (text: string) => {
        const avatar = createAvatar(micah, {
            seed: text,
            size: 80,
        });

        return avatar.toDataUri();
    };
</script>

<header></header>
<main class="flex min-h-screen w-full">
    <div class="flex w-full flex-col items-center justify-center p-20 xl:max-w-[1800px]">
        <p class="text-6xl font-bold text-primary-brand">netbalance</p>
        <p class="mt-2">Your personal financial companion — track, plan, and grow with confidence.</p>
        <div class="mt-20 grid grid-cols-4 gap-4 overflow-y-auto p-4">
            {#each users as user}
                <button
                    class="flex h-[150px] w-[150px] cursor-pointer flex-col items-center justify-center space-y-2 rounded-lg p-2 shadow-lg hover:bg-primary-50"
                    onclick={() => {}}
                >
                    <img src={generateAvatar(user.username)} alt="Avatar" class="h-14 w-14 rounded-full" />
                    <span class="text-sm font-medium">{user.username}</span>
                </button>
            {/each}
            <button
                class="flex h-[150px] w-[150px] cursor-pointer flex-col items-center justify-center space-y-2 rounded-lg p-2 shadow-lg hover:bg-primary-50"
                onclick={() => {}}
            >
                <span class="text-sm font-medium">Add new</span>
            </button>
        </div>
    </div>
    <div class="bg-primary-100 xl:w-full"></div>
</main>
<footer></footer>
