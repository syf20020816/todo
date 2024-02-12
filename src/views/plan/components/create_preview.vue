<template>
  <div id="create_preview">
    <div class="create_wrapper">
      <div class="date_wrapper">
        <span>Today is {{ date.toLocaleDateString() }}</span>
        <span>{{ week[date.getDay()] }}</span>
      </div>
      <div class="info_wrapper">
        <div class="info_item" v-for="(item, index) in infoList" :key="index">
          <div class="title">{{ item.label }}</div>
          <div class="value">{{ item.value }}</div>
        </div>
      </div>
      <div class="work_wrapper">
        <el-button type="primary" @click="addTodoVisible = true">Add New TODO</el-button>
        <el-button type="danger">Remove All TODOs</el-button>
        <el-button type="success">Complete All TODOs</el-button>
      </div>
      <div class="todo_wrapper">
        <TODOItem :current-todo="currentTodo"></TODOItem>
      </div>
    </div>
    <div class="preview_wrapper">
      <div class="todo-item" v-for="(item, index) in datas" :key="index">
        <div class="left">
          <span class="dot_wrapper">
            <span class="priority" :style="getPriorityDot(item)"></span>
            <span
              class="priority"
              style="border-radius: 50%"
              :style="getStatusDot(item)"
            ></span>
          </span>
          <div class="todo-name">{{ item.name }}</div>
        </div>
        <div class="right">
          <el-icon size="18px" class="icons" @click="showTodoDetails(item)"
            ><QuestionFilled
          /></el-icon>
          <el-icon size="18px" class="icons"><CircleClose /></el-icon>
        </div>
      </div>
    </div>
  </div>
  <el-dialog v-model="addTodoVisible" title="Add New Todo" width="680">
    <div>
      <el-form
        label-position="left"
        label-width="120px"
        :model="todoForm"
        style="max-width: 600px"
      >
        <el-form-item label="Todo Name">
          <el-input v-model="todoForm.name" />
        </el-form-item>
        <el-form-item label="Priority">
          <el-select
            v-model="todoForm.priority"
            placeholder="Select Priority"
            style="width: 240px"
          >
            <el-option
              v-for="item in priorityOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="Tags">
          <el-popover
            placement="bottom"
            title="Add Tag"
            :width="360"
            trigger="click"
            :visible="addTagVisible"
          >
            <div class="tag-popover-wrapper">
              <el-input
                v-model="todoTag.label"
                placeholder="input tag name"
                clearable
              ></el-input>
              <el-select v-model="todoTag.effect" placeholder="choose effect">
                <el-option
                  v-for="item in effectOptions"
                  :label="item.label"
                  :value="item.value"
                  :key="item.value"
                >
                </el-option>
              </el-select>
              <el-select v-model="todoTag.type" placeholder="choose type">
                <el-option
                  v-for="item in typeOptions"
                  :label="item.label"
                  :value="item.value"
                  :key="item.value"
                >
                </el-option>
              </el-select>
              <el-button type="primary" @click="addTag">Add</el-button>
            </div>
            <template #reference>
              <el-button
                type="primary"
                :icon="CirclePlusFilled"
                @click="addTagVisible = !addTagVisible"
              >
              </el-button>
            </template>
          </el-popover>
          <div class="tags-wrapper">
            <el-tag
              :effect="item.effect"
              round
              closable
              @close="removeTag(item)"
              v-for="item in todoForm.tags"
              :key="item"
              :type="item.type"
              >{{ item.label }}</el-tag
            >
          </div>
        </el-form-item>
        <el-form-item label="Description">
          <el-input
            v-model="todoForm.description"
            type="text"
            maxlength="30"
            show-word-limit
            placeholder="Please input"
          />
        </el-form-item>
        <el-form-item label="Information">
          <el-input
            v-model="todoForm.information"
            :rows="2"
            type="textarea"
            placeholder="Please input"
          />
        </el-form-item>
        <el-form-item label="Date">
          <el-date-picker
            v-model="dateRange"
            type="datetimerange"
            :shortcuts="shortcuts"
            range-separator="To"
            start-placeholder="Start date"
            end-placeholder="End date"
          />
        </el-form-item>
        <el-form-item label="Focus?">
          <el-switch v-model="todoForm.isFocus" />
        </el-form-item>
        <el-form-item label="Annex?">
          <el-upload
            style="width: 100%"
            ref="uploadRef"
            :auto-upload="false"
            v-model:file-list="fileList"
            class="upload-demo"
            :multiple="true"
            @change="uploadAndConvertBase64"
            action="https://127.0.0.1:10016/api/v1/file"
          >
            <el-button type="primary" :icon="CirclePlusFilled"
              >Click to choose file</el-button
            >
            <template #tip>
              <div class="el-upload__tip">file size 10MB</div>
            </template>
          </el-upload>
        </el-form-item>
      </el-form>
    </div>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="addTodoVisible = false">Cancel</el-button>
        <el-button type="primary" @click="addNewTodo"> Add </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script lang="ts" setup>
import { ref, reactive, computed, toRaw } from 'vue'
import { TODOItem } from '../index'
import { CircleClose, QuestionFilled, CirclePlusFilled } from '@element-plus/icons-vue'
import { Todo, Priorities, Avatars, Status, usePriorityColor, useAvatar, useStatus, convertFileToBase64,ITagProps, priorityOptions, effectOptions, typeOptions } from '../../../core'
import { type UploadProps, type UploadUserFile, type UploadInstance, ElMessage,type UploadFile , type UploadFiles } from 'element-plus'
import api from '../../../api'
import {user as userPinia} from "../../../store/src/user"

const props = defineProps<{
  datas: Todo[]
}>()
const userStore = userPinia()
const date = new Date()
const addTodoVisible = ref(false)
const addTagVisible = ref(false)
const week = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']
const uploadRef = ref<UploadInstance>()
const dateRange = ref<[Date, Date]>([new Date(), new Date()])
const todoTag = ref<ITagProps>({
  type: '',
  effect: 'dark',
  label: ''
})
const infoList = reactive([
  {
    label: 'TODOs for today',
    value: 3
  },
  {
    label: 'Emergent TODOs',
    value: 1
  },
  {
    label: 'Normal TODOs',
    value: 2
  },
  {
    label: 'Start ~ End',
    value: '10.30 ~ 16.00'
  }
])
const fileList = ref<UploadUserFile[]>([])
const fileBase64List = ref<string[]>([])
const currentTodo = ref<any>()
const todoForm = reactive<Todo>({} as Todo)
const getPriorityDot = computed(() => (item: Todo) => {
  let { priority } = item || Priorities.Low
  return `background-color : ${usePriorityColor(priority)}`
})

const getStatusDot = computed(() => (item: Todo) => {
  let { status } = item || Status.NOT_START
  return `background-color : ${useStatus(status)}`
})

const showTodoDetails = (item: Todo) => {
  currentTodo.value = item
}

const shortcuts = [
  {
    text: 'Next day',
    value: () => {
      const end = new Date()
      const start = new Date()
      end.setTime(start.getTime() + 3600 * 1000 * 24 * 1)
      return [start, end]
    }
  },
  {
    text: 'Next week',
    value: () => {
      const end = new Date()
      const start = new Date()
      end.setTime(start.getTime() + 3600 * 1000 * 24 * 7)
      return [start, end]
    }
  },
  {
    text: 'Next month',
    value: () => {
      const end = new Date()
      const start = new Date()
      end.setTime(start.getTime() + 3600 * 1000 * 24 * 30)
      return [start, end]
    }
  }
]

const addNewTodo = async () => {
  let {username} = userStore.user

  let todo = todoForm;
  console.log(todo)

  // const data = await api.todo.addNewTodo(username , );
}

const uploadAndConvertBase64 = (uploadFile: UploadFile, _uploadFiles: UploadFiles) =>{
  let file = uploadFile.raw;
  if(typeof file !=='undefined'){
    convertFileToBase64(file).then(base64=>{
      fileBase64List.value.push(base64)
    })
  }
}

const submitUpload = () => {
  // uploadRef.value!.submit();
}

const addTag = () => {
  let { tags } = todoForm

  if (typeof tags === 'undefined') {
    tags = new Array()
  }
  if (todoTag.value.label === '') {
    ElMessage({
      type: 'warning',
      message: 'you should add tag name'
    })
    return
  }
  for(let tag of tags){
    let tagStr = JSON.stringify(toRaw(tag))
    let newTagStr = JSON.stringify(toRaw(todoTag.value))
    if(tagStr===newTagStr){
      ElMessage({
      type: 'warning',
      message: 'you already have the same tag'
    })
    return
    }else{
      continue
    }

  }

  tags.push(toRaw(todoTag.value))
  todoForm.tags = tags
  todoTag.value = {
    type: '',
    effect: 'dark',
    label: ''
  } as ITagProps
}

const removeTag = (tag: ITagProps) => {
  todoForm.tags = todoForm.tags.filter(item => item !== tag)
}
</script>

<style lang="scss" scoped>
@use '../../../styles/src/var.scss' as *;
#create_preview {
  height: 100%;
  width: 100%;
  box-sizing: border-box;
  padding: 8px;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  overflow-y: scroll;
  scrollbar-width: thin;
  .create_wrapper {
    width: 65%;
    .date_wrapper {
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: flex-start;
      span {
        margin: 0 6px;
        font-weight: 700;
      }
    }
    .info_wrapper {
      box-sizing: border-box;
      padding: 6px;
      border-top: 1px solid $bg-color-hover;
      width: 100%;
      height: fit-content;
      border-bottom: 1px solid $bg-color-hover;
      .info_item {
        font-size: 14px;
        height: 2em;
        display: flex;
        align-items: center;
        justify-content: space-between;
        box-sizing: border-box;
        padding: 0 6px;
        transition: all 0.25s ease-in-out;
        cursor: pointer;
        &:hover {
          background-color: $bg-color-hover;
          color: $force-color;
        }
      }
    }
    .work_wrapper {
      box-sizing: border-box;
      padding: 6px;
      display: flex;
      align-items: center;
      justify-content: flex-end;
    }
    .todo_wrapper {
      height: 460px;
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: center;
      box-sizing: border-box;
      padding: 12px;
    }
  }
  .preview_wrapper {
    height: 100%;
    width: 35%;
    box-sizing: border-box;
    border-left: 1px dashed $bg-color-hover;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    flex-wrap: wrap;
    align-content: flex-start;
    .todo-item {
      margin-bottom: 6px;
      height: 60px;
      width: 96%;
      background-color: $bg-color-hover;
      border-radius: 4px;
      display: flex;
      align-items: center;
      justify-content: flex-start;
      box-sizing: border-box;
      padding: 12px;
      .left {
        display: flex;
        align-items: center;
        justify-content: flex-start;
        width: calc(100% - 48px);
        .dot_wrapper {
          display: inline-block;
          height: 1.5em;
          display: flex;
          align-items: center;
          justify-content: center;
          margin-right: 12px;
          .priority {
            display: inline-block;
            height: 12px;
            width: 12px;
            border-radius: 2px;
            margin: 0 4px;
          }
        }
        .todo-name {
          font-size: 18px;
          width: calc(100% - 24px);
          font-weight: 700;
          text-align: left;
        }
      }
      .right {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 48px;
        .icons {
          cursor: pointer;
        }
      }
    }
  }
}
.tag-popover-wrapper {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  .el-select {
    width: 100%;
    margin: 6px 0;
  }
}
.tags-wrapper {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  flex-wrap: wrap;
  align-content: flex-start;
  .el-tag {
    margin: 6px;
  }
}
</style>
