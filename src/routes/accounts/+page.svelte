<script lang="ts">
    import AccountsList from '$modules/accounts/components/AccountsList.svelte';
</script>

<div class="mb-10 flex justify-between gap-2">
    <h1 class="text-xl text-primary-800">All accounts</h1>
    <a href="/accounts/new" class="button-primary ml-auto"> Add Account </a>
</div>
<AccountsList />
