const { createApp } = Vue

createApp({
    data() {
        return {
            secret: null,
        }
    },

    mounted() {
        console.log('running');
        this.secret = localStorage.getItem('secret');

        if (!this.secret) {
            window.location.href = '/login';
        } else {
            window.location.href = '/dashboard';
        }
    }
})
.mount('#app');