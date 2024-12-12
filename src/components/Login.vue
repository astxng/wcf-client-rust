<script lang="ts" setup>
import { ElMessage } from 'element-plus';
import { rhttp } from '@/utils/http/rhttp';
import { useConfigStore } from "@/store/modules/config";
import { ref } from "vue";
import { router } from "@/router";

const Login = async (data: any) => {
    return rhttp.post<{token: string}, any>('/api/auth/login', {data});
}

const configStore = useConfigStore();

const loginModel = ref({
    email: '',
    password: '',
    wx_id: '',
});

const submitForm = async () => {
    loginModel.value.wx_id = configStore.wechatConfig.wx_id;
    const res = await Login(loginModel.value);
    if (res && res.token) {
        configStore.wechatConfig.token = res.token;
        await configStore.update();
        ElMessage({
            message: `登录成功`,
            type: 'success',
        });
        router.replace("/");
    } else {
        ElMessage.error('登录错误')
    }
}
</script>

<template>
    <el-container>
        <el-main>
            <el-form ref="formRef" :model="loginModel" label-width="auto" class="demo-dynamic w-full">
                <el-card class="w-full mt-4">
                    <template #header>登录</template>
                    <el-form-item label="Email">
                        <el-input v-model="loginModel.email" />
                    </el-form-item>
                    <el-form-item label="密码">
                        <el-input type="password" v-model="loginModel.password" />
                    </el-form-item>
                    <el-button type="success" size="large" @click="submitForm()">登录</el-button>
                </el-card>
            </el-form>
        </el-main>
    </el-container>
</template>

<style lang="scss" scoped>
.el-container {
    padding: 0;
    height: calc(100vh - var(--header-height));

    >.el-main {
        padding: 10px;
    }
}

.grid-content {
    border-radius: 4px;
    min-height: 36px;
}
</style>