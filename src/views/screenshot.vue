<template>
  <div class="screen_wrap">
    <div :style="pickerStyle" class="screen-picker"></div>
    <a-button
      :style="pickerButtonStyle"
      class="picker-check"
      type="primary"
      @click="handleClick"
    >
      确定</a-button
    >
  </div>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useRoute } from "vue-router";
import { Webview } from '@tauri-apps/api/webview';
import { emit } from "@tauri-apps/api/event";

const index = Number(useRoute().query.index as string);
const width = ref(0);
const height = ref(0);
const top = ref(0);
const left = ref(0);

const pickerStyle = computed(() => {
  return {
    width: `${width.value}px`,
    height: `${height.value}px`,
    top: `${top.value}px`,
    left: `${left.value}px`,
  };
});

const pickerButtonStyle = computed(() => {
  return {
    top: `${top.value + height.value + 30}px`,
    left: `${left.value + width.value - 80}px`,
    visibility: isMouseDown.value ? "visible" : "hidden",
  };
});
// 鼠标左键是否按下
let isMouseDown = ref(false);

// 监听鼠标按下事件
window.addEventListener("mousedown", (e) => {
  // 判断当前按下的位置不是 picker-check 的位置
  // 获取 picker-check 的包围盒
  const pickerCheck = document.querySelector(".picker-check");
  const pickerCheckRect = pickerCheck?.getBoundingClientRect();

  if (
    pickerCheckRect &&
    e.clientX >= pickerCheckRect.left &&
    e.clientX <= pickerCheckRect.right &&
    e.clientY >= pickerCheckRect.top &&
    e.clientY <= pickerCheckRect.bottom
  ) {
    return;
  } else {
    isMouseDown.value = true;
    // 获取鼠标位置
    const { clientX, clientY } = e;
    left.value = clientX;
    top.value = clientY;
  }
});

// 监听鼠标移动事件
window.addEventListener("mousemove", (e) => {
  if (isMouseDown.value) {
    // 获取鼠标位置
    const { clientX, clientY } = e;
    // 计算宽高
    width.value = Math.abs(clientX - left.value);
    height.value = Math.abs(clientY - top.value);
    left.value = Math.min(clientX, left.value);
    top.value = Math.min(clientY, top.value);
  }
});

// 监听鼠标抬起事件
window.addEventListener("mouseup", () => {
  isMouseDown.value = false;
});

async function handleClick() {
  const curentwindow = getCurrentWindow();
  const allwindows = Webview.getAll();
  const mainWindow = (await allwindows).find(item=>item.label === "main");
  const scaleFactor = await curentwindow.scaleFactor();
  const position = await curentwindow.innerPosition();
  //窗口高度
  const payload = {
    startX: left.value,
    endX: left.value + width.value,
    y:top.value + height.value/2+Math.round(position.y/scaleFactor),
    index: index,
  }

  await emit("location", payload);
  if(mainWindow){
    await mainWindow.show();
  }
  await curentwindow.close();
}
</script>
<style  scoped>
.screen-wrap{
  width: 100vw;
  height: 100vh;
  position: absolute;
  top: 0;
  left: 0;
  z-index: 999;
  box-sizing: border-box;
  position: relative;
  .screen-picker{
    position: absolute;
    z-index: 999;
    border: solid 2px #0a7baf;
    box-sizing: content-box;
    transform: translate(-2px, -2px);
  }
  .picker-check{
    width: 80px;
    position: absolute;
    right: 0;
    bottom: -50px;
  }
}
</style>