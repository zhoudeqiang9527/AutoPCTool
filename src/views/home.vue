<template>
  <div class="home">
    <a-space>
      <a-button
        type="primary"
        size="small"
        :icon="h(PlayCircleOutlined)"
        @click="handleScanLoop"
      >
        扫描
      </a-button>
      <a-button
        type="primary"
        danger
        size="small"
        :icon="h(PauseCircleOutlined)"
        @click="handleStop"
      >
        终止
      </a-button>
    </a-space>
    <a-divider>设置</a-divider>
    <a-input-number
      size="small"
      v-model:value="scanInterval"
      addon-before="扫描间隔(秒)"
      addon-after="毫秒"
    />
    <a-divider>任务</a-divider>
    <a-collapse
      v-if="startX.length"
      size="small"
      accordion
      v-model:activeKey="activeKey"
    >
      <a-collapse-panel
        :header="`任务${index} X: ${
          startX[index - 1] + endX[index - 1] / 2
        } Y: ${y[index - 1]}`"
        v-for="index in startX.length"
        :key="index"
      >
        <a-row :gutter="10">
          <!-- Radio buttons for type selection -->
          <a-col :span="4">
            <a-radio-group
              v-model:value="type[index - 1]"
              default-value="button"
              button-style="solid"
            >
              <a-radio value="button">按键</a-radio>
              <a-radio value="text">文本</a-radio>
            </a-radio-group>
          </a-col>

          <!-- Image and textarea -->
          <a-col :span="8">
            <ScreenImage :colors="colors[index - 1]" width="60%" />
            <a-textarea
            v-if="type[index - 1] === 'text'"
              v-model:value="textDetail[index - 1]"
              :rows="4"
              placeholder="输入文本内容"
            />
          </a-col>

          <!-- Input numbers for start X, end X, and Y -->
          <a-col :span="8">
            <a-space direction="vertical" align="start">
              <a-input-number
                size="small"
                v-model:value="startX[index - 1]"
                addon-before="起始X"
              />
              <a-input-number
                size="small"
                v-model:value="endX[index - 1]"
                addon-before="终止X"
              />
              <a-input-number
                size="small"
                v-model:value="y[index - 1]"
                addon-before="高度 Y"
              />
            </a-space>
          </a-col>
        </a-row>

        <template #extra>
          <a-space>
            <CreateScreenshot size="small" :index="index - 1" />
            <a-button
              size="small"
              :icon="h(CloseOutlined)"
              @click="deleteTask(index - 1)"
            ></a-button>
          </a-space>
        </template>
      </a-collapse-panel>
    </a-collapse>
    <a-space style="margin-top: 12px">
      <a-button size="small" @click="addProject">添加任务</a-button>
      <a-button size="small" type="primary" @click="reset">reset</a-button>
    </a-space>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useStorage } from "@vueuse/core";
import { ref, h, onMounted, onUnmounted } from "vue";
import {
  PlayCircleOutlined,
  PauseCircleOutlined,
  CloseOutlined,
} from "@ant-design/icons-vue";
import {
  isRegistered,
  register,
  unregister,
} from "@tauri-apps/plugin-global-shortcut";
import { getCurrentWindow } from "@tauri-apps/api/window";
import CreateScreenshot from "../components/createScreenshot.vue";
import ScreenImage from "../components/screenImage.vue";
const activeKey = ref("1");
const startX = useStorage<number[]>("startX", [0]);
const endX = useStorage<number[]>("endX", [0]);
const y = useStorage<number[]>("y", [0]);
const colors = useStorage<[][][]>("colors", [[]]);
const scanInterval = useStorage<number>("scanInterval", 1000);
const type = useStorage<string[]>(
  "type",
  new Array(startX.value.length).fill("button")
);
const textDetail = useStorage<string[]>("textDetail", [""]);

async function handleScanLoop() {
  startX.value.forEach((_item, index) => {
    setTimeout(() => {
      const data = {
        colors: colors.value[index],
        startX: startX.value[index],
        endX: endX.value[index],
        y: y.value[index],
        buttontype: type.value[index],
        textDetail: textDetail.value[index],
        interval: scanInterval.value * startX.value.length,
      };
      console.log("data-->", data);
      invoke("scan_loop", data);
    }, index * scanInterval.value);
  });
}

listen<{ startX: number; endX: number; y: number; index: number }>(
  "location",
  async (event) => {
    const index = event.payload.index;
    startX.value[index] = event.payload.startX;
    endX.value[index] = event.payload.endX;
    y.value[index] = event.payload.y;
    colors.value[index] = await invoke("scan_colors", {
      startX: event.payload.startX,
      endX: event.payload.endX,
      y: event.payload.y,
    });
  }
);

function addProject() {
  startX.value.push(0);
  endX.value.push(0);
  y.value.push(0);
  colors.value.push([]);
}

function handleStop() {
  invoke("stop_scan");
  console.log("stop");
}

onMounted(async () => {
  if (await isRegistered("Alt+Q")) {
    await unregister("Alt+Q");
  }
  if (await isRegistered("Alt+Z")) {
    await unregister("Alt+Z");
  }

  await register("Alt+Q", (event) => {
    if (event.state === "Pressed") {
      handleScanLoop();
      getCurrentWindow().hide();
    }
  });
  await register("Alt+Z", (event) => {
    if (event.state === "Pressed") {
      handleStop();
      getCurrentWindow().show();
    }
  });
});

onUnmounted(async () => {
  console.log("unmounted");
});

function deleteTask(index: number) {
  startX.value.splice(index, 1);
  endX.value.splice(index, 1);
  y.value.splice(index, 1);
  colors.value.splice(index, 1);
}

function reset() {
  localStorage.clear();
  window.location.reload();
}
</script>
<style scoped>
.home {
  min-height: 100vh;
  background-color: #fff;
  padding: 16px;
}
</style>
