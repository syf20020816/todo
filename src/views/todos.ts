import { ref } from 'vue'
import { Avatars, Todo, Priorities, Status } from '../core'

export const todos = ref<Todo[]>([
  {
    id: '001',
    name: 'Fix card display',
    priority: Priorities.Emergent,
    reviewers: [
      {
        username: '',
        name: '',
        avatar: Avatars.Avatar1,
        email: ' String',
        teamNumber: 0,
        todoNumber: 0,
        totalTodo: 0,
        todos: {},
        teams: null
      }
    ],
    performers: [
      {
        name: 'Jany',
        username: 'Jany001',
        todoNumber: 0,
        teamNumber: 0,
        email: 'Jany@gmail.com',
        avatar: Avatars.Avatar1,
        totalTodo: 0,
        todos: {},
        teams: null
      }
    ],
    date: {
      start: '10.30',
      end: '14.30',
      during: 2
    },
    tags: [
      {
        type: 'info',
        effect: 'dark',
        label: 'Rust'
      },
      {
        type: 'success',
        effect: 'dark',
        label: 'CSS'
      }
    ],
    status: Status.IN_PROGRESS,
    description: 'test'
  },
  {
    id: '002',
    name: 'Fix card display2',
    priority: Priorities.Emergent,
    reviewers: [
      {
        id: '1',
        name: 'Jany',
        email: 'Jany@gmail.com',
        avatar: Avatars.Avatar1
      }
    ],
    performers: [
      {
        id: '2',
        name: 'Surrog',
        email: 'Surrog@gmail.com',
        avatar: Avatars.Avatar2
      }
    ],
    date: {
      start: '10.30',
      end: '14.30',
      during: 2
    },
    tags: [
      {
        type: 'info',
        effect: 'dark',
        label: 'Rust'
      },
      {
        type: 'success',
        effect: 'dark',
        label: 'CSS'
      }
    ],
    status: Status.IN_PROGRESS,
    description: 'test'
  }
])
