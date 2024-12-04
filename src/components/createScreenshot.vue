<script setup lang="ts">
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ScanOutlined } from '@ant-design/icons-vue';
import { h } from 'vue';

const props = defineProps<{index: number}>();

async function handleScreenshot(){
    //创建截屏窗口
    const screenshot = new WebviewWindow('screenshot',{
        title: '截屏',
        decorations: false,
        url: `/#/screenshot?index=${props.index}`,
        alwaysOnTop: true,
        hiddenTitle: true,
        visible: true,
        resizable: true,
        skipTaskbar: true,
        transparent: true,
        maximized: true,
    });
    await getCurrentWindow().hide();
    await screenshot.show();
}
</script>

<template>
    <a-button @click="handleScreenshot" :icon="h(ScanOutlined)"></a-button>
</template>