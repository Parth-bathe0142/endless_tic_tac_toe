<script lang="ts">
    import "../app.css";
    const { value, handleClick, index } = $props();
</script>

<div class="h-16 w-16 outline-2 outline-gray-700 text-white">
    <button
        onclick={() => handleClick(index)}
        class="h-full w-full flex items-center justify-center">{value}</button
    >
</div>
