<template>
    <div>
        <div class="h-box">
            <!-- quote -->
            <div v-if="quote" class="space-y-3">
                <p class="font-semibold">"{{ quote.text }}"</p>
                <div class="flow-root">
                    <button v-on:click="updateQuote" class="float-left select-none text-sm">Refresh</button>
                    <p class="float-right text-sm">-{{ quote.author }}</p>
                </div>
            </div>
            <!-- loading state -->
            <div v-else class="flex items-center justify-center">
                <Transition name="loading">
                    <div style="border-top-color:transparent" class="w-16 h-16 border-magenta border-solid border-8 rounded-full animate-spin"></div>
                </Transition>
            </div>
        </div>
        <ErrorBox :error="error" v-if="error" v-on:close-box="error = null"/>
    </div>
</template>

<script>
import ErrorBox from './ErrorBox.vue'

export default {
    name: 'Quote',
    components: {
        ErrorBox,
    },
    data() {
        return {
            quote: null,
            error: null,
        }
    },
    async mounted() {
        await this.updateQuote();
    },
    methods: {
        async updateQuote() {
            this.quote = null;

            await fetch(window.location.origin + '/api/quote', {
                method: 'GET',
                headers: {
                    'Accept': 'application/json',
                    'X-AuthToken': localStorage.getItem('accessToken'),
                },
            })
            .then(response => response.json())
            .then(response => this.quote = response)
            .catch(error => {
                this.error = error.message;
            });;
        }
    }
}
</script>