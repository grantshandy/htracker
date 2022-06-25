<template>
  <div class="h-screen text-base-00 bg-base-3 dark:text-base-0 dark:bg-base-03">
    <div class="content-root">
      <div class="flow-root">
        <div class="float-right">
          <ColorSwitcher />
        </div>
      </div>
      <div class="content-box">
        <h1 class="content-title">Login</h1>
        <div class="flow-root text-base-01 dark:text-base-1">
          <button v-on:click="login" class="float-left">Login</button>
          <a class="float-right" href="/register">Register Instead</a>
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
  },
  methods: {
    login() {
      console.log('login');
    }
  }
}
</script>
