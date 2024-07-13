<template>
  <div class="container mx-auto my-8 px-4">
    <h1 class="text-2xl font-bold mb-4">任务管理器</h1>

    <div class="bg-white shadow-md rounded-lg p-6 mb-6">
      <h2 class="text-xl font-bold mb-4">添加任务</h2>

      <div class="grid grid-cols-2 gap-4">
        <div>
          <label for="source-dir" class="block font-medium mb-2">源目录</label>
          <div class="flex">
            <input
              id="source-dir"
              v-model="sourceDir"
              type="text"
              class="flex-1 px-3 py-2 border rounded-l-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="请选择源目录"
            />
            <button
              class="px-3 py-2 bg-blue-500 text-white rounded-r-md hover:bg-blue-600"
              @click="browerDirectory('sourceDir')"
            >
              浏览
            </button>
            <button
              class="px-3 py-2 bg-green-500 text-white rounded hover:bg-green-600 ml-2"
              @click="openDirectory('sourceDir')"
            >
              打开
            </button>
          </div>
        </div>
        <div>
          <label for="target-dir" class="block font-medium mb-2"
            >目标目录</label
          >
          <div class="flex">
            <input
              id="target-dir"
              v-model="targetDir"
              type="text"
              class="flex-1 px-3 py-2 border rounded-l-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="请选择目标目录"
            />
            <button
              class="px-3 py-2 bg-blue-500 text-white rounded-r-md hover:bg-blue-600"
              @click="browerDirectory('targetDir')"
            >
              浏览
            </button>
            <button
              class="px-3 py-2 bg-green-500 text-white rounded hover:bg-green-600 ml-2"
              @click="openDirectory('targetDir')"
            >
              打开
            </button>
          </div>
        </div>
      </div>

      <div class="mt-4">
        <label for="schedule" class="block font-medium mb-2">执行频率</label>
        <select
          id="schedule"
          v-model="schedule"
          class="px-3 py-2 border rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="daily">每天</option>
          <option value="interval">每隔几分钟/几小时</option>
        </select>
        <div v-if="schedule === 'daily'" class="mt-2">
          <label for="daily-time" class="block font-medium mb-2"
            >每天执行时间</label
          >
          <input
            id="daily-time"
            v-model="dailyTime"
            type="time"
            class="px-3 py-2 border rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div v-if="schedule === 'interval'" class="mt-2">
          <label for="interval" class="block font-medium mb-2">执行间隔</label>
          <input
            id="interval"
            v-model="interval"
            type="number"
            class="px-3 py-2 border rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="请输入间隔时长"
          />
          <select
            v-model="intervalUnit"
            class="px-3 py-2 border rounded focus:outline-none focus:ring-2 focus:ring-blue-500 mt-2"
          >
            <option value="分钟">分钟</option>
            <option value="小时">小时</option>
          </select>
        </div>
      </div>

      <div class="mt-4 text-right">
        <input
          id="task-name"
          v-model="taskName"
          type="text"
          class="flex-1 px-3 py-2 border rounded-l-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="请输入任务名称"
        />
        <button
          class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          @click="addTask"
        >
          添加任务
        </button>
      </div>
    </div>

    <div class="bg-white shadow-md rounded-lg p-6">
      <h2 class="text-xl font-bold mb-4">任务列表</h2>
      <table class="w-full table-auto">
        <thead>
          <tr class="bg-gray-200">
            <th class="px-4 py-2 text-left">任务</th>
            <th class="px-4 py-2 text-left">源目录</th>
            <th class="px-4 py-2 text-left">目标目录</th>
            <th class="px-4 py-2 text-left">执行频率</th>
            <th class="px-4 py-2 text-left">下次执行时间</th>
            <th class="px-4 py-2 text-left">任务状态</th>
            <th class="px-4 py-2 text-left">操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="task in tasks" :key="task.id">
            <td class="px-4 py-2">{{ task.name }}</td>
            <td class="px-4 py-2">{{ task.sourceDir }}</td>
            <td class="px-4 py-2">{{ task.targetDir }}</td>
            <td class="px-4 py-2">{{ getScheduleLabel(task) }}</td>
            <td class="px-4 py-2">
              {{ getDatetime(task.nextExecutionTimestamp) }}
            </td>
            <td class="px-4 py-2">{{ task.status }}</td>
            <td class="px-4 py-2">
              <button
                class="px-3 py-1 bg-yellow-500 text-white rounded hover:bg-yellow-600 mr-2"
                @click="pauseTask(task.id)"
              >
                暂停
              </button>
              <button
                class="px-3 py-1 bg-green-500 text-white rounded hover:bg-green-600 mr-2"
                @click="openTask(task.id)"
              >
                开启
              </button>
              <button
                class="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 mr-2"
                @click="triggerTask(task.id)"
              >
                立即触发
              </button>
              <button
                class="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600 mr-2"
                @click="deleteTask(task.id)"
              >
                删除
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { attachConsole } from "@tauri-apps/plugin-log";
import { isEnabled } from "@tauri-apps/plugin-autostart";

// 定义 Task 对象类型
type Task = {
  id: number;
  name: string;
  sourceDir: string;
  targetDir: string;
  schedule: string;
  nextExecutionTimestamp: number;
  cron: {
    weekday: number; // 0-6 (0 is Sunday) 99 is every day
    hour: number; // 0-23 99 is every hour
    minute: number; // 0-59 99 is every minute
    timer: number; // 每隔多少分钟
    timerType: number; // 0 分钟 1 小时
  };
  status: string;
};

// 启用 TargetKind::Webview 后，这个函数将把日志打印到浏览器控制台
const detach = await attachConsole();

export default defineComponent({
  name: "TaskManager",
  setup() {
    const targetDir = ref<string>("");
    const sourceDir = ref<string>("");
    const taskName = ref<string>("");
    const schedule = ref<string>("daily");
    const dailyTime = ref("08:00");
    const weeklyDay = ref("星期一");
    const weeklyTime = ref("08:00");
    const interval = ref(15);
    const intervalUnit = ref("分钟");
    const tasks = ref<Task[]>([]);

    const weekdays = [
      "星期一",
      "星期二",
      "星期三",
      "星期四",
      "星期五",
      "星期六",
      "星期日",
    ];

    function getScheduleLabel(task: Task) {
      if (task.schedule === "daily") {
        return `每天 ${task.cron.hour.toString().padStart(2, "0")}:${task.cron.minute.toString().padStart(2, "0")}`;
      } else if (task.schedule === "weekly") {
        return `每周 ${weekdays[task.cron.weekday]} ${task.cron.hour.toString().padStart(2, "0")}:${task.cron.minute.toString().padStart(2, "0")}`;
      } else if (task.schedule === "interval") {
        return `每隔 ${task.cron.timer} ${task.cron.timerType === 0 ? "分钟" : "小时"}`;
      } else {
        return "未知";
      }
    }

    const loadDirs = () => {
      const target = localStorage.getItem("targetDir");
      if (target) targetDir.value = target;
      const source = localStorage.getItem("sourceDir");
      if (source) sourceDir.value = source;
    };

    const openDirectory = (key: "targetDir" | "sourceDir") => {
      const res = key === "targetDir" ? targetDir.value : sourceDir.value;
      show_in_folder(res);
    };

    async function show_in_folder(path: string) {
      await invoke("show_in_folder", { path });
    }

    const saveDir = (key: "targetDir" | "sourceDir") => {
      localStorage.setItem(
        key,
        key === "targetDir" ? targetDir.value : sourceDir.value
      );
    };

    const browerDirectory = (type: "sourceDir" | "targetDir") => {
      open({ directory: true }).then((res: string | null) => {
        if (res !== null) {
          if (type === "sourceDir") sourceDir.value = res;
          else targetDir.value = res;
          saveDir(type);
        }
      });
    };

    const addTask = () => {
      const newTask = {
        id: tasks.value.length + 1,
        name: taskName.value,
        sourceDir: sourceDir.value,
        targetDir: targetDir.value,
        schedule: schedule.value,
        nextExecutionTimestamp: 0,
        cron: {
          weekday: 0,
          hour: 0,
          minute: 0,
          timer: 0,
          timerType: 0,
        },
        status: "正在运行",
      };
      newTask.cron = genCron(newTask);
      newTask.nextExecutionTimestamp = getNextExecutionTime(newTask);
      if (checkTask(newTask)) {
        tasks.value.push(newTask);
        // 保存任务数据
        saveTaskData();
      } else {
        alert("请填写完整任务信息");
      }
    };

    const genCron = (task: Task) => {
      if (task.schedule === "daily") {
        return {
          weekday: 99,
          hour: parseInt(dailyTime.value.split(":")[0]),
          minute: parseInt(dailyTime.value.split(":")[1]),
          timer: 0,
          timerType: 0,
        };
      } else if (task.schedule === "weekly") {
        return {
          weekday: weekdays.indexOf(weeklyDay.value),
          hour: parseInt(weeklyTime.value.split(":")[0]),
          minute: parseInt(weeklyTime.value.split(":")[1]),
          timer: 0,
          timerType: 0,
        };
      } else if (task.schedule === "interval") {
        return {
          weekday: 99,
          hour: 99,
          minute: 0,
          timer: interval.value,
          timerType: intervalUnit.value === "分钟" ? 0 : 1,
        };
      } else {
        console.log("未知的任务类型");
        return {
          weekday: 99,
          hour: 99,
          minute: 0,
          timer: 0,
          timerType: 0,
        };
      }
    };

    const checkTask = (task: Task) => {
      if (task.name === "" || task.sourceDir === "" || task.targetDir === "") {
        return false;
      }
      return true;
    };

    const pauseTask = (id: number) => {
      const task = tasks.value.find((t: Task) => t.id === id);
      if (task) {
        task.status = "已暂停";
        // 保存任务数据
        saveTaskData();
      }
    };

    const runTask = (task: Task) => {
      // 保存任务数据
      invoke("copy_dir", {
        sourceDir: task.sourceDir,
        targetDir: task.targetDir,
      })
        .then((message: string) => {
          task.status = "成功";
          console.log(message);
        })
        .catch((error: string) => {
          task.status = "失败";
          console.error(error);
        });
    };

    const triggerTask = (id: number) => {
      const task = tasks.value.find((t: Task) => t.id === id);
      if (task) {
        // 执行任务
        console.log(`手动执行任务 ${task.name}`);
        // 保存任务数据
        runTask(task);
      }
    };

    const autuRunTasks = () => {
      if (tasks == null) {
        return;
      }
      if (tasks.value == null) {
        return;
      }
      if (tasks.value.length === 0) {
        return;
      }
      tasks.value.forEach((task: Task) => {
        // 执行任务
        if (task.status === "已暂停") {
          console.log(`执行任务 ${task.name} 暂停, 无需执行`);
          return;
        }

        if (canRunTask(task)) {
          runTask(task);
          task.nextExecutionTimestamp = getNextExecutionTime(task);
          // 执行任务
          console.log(
            `执行任务 ${task.name} 成功--------------------------------------------`
          );
        } else {
          console.log(`执行任务 ${task.name} 未到执行时间`);
        }
      });
      // 保存任务数据
      saveTaskData();
    };

    const canRunTask = (task: Task) => {
      const now = new Date();
      if (task.nextExecutionTimestamp == 0) {
        task.nextExecutionTimestamp = getNextExecutionTime(task);
      }
      const nextExecutionTime = new Date(task.nextExecutionTimestamp);
      if (now >= nextExecutionTime) {
        return true;
      }
      return false;
    };

    const getNextExecutionTime = (task: Task) => {
      const now = new Date();
      let next = new Date();
      if (task.schedule === "daily") {
        // 每小时
        if (task.cron.hour === 99) {
          next = new Date(
            now.getFullYear(),
            now.getMonth(),
            now.getDate(),
            now.getHours() + 1,
            task.cron.minute
          );
        } else {
          next = new Date(
            now.getFullYear(),
            now.getMonth(),
            now.getDate() + 1,
            task.cron.hour,
            task.cron.minute
          );
        }
      } else if (task.schedule === "weekly") {
        if (task.cron.weekday === 99) {
          next = new Date(
            now.getFullYear(),
            now.getMonth(),
            now.getDate() + 1,
            task.cron.hour,
            task.cron.minute
          );
          return next.getTime();
        }
        next = new Date(
          now.getFullYear(),
          now.getMonth(),
          now.getDate(),
          task.cron.hour,
          task.cron.minute
        );
        const weekday = task.cron.weekday;
        let diff = weekday - now.getDay();
        if (diff < 0) {
          diff += 7;
        }
        next.setDate(next.getDate() + diff);
        if (next < now) {
          next.setDate(next.getDate() + 7);
        }
        return next.getTime();
      } else if (task.schedule === "interval") {
        if (task.cron.timerType === 0) {
          next = new Date(now.getTime() + task.cron.timer * 60 * 1000);
        } else if (task.cron.timerType === 1) {
          next = new Date(now.getTime() + task.cron.timer * 60 * 60 * 1000);
        }
        return next.getTime();
      } else {
        console.log("未知的任务类型");
        task.status = "已暂停";
        return now.getTime();
      }
      return next.getTime();
    };

    const saveTaskData = () => {
      // 将任务数据保存到本地存储或其他持久化方式
      const data = JSON.stringify(tasks.value);
      invoke("save_json_string", {
        jsonData: data,
        filePath: "tasks.json",
      })
        .then((dd: string) => {
          console.log(dd);
        })
        .catch((error: string) => {
          console.error(error);
        });
    };

    const loadTasks = () => {
      invoke("load_json_string", { filePath: "tasks.json" })
        .then((dd: string) => {
          console.log(dd);
          tasks.value = JSON.parse(dd);
        })
        .catch((error: string) => {
          console.error(error);
        });
      tasks.value.forEach((task: Task) => {
        if (task.nextExecutionTimestamp === 0) {
          task.nextExecutionTimestamp = getNextExecutionTime(task);
        }
      });
    };

    const deleteTask = (id: number) => {
      const index = tasks.value.findIndex((t: Task) => t.id === id);
      if (index > -1) {
        tasks.value.splice(index, 1);
        // 保存任务数据
        saveTaskData();
      }
    };

    const openTask = (id: number) => {
      const task = tasks.value.find((t: Task) => t.id === id);
      if (task) {
        task.status = "正在运行";
        // 保存任务数据
        saveTaskData();
      }
    };

    const getDatetime = (timeStamp: number) => {
      const date = new Date(timeStamp);
      return date.toLocaleString();
    };

    async function isEnabledLog() {
      // 检查 enable 状态
      console.log(`registered for autostart? ${await isEnabled()}`);
    }

    onMounted(() => {
      loadDirs();
      loadTasks();
      isEnabledLog();
    });

    // 定期检查有没有要执行的任务
    setInterval(autuRunTasks, 5000);

    // 将浏览器控制台与日志流分离

    return {
      deleteTask,
      sourceDir,
      targetDir,
      taskName,
      schedule,
      dailyTime,
      weeklyDay,
      weeklyTime,
      interval,
      intervalUnit,
      tasks,
      weekdays,
      getScheduleLabel,
      openDirectory,
      addTask,
      pauseTask,
      triggerTask,
      browerDirectory,
      openTask,
      getDatetime,
    };
  },
});
detach();
</script>
