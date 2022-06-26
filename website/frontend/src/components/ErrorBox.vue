<template>
    <div class="bg-red rounded-md text-base-2 font-bold p-3 flow-root">
        <p class="float-left">Error: {{ error }}</p>
        <button v-on:click="this.$emit('closeBox')" class="float-right inline-flex items-center justify-center bg-error hover:bg-error-dark rounded-full">
            <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
        </button>
    </div>
</template>

<script>
export default {
    name: 'ErrorBox',
    props: ['error'],
}
</script>