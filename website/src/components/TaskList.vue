<template>
    <div class="space-y-1">
        <div class="h-box">
            <!-- if initialized -->
            <div v-if="init" class="space-y-5">
                <!-- title and refresh -->
                <div class="flex">
                    <h1 class="flex-1 h-title">Tasks</h1>
                    <div class="flex space-x-2 place-items-center">
                        <div v-if="loading" style="border-top-color:transparent" class="float-right h-7 w-7 border-4 border-magenta border-solid rounded-full animate-spin"></div>
                        <button class="h-button flex place-items-center space-x-2" v-on:click="updateTasks">Refresh</button>
                    </div>
                </div>
                <!-- add a task -->
                <div class="h-inner-box p-3 space-y-3 divide-dashed divide-y-2">
                    <!-- input section -->
                    <div class="space-y-3">
                        <!-- name and plus button -->
                        <div class="w-full flex h-inner-box">
                            <input v-model="currentName" class="h-text-input w-full rounded-none border-none rounded-l-sm" v-on:keypress="addTask" placeholder="Name">
                            <button class="h-button rounded-none rounded-r-sm" v-on:click="addTask">+</button>
                        </div>
                        <!-- add description box -->
                        <div v-if="showDescription" class="w-full flex h-inner-box">
                            <button v-on:click="showDescription = false" class="h-button rounded-none rounded-l-sm">Hide</button>
                            <input v-model="currentDescription" class="h-text-input w-full rounded-none border-none rounded-r-sm" v-on:keypress="addTask" placeholder=" Description">
                        </div>
                    </div>
                    <!-- add task feature buttons -->
                    <div v-if="!showDescription">
                        <div class="flex space-x-3 mt-3 place-items-center">
                            <button v-if="!showDescription" v-on:click="showDescription = true" class="h-button">Add Description</button>
                            <p>More Coming!</p>
                        </div>
                    </div>
                </div>
                <!-- task list -->
                <div v-if="tasks && tasks.length > 0" class="h-inner-box divide-y-2 divide-base-00 dark:divide-base-0">
                    <div v-bind:key="task" v-for="task in tasks">
                        <div class="flex place-items-center p-3 bg-base-2 dark:bg-base-02 hover:bg-base-3 dark:hover:bg-base-03 rounded-md">
                            <div class="flex-1 select-none">
                                <p class="font-semibold">{{ task.name }}</p>
                                <p v-if="task.description" class="text-sm">{{ task.description }}</p>
                            </div>
                            <button class="inline-flex items-center justify-center rounded-full select-none" v-on:click="removeTask(task)">
                                <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
            <!-- loading box -->
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
    name: 'TaskList',
    components: {
        ErrorBox
    },
    data() {
        return {
            init: false,
            sessionToken: null,
            tasks: [],
            error: null,
            loading: false,
            showRepeat: false,
            repeatEvery: 1,
            showDescription: false,
            currentName: '',
            currentDescription: '',
            otherOptions: false,
        }
    },
    async mounted() {
        await this.updateTasks();
        this.init = true;
    },
    methods: {
        async updateTasks() {
            this.loading = true;

            await fetch(window.location.origin + '/api/get_tasks', {
                method: 'GET',
                headers: {
                    'Accept': 'application/json',
                    'X-SessionToken': localStorage.getItem('sessionToken'),
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
            if ((event && event.key && event.key != 'Enter') || this.currentName == '' || !this.currentName) {
                return;
            }

            let currentName = this.currentName;
            let currentDescription = this.currentDescription;

            this.currentName = null;
            this.currentDescription = null;

            this.showDescription = false;

            // insert temporary task to site state to appear more responsive
            this.tasks.push({ "name": currentName, "description": currentDescription });

            this.loading = true;

            await fetch(window.location.origin + '/api/add_task', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'X-SessionToken': localStorage.getItem('sessionToken'),
                },
                body: JSON.stringify({
                    name: currentName,
                    description: currentDescription
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
            });

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
                    'X-SessionToken': localStorage.getItem('sessionToken'),
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