<script lang="ts">
    let show: boolean = true;
</script>

<nav>
    <div id="container">
        {#if show}
            <div class="panel large"></div>
        {:else}
            <div class="large"></div>
        {/if}
        <div class="panel small">
            <button id="show" on:click={() => show = !show}>Show</button>
        </div>
    </div>
</nav>

<style lang="scss">
    nav {
        display: block;
        position: absolute;
        
        top: 0;
        bottom: 0;
        right: 0;

        width: 350px;
        margin: 10px;
    }

    #container {
        display: flex;
        flex-direction: column;

        height: 100%;
        gap: 10px;
    }

    .panel {
        background-color: rgba(0, 0, 0, 0.75);
        border-radius: 10px;
    }
    
    .large {
        flex: 95;
    }

    .small {
        flex: 5;
    }

    button {
        height: 100%;

        &#show {
            border-radius: 10px 0 0 10px;
        }
    }
</style>