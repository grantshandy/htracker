<template>
    <div class="space-y-1">
        <div class="h-box">
            <h1 class="h-title">Tasks</h1>
            <div class="flow-root">
                <button class="float-left h-button" v-on:click="updateTasks">Refresh</button>
                <Transition name="loading">
                    <p v-if="loading" class="float-right text-sm">Syncing...</p>
                </Transition>
            </div>
            <div v-if="tasks && tasks.length > 0" class="h-inner-box divide-y-2 divide-base-00 dark:divide-base-0">
                <div v-bind:key="task" v-for="task in tasks">
                    <div class="flow-root p-3 bg-base-2 dark:bg-base-02 hover:bg-base-3 dark:hover:bg-base-03 rounded-md">
                        <p class="float-left select-none">{{ task.name }}</p>
                        <button class="float-right inline-flex items-center justify-center rounded-full select-none" v-on:click="removeTask(task)">
                            <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
            <div class="flex h-inner-box">
                <input v-model="currentTask" class="flex-1 h-text-input rounded-none border-none" v-on:keypress="addTask" placeholder="Add Task">
                <button class="h-button rounded-none rounded-r-sm" v-on:click="addTask">+</button>
            </div>
        </div>
        <ErrorBox :error="error" v-if="error" v-on:close-box="error = null"/>
    </div>
</template>

<script>
import ErrorBox from './ErrorBox.vue'

export default {
    name: 'TaskList',
    components: {
        ErrorBox
    },
    data() {
        return {
            accessToken: null,
            tasks: [],
            error: null,
            loading: false,
            currentTask: '',
        }
    },
    created() {
        this.accessToken = localStorage.getItem('accessToken');
    },
    async mounted() {
        await this.updateTasks();
    },
    methods: {
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
                    this.tasks = response;
                }
            })
            .catch(error => {
                this.error = error.message;
            });

            this.loading = false;
        },
        async addTask(event) {
            if ((event && event.key && event.key != 'Enter') || this.currentTask == '' || !this.currentTask) {
                return;
            }

            let currentTask = this.currentTask;
            this.currentTask = '';

            // insert temporary task to site state to appear more responsive
            this.tasks.push({ "name": currentTask });

            this.loading = true;

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
                    this.tasks = response;
                }
            })
            .catch(error => {
                this.error = error.message;
            });;

            this.loading = false;
        },
        async removeTask(task) {
            let taskIndex = this.tasks.indexOf(task);

            // remove task on client state before it's officially removed to make it appear more responsive.
            if (taskIndex > -1) {
                this.tasks.splice(taskIndex, 1);
            }

            this.loading = true;

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
                    this.tasks = response;
                }
            })
            .catch(error => {
                this.error = error.message;
            });;

            this.loading = false;
        },
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
</style>