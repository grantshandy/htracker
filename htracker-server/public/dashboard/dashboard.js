const { createApp } = Vue

createApp({
    data() {
        return {
            secret: null,
        }
    },

    mounted() {
        this.secret = localStorage.getItem('secret');

        if (!this.secret) {
            window.location.href = '/login';
        }
    },

    methods: {
        logout() {
            localStorage.removeItem('secret');
            window.location.href = '/login';
        },
    }
})
.mount('#app');