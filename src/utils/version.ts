import { i18n } from "../main";

/**
 * 根据版本号获取更新主题。
 * @param versionId 版本号字符串
 * @returns 更新主题，未找到则返回空字符串
 */
export async function getMajorUpdateThemeById(versionId: string): Promise<string> {
  try {
    // 动态加载 major_updates.json（Vite 静态资源）
    const locale = i18n?.global?.locale ?? 'en';
    const url = new URL(`../assets/data/major_updates/${locale}.json`, import.meta.url).toString();
    const res = await fetch(url);
    if (!res.ok) return '';
    const data = await res.json();
    return data[versionId] || '';
  } catch (e) {
    return '';
  }
}
