<script lang="ts">
    import TransactionsFilters from '$modules/transactions/components/TransactionsFilters.svelte';
    import TransactionsTable from '$modules/transactions/components/TransactionsTable.svelte';
    import TransactionsPagination from '$modules/transactions/components/TransactionsPagination.svelte';
</script>

<div class="flex flex-col gap-4">
    <div class="space-between flex items-center gap-4">
        <TransactionsFilters />
    </div>
    <TransactionsTable />
    <TransactionsPagination />
</div>
