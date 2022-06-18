const { createApp } = Vue

createApp({
    data() {
        return {
            accessToken: null,
        }
    },

    mounted() {
        console.log('running');
        this.accessToken = localStorage.getItem('accessToken');

        if (this.accessToken) {
            window.location.href = '/dashboard';
        }
    }
})
.mount('#app');