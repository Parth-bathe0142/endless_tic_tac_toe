<script lang="ts">
    import "../app.css";
    import Square from "./Square.svelte";
    const board = $state([1, 2, 3, 4, 5, 6, 7, 8, 9]);

    const handleClick = (index: number) => {
        board[index] += 1;
    };
</script>

<div
    class="grid grid-cols-3 grid-rows-3 bg-gray-900 w-fit gap-1 p-1 outline-2 outline-gray-700 rounded-xs"
>
    {#each board as value, index}
        <Square {handleClick} {value} {index} />
    {/each}
</div>
