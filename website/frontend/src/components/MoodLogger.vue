<template>
    <div class="space-y-1">
        <div class="h-box">
            <div class="md:flow-root">
                <h1 class="md:float-left h-title">Log Mood</h1>
                <Transition name="loading">
                    <p v-if="loading" class="md:float-right text-sm">Sending...</p>
                </Transition>
            </div>
            <div class="h-inner-box grid grid-cols-5 grid-rows-1 text-center divide-x-4 divide-base-00 dark:divide-base-0">
                <button v-on:click="logMood(1)" class="mood-button">1</button>
                <button v-on:click="logMood(2)" class="mood-button">2</button>
                <button v-on:click="logMood(3)" class="mood-button">3</button>
                <button v-on:click="logMood(4)" class="mood-button">4</button>
                <button v-on:click="logMood(5)" class="mood-button">5</button>
            </div>
            <Transition name="loading">
                <p class="mx-2" v-if="sent">Sent!</p>
            </Transition>
        </div>
        <ErrorBox :error="error" v-if="error" v-on:close-box="error = null"/>
    </div>
</template>

<script>
import ErrorBox from './ErrorBox.vue'

export default {
    name: 'MoodLogger',
    components: {
        ErrorBox
    },
    data() {
        return {
            accessToken: null,
            error: null,
            loading: false,
            sent: false,
        }
    },
    created() {
        this.accessToken = localStorage.getItem('accessToken');
    },
    methods: {
        async logMood(mood) {
            this.loading = true;

            await fetch(window.location.origin + '/api/log_mood', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'X-AuthToken': this.accessToken,
                },
                body: JSON.stringify({
                    mood,
                }),
            })
            .then(response => response.json())
            .then(response => {
                if (response.error) {
                    this.error = response.error;
                } else {
                    this.tasks = response;
                }
            })
            .catch(error => {
                this.error = error.message;
            });;

            this.loading = false;

            this.sent = true;
            setTimeout(() => this.sent = false, 2000);
        }
    }
}
</script>

<style scoped>
    .loading-leave-active, .loading-enter-active {
        @apply duration-1000;
        @apply transition;
    }

    .loading-enter-from, .loading-leave-to {
        @apply opacity-0;
    }

    .mood-button {
        @apply h-button rounded-none border-none hover:border-none;
    }
</style>