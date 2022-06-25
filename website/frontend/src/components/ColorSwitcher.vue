<template>
    <div class="text-sm">
        <button v-if="darkMode" v-on:click="disableDarkMode">
            Switch to Light Mode
        </button>
        <button v-if="!darkMode" v-on:click="enableDarkMode">
            Switch to Dark Mode
        </button>
    </div>
</template>

<script>
export default {
    name: 'ColorSwitcher',

    data() {
        return {
            darkMode: false,
        }
    },

    created() {
        if (localStorage.getItem('theme') == 'dark' || !localStorage.getItem('theme')) {
            this.enableDarkMode();
        } else {
            this.disableDarkMode();
        }
    },

    methods: {
        enableDarkMode() {
            localStorage.setItem('theme', 'dark');
            document.documentElement.classList.add('dark');
            this.darkMode = true;
        },

        disableDarkMode() {
            localStorage.setItem('theme', 'light');
            document.documentElement.classList.remove('dark');
            this.darkMode = false;
        },
    }
}
</script>