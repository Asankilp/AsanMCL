import type { ProfileIcon } from '../types/profile';
import { VersionType } from '../types/version';

/**
 * 根据 ProfileIcon 获取对应的图片 URL。
 * @param icon ProfileIcon
 * @returns 图片 URL
 */
export function getProfileIconUrl(icon: ProfileIcon): string {
  if (typeof icon === 'string') {
    // 使用 import.meta.url 构造完整 URL
    return new URL(`../assets/images/icons/${icon}.png`, import.meta.url).toString();
  } else if ('custom' in icon) {
    // custom，base64 图片
    return `${icon.custom}`;
  }
  return '';
}

/**
 * 根据版本类型获取对应的图标 URL。
 * @param versionType 版本类型
 * @returns 图标 URL
 */
export function getVersionIcon(versionType: VersionType): string {
  switch (versionType) {
    case VersionType.Release:
      return getProfileIconUrl('grassblock');
    case VersionType.Snapshot:
      return getProfileIconUrl('dirt');
    case undefined:
    case null:
      return getProfileIconUrl('cobblestone');
    default:
      return getProfileIconUrl('cobblestone');
  }
}