<template>
    <div>
        <div class="h-box">
            <div v-if="quote" class="space-y-3">
                <p class="font-semibold">"{{ quote.text }}"</p>
                <div class="flow-root">
                    <button v-on:click="updateQuote" class="float-left select-none text-sm">Refresh</button>
                    <p class="float-right text-sm">-{{ quote.author }}</p>
                </div>
            </div>
            <div v-else>
                <p class="text-center select-none">Loading Quote...</p>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: 'Quote',
    data() {
        return {
            quote: null,
        }
    },
    async mounted() {
        await this.updateQuote();
    },
    methods: {
        async updateQuote() {
            await fetch(window.location.origin + '/api/quote', {
                method: 'GET',
                headers: {
                    'Accept': 'application/json',
                    'X-AuthToken': localStorage.getItem('accessToken'),
                },
            })
            .then(response => response.json())
            .then(response => this.quote = response);
        }
    }
}
</script>