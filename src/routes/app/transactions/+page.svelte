<script lang="ts">
    import DataTable from '$lib/modules/transaction/components/datatable/DataTable.svelte';
</script>

<div class="mb-10 flex items-end gap-2">
    <a href="/app/transactions/new" class="button-primary ml-auto"> Add Transaction </a>
</div>
<DataTable />
