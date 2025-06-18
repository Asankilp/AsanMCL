// import type { JreInfo } from '../types/jre'

// /**
//  * 对JRE列表按版本号进行降序排序
//  * @param jres JRE信息列表
//  * @returns 排序后的JRE列表
//  */
// export function sortJresByVersion(jres: JreInfo[]): JreInfo[] {
//   return jres.sort((a, b) => {
//     // 将版本号分割成数组，比如 "1.8.0" => ["1", "8", "0"]
//     const versionA = a.version.split('.').map(Number)
//     const versionB = b.version.split('.').map(Number)
    
//     // 从左到右比较每个版本号部分
//     for (let i = 0; i < Math.max(versionA.length, versionB.length); i++) {
//       const numA = versionA[i] || 0
//       const numB = versionB[i] || 0
//       if (numA !== numB) {
//         return numB - numA // 降序排列
//       }
//     }
//     return 0
//   })
// }
