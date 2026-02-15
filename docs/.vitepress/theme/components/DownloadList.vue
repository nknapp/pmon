<template>
  <div class="download-list">
    <div v-if="loading" class="download-card">Loading packages...</div>
    <div v-else-if="error" class="download-card download-error">
      <div class="download-title">Downloads not ready yet</div>
      <div class="download-meta">{{ error }}</div>
      <div class="download-meta">
        Once the CI build finishes, packages appear under
        <a href="/downloads/">/downloads/</a>.
      </div>
    </div>
    <div v-else class="download-grid">
      <a
        v-for="item in items"
        :key="item.name"
        class="download-card"
        :href="`/downloads/${item.name}`"
      >
        <div class="download-title">{{ item.label }}</div>
        <div class="download-meta">{{ item.name }}</div>
        <div class="download-kind">{{ item.kind.toUpperCase() }}</div>
      </a>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";

type DownloadItem = {
  name: string;
  label: string;
  kind: string;
};

const items = ref<DownloadItem[]>([]);
const loading = ref(true);
const error = ref("");

onMounted(async () => {
  try {
    const response = await fetch("/downloads/manifest.json", { cache: "no-store" });
    if (!response.ok) {
      throw new Error(`Manifest request failed (${response.status})`);
    }
    const data = (await response.json()) as { items?: DownloadItem[] };
    items.value = data.items ?? [];
    if (!items.value.length) {
      error.value = "No packages listed yet.";
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Unable to load downloads.";
  } finally {
    loading.value = false;
  }
});
</script>
