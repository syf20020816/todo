const toStore = (target: any): string => {
  return JSON.stringify(target)
}

const initLocalStorage = () => {
  // init local storage
  // - todo-sigin-in : false
  // - todo-user: {}
  window.localStorage.setItem('todo-sigin-in', toStore(false))
  window.localStorage.setItem('todo-user', toStore(new Object()))
}

export const init = () => {
  initLocalStorage()
}
