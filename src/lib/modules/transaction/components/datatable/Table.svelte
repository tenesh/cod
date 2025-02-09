<script lang="ts">
    import { ArrowUp, ArrowDown, Eye } from 'lucide-svelte';
    import Badge from '$lib/common/components/badge/Badge.svelte';
</script>

<div class="table-wrapper">
    <table>
        <thead class="">
        <tr class="row head-row">
            <th class="column action-column">
                <input type="checkbox" id="" class="checkbox" />
            </th>
            <th class="column text-column date-column">
                <p>Created at</p>
                <button>
                    <ArrowUp size={14} class="mt-0.5" />
                </button>
            </th>
            <th class="column text-column type-column">
                <p>Type</p>
            </th>
            <th class="column text-column party-column">
                <p>Party</p>
                <button>
                    <ArrowUp size={14} class="mt-0.5" />
                </button>
            </th>
            <th class="column text-column amount-column">
                <p>Amount</p>
                <button>
                    <ArrowUp size={14} class="mt-0.5" />
                </button>
            </th>
            <th class="column text-column medium-column">
                <p>Medium</p>
            </th>
            <th class="column badge-column category-column">
                <p>Categories</p>
            </th>
            <th class="column text-column note-column">
                <p>Note</p>
            </th>
            <th class="column text-column status-column">
                <p>Status</p>
            </th>
            <th class="column action-column">
            </th>
        </tr>
        </thead>
        <tbody>
        <tr class="row body-row">
            <td class="column checkbox-column action-column">
                <input type="checkbox" id="" class="checkbox" />
            </td>
            <td class="column text-column date-column">
                <p>25 January 2025</p>
            </td>
            <td class="column text-column type-column">
                <Badge text="Expense" size="sm" />
            </td>
            <td class="column text-column party-column">
                <p>Ramlee bukit bintang bills</p>
            </td>
            <td class="column text-column amount-column">
                <p>RM 3.00</p>
            </td>
            <td class="column text-column medium-column">
                <p>Credit card</p>
            </td>
            <td class="column badge-column category-column">
                <Badge text="ipsum dolor" size="sm" />
                <Badge text="Household" size="sm" />
                <Badge text="Shopping" size="sm" />
            </td>
            <td class="column text-column note-column">
                <p>
                    Lorem, ipsum dolor sit amet consectetur adipisicing elit. Ducimus facere beatae delectus quo
                    aspernatur impedit, exercitationem quaerat.
                </p>
            </td>
            <td class="column text-column status-column">
                <Badge text="Completed" size="sm" />
            </td>
            <td class="column action-column">
                <button>
                    <Eye size={16} />
                </button>
            </td>
        </tr>
        </tbody>
    </table>
</div>

<style lang="postcss">

	.table-wrapper {
		@apply flex w-full flex-col rounded-lg border border-secondary-100 overflow-x-auto;
	}

	.checkbox {
		cursor: pointer;
		@apply w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded-sm focus:ring-transparent;
	}

	table {
		min-width: 1200px;
	}

	tbody tr:not(:last-child) {
		@apply border-b border-secondary-100;
	}

	.row {
		@apply flex gap-4 justify-start items-start hover:bg-primary-50;
	}

	.head-row {
		@apply bg-primary-100 text-neutral-600 text-sm p-3;
	}

	.body-row {
		@apply text-neutral-600 text-sm p-3;
	}

	.column {
		@apply flex flex-wrap gap-2 w-[100px];
	}

	.action-column {
		@apply p-0 mt-1 w-[20px];
	}

	.type-column,
	.status-column,
	.date-column,
	.medium-column {
		@apply w-[150px];
	}

	.amount-column,
	.party-column,
	.category-column {
		@apply w-[250px];
	}

	.note-column {
		@apply flex-grow min-w-[300px];
	}
</style>
