const { createApp } = Vue

createApp({
    data() {
        return {
            accessToken: null,
            darkMode: false,
        }
    },

    created() {
        this.accessToken = localStorage.getItem('accessToken');

        if (this.accessToken) {
            window.location.href = '/dashboard';
        }

        if (localStorage.getItem('darkMode') == 'true') {
            this.enableDarkMode();
        } else {
            this.disableDarkMode();
        }
    },

    methods: {
        enableDarkMode() {
            this.darkMode = true;
            localStorage.setItem('darkMode', 'true');
            DarkReader.enable({
                brightness: 100,
                contrast: 90,
                sepia: 10
            });
        },

        disableDarkMode() {   
            this.darkMode = false;
            localStorage.setItem('darkMode', 'false');         
            DarkReader.disable();
        },

        login() {
            window.location.href = '/login';
        },

        register() {
            window.location.href = '/register';
        }
    }
})
.mount('#app');