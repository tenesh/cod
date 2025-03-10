<script lang="ts">
    import TransactionsList from '$modules/transactions/components/TransactionsList.svelte';
</script>

<div class="mb-10 flex items-end gap-2">
    <a href="/accounts/new" class="button-primary ml-auto"> Add Transaction </a>
</div>
<TransactionsList />
