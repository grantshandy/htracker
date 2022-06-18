const { createApp } = Vue

createApp({
    data() {
        return {
            secret: null,
        }
    },

    mounted() {
        this.secret = localStorage.getItem('accessToken');

        if (!this.secret) {
            window.location.href = '/login';
        }
    },

    methods: {
        logout() {
            localStorage.removeItem('accessToken');
            window.location.href = '/';
        },
    }
})
.mount('#app');