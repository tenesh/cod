<script lang="ts">
    import Breadcrumb from '$lib/components/topbar/Breadcrumb.svelte';
    import Notification from '$lib/components/topbar/Notification.svelte';
</script>

<div
    id="top-bar"
    class="fixed left-[250px] flex h-[76px] w-full items-center justify-between border-b border-b-secondary-100 p-4"
>
    <Breadcrumb />
    <div class="flex gap-2">
        <Notification />
    </div>
</div>
