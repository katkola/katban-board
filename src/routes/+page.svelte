<script lang="ts">
  import BoardList from '../components/BoardList.svelte';
</script>

<main class="container">
  <BoardList />
</main>

<style>
  .container {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  }

  :global(body) {
    margin: 0;
    padding: 0;
  }
</style>
