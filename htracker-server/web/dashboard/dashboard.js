const { createApp } = Vue

createApp({
    data() {
        return {
            accessToken: null,
            username: null,
            data: {
                tasks: null,
            },
            darkMode: false,
            error: null,
            currentTask: '',
            loading: false,
        }
    },

    created() {
        this.accessToken = localStorage.getItem('accessToken');

        if (!this.accessToken) {
            window.location.href = '/login';
        }

        // set username from access token
        this.username = atob(this.accessToken).split(':')[0];

        if (localStorage.getItem('darkMode') == 'true') {
            this.enableDarkMode();
        } else {
            this.disableDarkMode();
        }
    },

    async mounted() {
        await this.updateTasks();
    },

    methods: {
        // logout of current site instance
        logout() {
            localStorage.removeItem('accessToken');
            window.location.href = '/';
        },

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

        // update site's task state from server
        async updateTasks() {
            this.loading = true;

            await fetch(window.location.origin + '/api/get_tasks', {
                method: 'GET',
                headers: {
                    'Accept': 'application/json',
                    'X-AuthToken': this.accessToken,
                },
            })
            .then(response => response.json())
            .then(response => {
                if (response.error) {
                    this.error = response.error;
                } else {
                    this.data.tasks = response;
                }
            });

            this.loading = false;
        },

        // add a task to the server and site
        async addTask(event) {
            if ((event && event.key && event.key != 'Enter') || this.currentTask == '' || !this.currentTask) {
                return;
            }

            this.loading = true;

            let currentTask = this.currentTask;
            this.currentTask = '';

            // insert temporary task to site state to appear more responsive
            this.data.tasks.push({ "name": currentTask });

            await fetch(window.location.origin + '/api/add_task', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'X-AuthToken': this.accessToken,
                },
                body: JSON.stringify({
                    name: currentTask,
                }),
            })
            .then(response => response.json())
            .then(response => {
                if (response.error) {
                    this.error = response.error;
                } else {
                    this.data.tasks = response;
                }
            });

            this.loading = false;
        },

        // remove a task from the sever and site
        async removeTask(task) {
            this.loading = true;

            // remove task on client state before it's officially removed to make it appear more responsive.
            let taskIndex = this.data.tasks.indexOf(task);
            if (taskIndex > -1) {
                this.data.tasks.splice(taskIndex, 1);
            }

            await fetch(window.location.origin + '/api/remove_task', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'X-AuthToken': this.accessToken,
                },
                body: JSON.stringify({
                    id: task.id,
                }),
            })
            .then(response => response.json())
            .then(response => {
                if (response.error) {
                    this.error = response.error;
                } else {
                    this.data.tasks = response;
                }
            });

            this.loading = false;
        },
    }
})
.mount('#app');