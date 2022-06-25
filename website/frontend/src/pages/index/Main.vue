<template>
  <div class="h-screen text-base-00 bg-base-3 dark:text-base-0 dark:bg-base-03">
    <div class="content-root">
      <div class="flow-root">
        <div class="float-right">
          <ColorSwitcher />
        </div>
      </div>
      <div class="content-box text-center">
        <div>
          <h1 class="content-title">htracker</h1>
          <p>A minimalist habit tracker, to-do list, and mood log. (WIP)</p>
        </div>
        <div class="flow-root text-base-01 dark:text-base-1">
          <a class="float-left" href="/login">Login</a>
          <a class="float-right" href="/register">Register</a>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import ColorSwitcher from '../../components/ColorSwitcher.vue'

export default {
  name: 'App',
  components: {
    ColorSwitcher,
  }
}
</script>
