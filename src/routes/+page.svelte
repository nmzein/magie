<script lang="ts">
    import View from "$lib/components/View.svelte";
    import Controls from "$lib/components/Controls.svelte";
</script>

<View />
<Controls />

<style>
    :global(body) {
        margin: 0;
        background-color: green;
    }

    :global(button) {
        border: none;
        cursor: pointer;
    }
</style>