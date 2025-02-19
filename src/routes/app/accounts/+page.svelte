<script lang="ts">
    import TransactionsDataTable from '$lib/modules/accounts/components/transactions-datatable/TransactionsDataTable.svelte';
</script>

<div class="mb-10 flex items-end gap-2">
    <a href="/app/accounts/new" class="button-primary ml-auto"> Add Account </a>
</div>
<TransactionsDataTable />
