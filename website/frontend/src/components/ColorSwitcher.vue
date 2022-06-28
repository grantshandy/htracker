<template>
    <div>
        <button class="select-none" v-on:click="toggleDarkMode">
            Switch to {{ darkMode ? "Light" : "Dark" }} Mode
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
        toggleDarkMode() {
            if (this.darkMode) {
                this.disableDarkMode();
            } else {
                this.enableDarkMode();
            }
        },
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