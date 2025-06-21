import type { ProfileIcon } from '../types/profile';

/**
 * 根据 ProfileIcon 获取对应的图片 URL。
 * @param icon ProfileIcon
 * @returns 图片 URL
 */
export function getProfileIconUrl(icon: ProfileIcon): string {
  if (typeof icon === 'string') {
    // 非 custom，返回 public/assets 下同名 png，适配 Vite 静态资源
    return `src/assets/images/icons/${icon}.png`;
  } else if ('custom' in icon) {
    // custom，base64 图片
    return `${icon.custom}`;
  }
  return '';
}
