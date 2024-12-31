<script lang="ts">
    import { page } from '@inertiajs/svelte';
    import UserLayout from '@/layouts/UserLayout.svelte';
</script>

<svelte:head>
    <title>Dashboard</title>
    <meta
        name="description"
        content="Get a complete overview of your finances with the NetBalance dashboard. Gain insights, track goals, and make data-driven decisions."
    />
</svelte:head>

<UserLayout>
    <p>{$page.props.auth.user.name}</p>
</UserLayout>
