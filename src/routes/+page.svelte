<script lang="ts">
    import { sum } from "$lib/nemato-rendering-engine/package/nemato_rendering_engine";

    function sumJS(array: Int32Array): number {
        let sum = 0;
        for (let i = 0; i < array.length; i++) {
            sum += array[i];
        }
        return sum;
    }

    let largeArray = new Int32Array(1e6).fill(1);

    console.time("JS");
    sumJS(largeArray);
    console.timeEnd("JS");

    console.time("Rust");
    sum(largeArray);
    console.timeEnd("Rust");
</script>

<h1>Welcome to Nemato!</h1>
